#!/usr/bin/env python3
"""Generate a throwaway Cargo workspace for the ADR 0027 compile-cost study.

Kinds:
  mixed  N types as N/16 copies of the 16-type PostgreSQL prototype
         (`tk.rs` with typekin, `hand.rs` by hand plus `bitflags`). Members:
         `tk_types`, `hand_types`, and the downstream binaries `tk_use` and
         `hand_use`, which call every copy and print one checksum.
  flags  N copies of the `t_infomask` bitflag set alone. Members:
         `tk_types`, `hand_types`.

Every copy is its own module file (`m00.rs`, ...). Typekin derives exported
literal-macro names from the source file, so copies must not share a file.

Usage: gen.py OUT --kind mixed --types 256
"""

import argparse
import json
import pathlib
import shutil

HERE = pathlib.Path(__file__).resolve().parent
REPO = HERE.parents[1]
BITFLAGS = "=2.13.2"
TYPES_PER_COPY = 16
# The one-line edit the incremental scenario toggles; present in both
# templates (`Selectivity::clamped`).
EDIT_FROM = "raw.clamp(0.0, 1.0)"
EDIT_TO = "raw.clamp(0.0, 1.00)"

EXERCISE = """
#[inline(never)]
pub fn ex(s: u64) -> u64 {
    use CRATE::MODULE::*;
    let off = OffsetNumber::try_make((s % 2000) as u16 + 1).unwrap() + 1u16;
    let blk = BlockNumber::try_make((s % 100_000) as u32).unwrap();
    let a = NormalXid::try_make((s as u32) | 3).unwrap();
    let b = a.advance();
    let start = Lsn::try_make(s | 1).unwrap();
    let end = start + WalBytes::make(s % 4096 + 1);
    let item = ItemId::make(ItemIdValues {
        off: LpOff::try_make((s & 0x7fff) as u16).unwrap(),
        flags: NORMAL,
        len: LpLen::try_make(((s >> 15) & 0x7fff) as u16).unwrap(),
    });
    let info = IndexTupleInfo::make(IndexTupleInfoValues {
        size: IndexTupleSize::try_make((s & 0x1fff) as u16).unwrap(),
        am_reserved: false,
        has_varwidths: s & 1 == 1,
        has_nulls: true,
    });
    let im2 = Infomask2::try_make(((s % 1665) as u16) | 0xC000).unwrap();
    let excl = EXCL;
    let oid = Oid::<Relation>::try_make((s as u32) | 1).unwrap();
    let sel = Selectivity::try_make(0.5).unwrap()
        * Selectivity::clamped(s as f64 * 1e-9);
    let mut acc = u64::from(off.raw()) ^ u64::from(blk.raw());
    acc = acc.wrapping_add(u64::from(a.precedes(b)));
    acc = acc.wrapping_add((end - start).raw());
    acc ^= u64::from(item.raw()) ^ u64::from(item.is_used());
    acc ^= u64::from(info.raw()) ^ u64::from(info.size().raw());
    acc ^= u64::from(im2.natts().raw()) ^ u64::from(im2.heap_only());
    acc ^= u64::from(excl);
    acc ^= u64::from(oid.raw());
    acc ^= sel.raw().to_bits();
    return acc;
}
"""

# The two APIs differ only in these spellings.
FLAVOR = {
    "tk": {
        "NORMAL": "LpFlags::NORMAL",
        "EXCL": "Infomask::XmaxShrLock.into_value()"
        ".contains(Infomask::XmaxExclLock)",
    },
    "hand": {
        "NORMAL": "LpFlags::Normal",
        "EXCL": "Infomask::XMAX_SHR_LOCK.contains(Infomask::XMAX_EXCL_LOCK)",
    },
}
FLAG_START = {"tk": "/// `t_infomask` flags", "hand": "bitflags::bitflags! {"}


def template(variant):
    return (HERE / f"{variant}.rs").read_text()


def flag_section(variant):
    """The `t_infomask` declaration: from its first line to the next `}`."""
    lines = template(variant).splitlines()
    start = next(
        i for i, it in enumerate(lines) if it.startswith(FLAG_START[variant])
    )
    end = next(i for i in range(start, len(lines)) if lines[i] == "}")
    return "\n".join(lines[start : end + 1]) + "\n"


def package(name, dependency):
    return (
        f'[package]\nname = "{name}"\nversion = "0.0.0"\n'
        f'edition = "2024"\npublish = false\n\n'
        f"[dependencies]\n{dependency}\n"
    )


def write(path, text):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text)


def downstream(variant, modules):
    body = "".join(
        f"mod {m} {{"
        + EXERCISE.replace("CRATE", f"{variant}_types")
        .replace("MODULE", m)
        .replace("NORMAL", FLAVOR[variant]["NORMAL"])
        .replace("EXCL", FLAVOR[variant]["EXCL"])
        + "}\n"
        for m in modules
    )
    calls = ", ".join(f"{m}::ex" for m in modules)
    return body + (
        "\nfn main() {\n"
        "    let s = std::hint::black_box("
        "std::env::args().count() as u64 * 0x9E37_79B9);\n"
        "    let mut acc = 0u64;\n"
        f"    for f in [{calls}] {{\n"
        "        acc = acc.rotate_left(7) ^ f(s);\n"
        "    }\n"
        '    println!("{acc:#018x}");\n'
        "}\n"
    )


def generate(out, kind, types):
    """Write the workspace to `out` (replacing it) and return its metadata."""
    if kind == "mixed" and types % TYPES_PER_COPY:
        raise SystemExit(f"mixed needs a multiple of {TYPES_PER_COPY} types")
    copies = types // TYPES_PER_COPY if kind == "mixed" else types
    width = max(2, len(str(copies - 1)))
    prefix = "m" if kind == "mixed" else "f"
    modules = [f"{prefix}{i:0{width}}" for i in range(copies)]
    out = pathlib.Path(out)
    if out.exists():
        shutil.rmtree(out)
    variants = ("tk", "hand")
    members = [f"{v}_types" for v in variants]
    if kind == "mixed":
        members += [f"{v}_use" for v in variants]
    listed = ", ".join(f'"{m}"' for m in members)
    write(
        out / "Cargo.toml",
        f'[workspace]\nresolver = "3"\nmembers = [{listed}]\n',
    )
    # Seed the lock file with typekin's own dependency versions.
    shutil.copy(REPO / "Cargo.lock", out / "Cargo.lock")
    dependency = {
        "tk": f'typekin = {{ path = "{REPO}" }}',
        "hand": f'bitflags = "{BITFLAGS}"',
    }
    for v in variants:
        source = template(v) if kind == "mixed" else flag_section(v)
        root = out / f"{v}_types"
        write(root / "Cargo.toml", package(f"{v}_types", dependency[v]))
        write(
            root / "src/lib.rs",
            "#![allow(dead_code)]\n"
            + "".join(f"pub mod {m};\n" for m in modules),
        )
        for m in modules:
            write(root / f"src/{m}.rs", source)
        if kind == "mixed":
            use = out / f"{v}_use"
            types_dep = f'{v}_types = {{ path = "../{v}_types" }}'
            write(use / "Cargo.toml", package(f"{v}_use", types_dep))
            write(use / "src/main.rs", downstream(v, modules))
    meta = {
        "kind": kind,
        "types": types,
        "copies": copies,
        "modules": modules,
        "members": members,
    }
    write(out / "bench.json", json.dumps(meta, indent=1) + "\n")
    return meta


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("out", type=pathlib.Path)
    parser.add_argument("--kind", choices=["mixed", "flags"], default="mixed")
    parser.add_argument("--types", type=int, default=256)
    args = parser.parse_args()
    meta = generate(args.out, args.kind, args.types)
    print(
        f"{args.out}: {meta['kind']}, {meta['types']} typekin types, "
        f"members {', '.join(meta['members'])}"
    )


if __name__ == "__main__":
    main()

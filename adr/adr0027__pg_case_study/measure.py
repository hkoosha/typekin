#!/usr/bin/env python3
"""Build the ADR 0027 workspaces, time them, and write a Markdown report.

Scenarios (each: median, min, max of --runs, plus peak RSS of the cargo tree):
  full        touch lib.rs, CARGO_INCREMENTAL=0; check/debug/release of the
              types crate.
  edit        toggle one literal in m00.rs, default incremental; check and
              debug of the types crate, debug of the downstream crate.
  downstream  touch main.rs, CARGO_INCREMENTAL=0; check/debug/release of the
              downstream crate only.
  scaling     `full` check only, at the --scaling sizes.
  flags       `full` check only, bitflag-only workspace.
  detail      at --detail size: check with the proc macro built at
              opt-level 3; `rustc -Z time-passes`; clean builds including
              dependencies (`--timings`); rlib/rmeta/binary/test-function
              sizes.
  expand      `cargo expand` of the 16-type workspace; lines per declaration.
  parity      both downstream binaries must print the same checksums in
              debug and release.

Requires rustup with the --toolchain (default nightly; time-passes needs it),
binutils (`strip`, `nm`), and cargo-expand unless `--skip expand`.

Usage: measure.py [--out DIR] [--offline] [--runs 7] [--skip flags,scaling]
"""

import argparse
import json
import os
import pathlib
import re
import shutil
import statistics
import subprocess
import sys
import tempfile
import time

import gen

PROFILE_ARGS = {"check": ["check"], "debug": ["build"], "release": ["build", "--release"]}
# Environment that would change what is measured.
SCRUB = (
    "RUSTFLAGS",
    "CARGO_ENCODED_RUSTFLAGS",
    "CARGO_BUILD_RUSTFLAGS",
    "CARGO_TARGET_DIR",
    "CARGO_INCREMENTAL",
)
PASSES = (
    "macro_expand_crate",
    "type_check_crate",
    "coherence_checking",
    "MIR_borrow_checking",
    "generate_crate_metadata",
    "codegen_crate",
    "LLVM_passes",
    "link",
    "total",
)
OPT3 = {"CARGO_PROFILE_DEV_BUILD_OVERRIDE_OPT_LEVEL": "3"}
NO_INCREMENTAL = {"CARGO_INCREMENTAL": "0"}


class Bench:
    def __init__(self, args):
        self.toolchain = args.toolchain
        self.offline = args.offline
        self.base_env = {k: v for k, v in os.environ.items() if k not in SCRUB}

    def cargo(self, sub, *rest, extra=()):
        """`cargo +TC SUB [--offline] REST [-- EXTRA]`."""
        cmd = ["cargo", f"+{self.toolchain}", *sub]
        if self.offline:
            cmd.append("--offline")
        cmd += rest
        if extra:
            cmd += ["--", *extra]
        return cmd

    def run(self, cmd, cwd, env=None):
        """Run to completion; return seconds, peak RSS in MiB, out, err."""
        full_env = dict(self.base_env, **(env or {}))
        with tempfile.TemporaryFile() as out, tempfile.TemporaryFile() as err:
            start = time.perf_counter()
            proc = subprocess.Popen(
                cmd, cwd=cwd, env=full_env, stdout=out, stderr=err
            )
            # wait4 reports the peak RSS of the child and every descendant
            # it waited for, i.e. the rustc processes cargo spawned.
            _, status, usage = os.wait4(proc.pid, 0)
            seconds = time.perf_counter() - start
            proc.returncode = os.waitstatus_to_exitcode(status)
            out.seek(0)
            err.seek(0)
            stdout = out.read().decode(errors="replace")
            stderr = err.read().decode(errors="replace")
        if proc.returncode:
            raise SystemExit(
                f"failed: {' '.join(map(str, cmd))} in {cwd}\n{stderr[-4000:]}"
            )
        return seconds, usage.ru_maxrss / 1024, stdout, stderr

    def series(self, cmd, cwd, env, prepare, runs):
        self.run(cmd, cwd, env)  # settle fingerprints for this environment
        samples = []
        for _ in range(runs):
            prepare()
            seconds, rss, _, _ = self.run(cmd, cwd, env)
            samples.append((seconds, rss))
        times = [s for s, _ in samples]
        return {
            "median": statistics.median(times),
            "min": min(times),
            "max": max(times),
            "rss": max(r for _, r in samples),
            "runs": runs,
        }


def touch(path):
    return lambda: os.utime(path)


def toggle(path):
    def prepare():
        text = path.read_text()
        if gen.EDIT_FROM in text:
            path.write_text(text.replace(gen.EDIT_FROM, gen.EDIT_TO))
        else:
            path.write_text(text.replace(gen.EDIT_TO, gen.EDIT_FROM))

    return prepare


def log(message):
    print(message, file=sys.stderr, flush=True)


def measure_mixed(bench, root, meta, runs, scaling_only=False):
    rows = []
    first = meta["modules"][0]

    def add(variant, mode, target, scenario, prepare, env):
        pkg = f"{variant}_{target}"
        cmd = bench.cargo(PROFILE_ARGS[mode], "-p", pkg)
        result = bench.series(cmd, root, env, prepare, runs)
        result.update(
            types=meta["types"],
            variant=variant,
            mode=mode,
            target=target,
            scenario=scenario,
        )
        rows.append(result)
        log(
            f"{meta['types']:>5} {variant:<4} {scenario:<10} {mode:<7} "
            f"{target:<5} {result['median']:.2f}s {result['rss']:.0f} MiB"
        )

    modes = ("check",) if scaling_only else ("check", "debug", "release")
    for mode in modes:
        for v in ("tk", "hand"):
            lib = root / f"{v}_types/src/lib.rs"
            add(v, mode, "types", "full", touch(lib), NO_INCREMENTAL)
    if scaling_only or meta["kind"] != "mixed":
        return rows
    for v in ("tk", "hand"):
        edited = toggle(root / f"{v}_types/src/{first}.rs")
        add(v, "check", "types", "edit", edited, {})
        add(v, "debug", "types", "edit", edited, {})
        add(v, "debug", "use", "edit", edited, {})
    for mode in ("check", "debug", "release"):
        for v in ("tk", "hand"):
            main_rs = root / f"{v}_use/src/main.rs"
            add(v, mode, "use", "downstream", touch(main_rs), NO_INCREMENTAL)
    return rows


def parity(bench, root):
    outputs = {}
    for mode in ("debug", "release"):
        bench.run(
            bench.cargo(PROFILE_ARGS[mode], "-p", "tk_use", "-p", "hand_use"),
            root,
        )
        for v in ("tk", "hand"):
            exe = root / f"target/{mode}/{v}_use"
            for args in ([], ["x", "y"], ["a", "b", "c", "d"]):
                out = bench.run([exe, *args], root)[2].strip()
                outputs.setdefault((mode, len(args)), {})[v] = out
    for key, pair in outputs.items():
        if pair["tk"] != pair["hand"]:
            raise SystemExit(f"parity mismatch at {key}: {pair}")
    return {f"{m}/{n} args": p["tk"] for (m, n), p in outputs.items()}


def time_passes(bench, root, variant, env):
    os.utime(root / f"{variant}_types/src/lib.rs")
    cmd = bench.cargo(
        ["rustc"], "-p", f"{variant}_types", extra=["-Z", "time-passes"]
    )
    stderr = bench.run(cmd, root, dict(NO_INCREMENTAL, **env))[3]
    found = {}
    for line in stderr.splitlines():
        match = re.match(r"time:\s+([\d.]+);.*\s(\S+)$", line)
        if match and match.group(2) in PASSES:
            found[match.group(2)] = float(match.group(1))
    return found


def unit_durations(target_dir):
    html = target_dir / "cargo-timings/cargo-timing.html"
    if not html.exists():
        return None
    match = re.search(r"const UNIT_DATA = (\[.*?\]);", html.read_text(), re.S)
    if not match:
        return None
    units = json.loads(match.group(1))
    units.sort(key=lambda u: -u["duration"])
    return [
        {"unit": f"{u['name']} {u.get('target', '').strip()}".strip(),
         "seconds": round(u["duration"], 2)}
        for u in units[:6]
    ]


def newest(base, pattern):
    return max(base.rglob(pattern), key=lambda p: p.stat().st_mtime)


def detail(bench, root, meta, runs):
    out = {"types": meta["types"]}
    lib = root / "tk_types/src/lib.rs"
    cmd = bench.cargo(PROFILE_ARGS["check"], "-p", "tk_types")
    out["opt3_check"] = bench.series(
        cmd, root, dict(NO_INCREMENTAL, **OPT3), touch(lib), runs
    )
    log(f"detail: opt-level 3 proc macro check {out['opt3_check']['median']:.2f}s")
    out["time_passes"] = {
        "tk": time_passes(bench, root, "tk", {}),
        "tk_opt3": time_passes(bench, root, "tk", OPT3),
        "hand": time_passes(bench, root, "hand", {}),
    }
    out["clean"] = {}
    out["sizes"] = {}
    for v in ("tk", "hand"):
        target = root / f"target-clean-{v}"
        if target.exists():
            shutil.rmtree(target)
        env = {"CARGO_TARGET_DIR": str(target)}
        clean = {}
        for mode in ("debug", "release"):
            args = PROFILE_ARGS[mode] + (["--timings"] if mode == "debug" else [])
            seconds, rss, _, _ = bench.run(
                bench.cargo(args, "-p", f"{v}_use"), root, env
            )
            clean[mode] = {"seconds": seconds, "rss": rss}
        clean["units_debug"] = unit_durations(target)
        out["clean"][v] = clean
        log(f"detail: clean {v} debug {clean['debug']['seconds']:.2f}s")
        for mode in ("debug", "release"):
            base = target / mode
            exe = base / f"{v}_use"
            with tempfile.TemporaryDirectory() as tmp:
                stripped = pathlib.Path(tmp) / "stripped"
                bench.run(["strip", "-o", stripped, exe], root)
                stripped_size = stripped.stat().st_size
            symbols = bench.run(
                ["nm", "--print-size", "--size-sort", "-C", exe], root
            )[2]
            ex_bytes = sum(
                int(line.split()[1], 16)
                for line in symbols.splitlines()
                if re.search(r"::[mf]\d+::ex$", line)
            )
            out["sizes"][f"{v}/{mode}"] = {
                "rlib": newest(base, f"lib{v}_types-*.rlib").stat().st_size,
                "rmeta": newest(base, f"lib{v}_types-*.rmeta").stat().st_size,
                "exe": exe.stat().st_size,
                "stripped": stripped_size,
                "ex_fns": ex_bytes,
            }
    return out


def code_lines(text):
    return sum(
        1
        for line in text.splitlines()
        if line.strip() and not line.strip().startswith("//")
    )


def declarations():
    """(name, typekin macro) for each annotated item in tk.rs, in order."""
    found, kind = [], None
    for line in gen.template("tk").splitlines():
        attr = re.match(r"#\[typekin::(\w+)", line)
        if attr:
            kind = attr.group(1)
        item = re.match(r"pub (?:struct|enum) (\w+)", line)
        if item and kind:
            found.append((item.group(1), kind))
            kind = None
    return found


def expand(bench, root):
    out = {}
    texts = {}
    for v in ("tk", "hand"):
        cmd = bench.cargo(["expand"], "-p", f"{v}_types", "--lib")
        texts[v] = bench.run(cmd, root)[2]
        out[f"{v}_lines"] = code_lines(texts[v])
    lines = texts["tk"].splitlines()
    starts = []
    for name, kind in declarations():
        pattern = re.compile(rf"\s*pub (?:struct|enum) {name}\b")
        index = next(i for i, line in enumerate(lines) if pattern.match(line))
        starts.append((index, name, kind))
    starts.sort()
    per_type = []
    for (start, name, kind), (end, _, _) in zip(
        starts, starts[1:] + [(len(lines), None, None)]
    ):
        per_type.append(
            {
                "type": name,
                "macro": kind,
                "lines": code_lines("\n".join(lines[start:end])),
            }
        )
    out["per_type"] = sorted(per_type, key=lambda r: -r["lines"])
    return out


def fmt_row(row):
    return f"{row['median']:.2f} s [{row['min']:.2f}–{row['max']:.2f}]"


def report(results):
    env = results["environment"]
    out = [
        "# ADR 0027 compile-cost benchmark",
        "",
        f"- rustc: `{env['rustc']}`",
        f"- CPU: {env['cpu']} ({env['cpus']} threads)",
        f"- Runs per scenario: {env['runs']} (scaling {env['scaling_runs']}, "
        f"flags {env['flag_runs']}); medians with [min–max].",
        "",
    ]
    if results.get("parity"):
        out += ["## Parity", "", "Downstream checksums match in debug and release:", ""]
        out += [f"- {k}: `{v}`" for k, v in results["parity"].items()]
        out.append("")
    rows = results.get("rows", [])
    for types in sorted({r["types"] for r in rows if r["scenario"] != "scaling"}):
        out += [
            f"## {types} types",
            "",
            "| Scenario | Command | typekin | hand + bitflags | Ratio | RSS typekin / hand (MiB) |",
            "|---|---|---|---|---|---|",
        ]
        picked = [r for r in rows if r["types"] == types and r["scenario"] not in ("scaling",)]
        keys = []
        for r in picked:
            key = (r["scenario"], r["mode"], r["target"])
            if key not in keys:
                keys.append(key)
        for scenario, mode, target in keys:
            pair = {
                r["variant"]: r
                for r in picked
                if (r["scenario"], r["mode"], r["target"]) == (scenario, mode, target)
            }
            tk, hand = pair["tk"], pair["hand"]
            out.append(
                f"| {scenario} | {mode} `{target}` | {fmt_row(tk)} | {fmt_row(hand)} "
                f"| {tk['median'] / hand['median']:.1f}× | {tk['rss']:.0f} / {hand['rss']:.0f} |"
            )
        out.append("")
    scaling = [r for r in rows if r["scenario"] == "scaling"]
    if scaling:
        out += [
            "## Scaling (`cargo check`, types crate)",
            "",
            "| Types | typekin | hand + bitflags | typekin ms/type | Peak RSS typekin (MiB) |",
            "|---|---|---|---|---|",
        ]
        for types in sorted({r["types"] for r in scaling}):
            pair = {r["variant"]: r for r in scaling if r["types"] == types}
            out.append(
                f"| {types} | {fmt_row(pair['tk'])} | {fmt_row(pair['hand'])} "
                f"| {1000 * pair['tk']['median'] / types:.1f} | {pair['tk']['rss']:.0f} |"
            )
        out.append("")
    flags = results.get("flags")
    if flags:
        pair = {r["variant"]: r for r in flags}
        out += [
            f"## {pair['tk']['types']} bitflag sets only (`cargo check`)",
            "",
            "| | typekin | bitflags crate |",
            "|---|---|---|",
            f"| Time | {fmt_row(pair['tk'])} | {fmt_row(pair['hand'])} |",
            f"| Peak RSS (MiB) | {pair['tk']['rss']:.0f} | {pair['hand']['rss']:.0f} |",
            "",
        ]
    d = results.get("detail")
    if d:
        out += [
            f"## Detail at {d['types']} types",
            "",
            f"- `check` with the proc macro at opt-level 3: {fmt_row(d['opt3_check'])}",
            "",
            "| rustc pass (s) | typekin | typekin, macro at opt 3 | hand |",
            "|---|---|---|---|",
        ]
        tp = d["time_passes"]
        for name in PASSES:
            cells = [tp[k].get(name) for k in ("tk", "tk_opt3", "hand")]
            if any(c is not None for c in cells):
                shown = " | ".join("—" if c is None else f"{c:.3f}" for c in cells)
                out.append(f"| {name} | {shown} |")
        out += ["", "| Clean build incl. dependencies | typekin | hand + bitflags |", "|---|---|---|"]
        for mode in ("debug", "release"):
            out.append(
                f"| {mode} | {d['clean']['tk'][mode]['seconds']:.2f} s "
                f"| {d['clean']['hand'][mode]['seconds']:.2f} s |"
            )
        for v in ("tk", "hand"):
            units = d["clean"][v]["units_debug"]
            if units:
                listed = ", ".join(f"{u['unit']} {u['seconds']} s" for u in units)
                out.append(f"\nSlowest units, clean debug, {v}: {listed}")
        out += [
            "",
            "| Artifact | typekin debug | hand debug | typekin release | hand release |",
            "|---|---|---|---|---|",
        ]
        s = d["sizes"]
        for key, label, scale, unit in (
            ("rlib", "types rlib", 1e6, "MB"),
            ("rmeta", "types rmeta", 1e6, "MB"),
            ("exe", "downstream binary", 1e6, "MB"),
            ("stripped", "stripped binary", 1e3, "KB"),
            ("ex_fns", "test functions (`ex`)", 1e3, "KB"),
        ):
            cells = " | ".join(
                f"{s[f'{v}/{m}'][key] / scale:.2f} {unit}"
                for m in ("debug", "release")
                for v in ("tk", "hand")
            )
            out.append(f"| {label} | {cells} |")
        out.append("")
    e = results.get("expand")
    if e:
        out += [
            "## Expansion of 16 types (`cargo expand`, non-blank non-comment lines)",
            "",
            f"typekin {e['tk_lines']}, hand + bitflags {e['hand_lines']} "
            f"({e['tk_lines'] / e['hand_lines']:.1f}×).",
            "",
            "| Type | Macro | Expanded lines |",
            "|---|---|---|",
        ]
        out += [f"| {r['type']} | {r['macro']} | {r['lines']} |" for r in e["per_type"]]
        out.append("")
    return "\n".join(out)


def environment(bench, args):
    rustc = bench.run(["rustc", f"+{bench.toolchain}", "--version"], ".")[2].strip()
    cpu = "unknown"
    cpuinfo = pathlib.Path("/proc/cpuinfo")
    if cpuinfo.exists():
        match = re.search(r"model name\s*:\s*(.+)", cpuinfo.read_text())
        cpu = match.group(1).strip() if match else cpu
    return {
        "rustc": rustc,
        "cpu": cpu,
        "cpus": os.cpu_count(),
        "runs": args.runs,
        "scaling_runs": args.scaling_runs,
        "flag_runs": args.flag_runs,
    }


def sizes(text):
    return [int(it) for it in text.split(",") if it.strip()]


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--out", type=pathlib.Path, default=pathlib.Path("/tmp/typekin-adr0027"))
    parser.add_argument("--sizes", type=sizes, default=[16, 256])
    parser.add_argument("--scaling", type=sizes, default=[512, 1024])
    parser.add_argument("--flags", type=int, default=256)
    parser.add_argument("--detail", type=int, default=256)
    parser.add_argument("--runs", type=int, default=7)
    parser.add_argument("--scaling-runs", type=int, default=3)
    parser.add_argument("--flag-runs", type=int, default=5)
    parser.add_argument("--toolchain", default="nightly")
    parser.add_argument("--offline", action="store_true")
    parser.add_argument(
        "--skip",
        type=lambda s: set(filter(None, s.split(","))),
        default=set(),
        help="comma list of: scaling, flags, detail, expand, parity",
    )
    args = parser.parse_args()
    bench = Bench(args)
    args.out.mkdir(parents=True, exist_ok=True)
    results = {"environment": environment(bench, args), "rows": []}

    workspaces = {}
    for types in args.sizes:
        root = args.out / f"mixed{types}"
        workspaces[types] = (root, gen.generate(root, "mixed", types))
        log(f"generated {root}")
        root, meta = workspaces[types]
        bench.run(bench.cargo(["build"]), root)  # dependencies and warm-up
        if "parity" not in args.skip:
            results.setdefault("parity", {}).update(
                {f"{types} types, {k}": v for k, v in parity(bench, root).items()}
            )
        results["rows"] += measure_mixed(bench, root, meta, args.runs)

    if "scaling" not in args.skip:
        for types in args.scaling:
            root = args.out / f"mixed{types}"
            meta = gen.generate(root, "mixed", types)
            for row in measure_mixed(bench, root, meta, args.scaling_runs, scaling_only=True):
                row["scenario"] = "scaling"
                results["rows"].append(row)
        # The --sizes runs belong on the scaling curve too.
        for row in list(results["rows"]):
            if row["scenario"] == "full" and row["mode"] == "check" and row["target"] == "types":
                results["rows"].append(dict(row, scenario="scaling"))

    if "flags" not in args.skip and args.flags:
        root = args.out / f"flags{args.flags}"
        meta = gen.generate(root, "flags", args.flags)
        results["flags"] = measure_mixed(bench, root, meta, args.flag_runs, scaling_only=True)

    if "detail" not in args.skip:
        if args.detail not in workspaces:
            raise SystemExit("--detail must be one of --sizes")
        root, meta = workspaces[args.detail]
        results["detail"] = detail(bench, root, meta, args.runs)

    if "expand" not in args.skip:
        root = args.out / "mixed16"
        if 16 not in workspaces:
            gen.generate(root, "mixed", 16)
        results["expand"] = expand(bench, root)

    (args.out / "results.json").write_text(json.dumps(results, indent=1) + "\n")
    text = report(results)
    (args.out / "report.md").write_text(text + "\n")
    print(text)
    log(f"wrote {args.out / 'report.md'} and {args.out / 'results.json'}")


if __name__ == "__main__":
    main()

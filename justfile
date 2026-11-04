set shell := ["bash", "-e", "-u", "-o", "pipefail", "-c"]

export RUST_BACKTRACE := 'full'
export RUSTFLAGS := '-Zmacro-backtrace'

[group("z")]
@def:
  just -l

fmt: fmt-markdown fmt-rs

[group("z")]
fmt-markdown:
  rumdl fmt --deny-config-warnings --no-cache *.md &>/dev/null
  rumdl fmt --deny-config-warnings --no-cache arena/*.md &>/dev/null || true

[group("z")]
fmt-rs:
  cargo fmt

[group("z")]
test-here: fmt z-test

[group("z")]
z-test *args:
  runner=test; \
  if cargo nextest --version &>/dev/null; then runner='nextest run'; fi; \
  time cargo $runner --offline {{ args }}

[group("z")]
[working-directory: 'typekin_testing']
@test-there-fast:
  just test-fast
[group("z")]
[working-directory: 'typekin_testing']
@test-there-slow:
  just test-slow

[group("z")]
[working-directory: 'typekin_testing']
@clean-there:
  just clean
[group("z")]
[working-directory: 'typekin_testing']
clean-here:
  cargo clean
clean: clean-here clean-there

test-slow: test-here test-there-slow
test-fast: test-here test-there-fast
alias test := test-fast
alias test-all := test-slow

build: fmt
  cargo build

clippy: fmt
  cargo clippy --offline --locked --frozen

fix: fmt
  cargo clippy --offline --locked --frozen --fix

# The expansion snapshot: typekin's own expansions of every testing-crate
# file with a typekin attribute, trailing commas stripped.
[group("z")]
snapshot out:
  rm -rf '{{ out }}'
  mkdir -p '{{ out }}'
  grep -l '#\[typekin::' typekin_testing/src/*.rs \
    | xargs -P 6 -I{} sh -c 'expandem "$1" "typekin::**" > "$2/${1##*/}"' _ {} '{{ out }}'
  perl -0pi -e 's/,(\s*[)\]}])/$1/g' '{{ out }}'/*

# A fresh snapshot, diffed against the shared base.
[group("z")]
snapshot-check name: (snapshot "/tmp/typekin-snap/" + name)
  diff -r /tmp/typekin-snap/base '/tmp/typekin-snap/{{ name }}'

# Regenerates every `_exp.rs`; fails when one changes.
[group("z")]
examples-check:
  just --summary | tr ' ' '\n' | grep -- '-expand$' | grep -v '^z' \
    | xargs -P 6 -n 1 just
  cargo +nightly fmt
  git diff --exit-code -- examples/

# Format, lint, typekin's own tests and the fast consumer tests.
[group("z")]
verify:
  cargo +nightly fmt -- --check
  cargo +nightly fmt --manifest-path typekin_testing/Cargo.toml -- --check
  cargo +nightly clippy --locked --lib --tests -- -D warnings
  cargo +nightly test --locked
  cargo +nightly test --locked --manifest-path typekin_testing/Cargo.toml --features test_fast

# A refactor task: `verify`, and neither the snapshot nor an example changes.
[group("z")]
gate name: verify (snapshot-check name) examples-check

# A phase's exit, on the merged result.
[group("z")]
gate-phase name: (gate name)
  cargo +nightly test --locked --manifest-path typekin_testing/Cargo.toml --features test_slow
  cargo +nightly test --locked --lib --features nightly
  for example in my_u32_non_const my_u32_stable_const my_tcp my_text my_flag my_f64 my_unit my_f32_point my_packed; do env -u RUSTFLAGS cargo +stable run --locked --quiet --example "$example"; done

# Functions in `src/` longer than `max` lines, longest first.
[group("z")]
fn-lengths max='100':
  #!/usr/bin/env bash
  set -euo pipefail
  awk -v max='{{ max }}' '
    FNR == 1 { open = 0 }
    !open && /^ *(pub(\([a-z]+\))? )?(const )?fn [A-Za-z_0-9]+/ && !/;$/ {
      match($0, /^ */); start = FNR; open = 1
      name = $0
      sub(/^ *(pub(\([a-z]+\))? )?(const )?fn /, "", name)
      sub(/[(<].*/, "", name)
      closer = sprintf("%" RLENGTH "s}", "")
      next
    }
    open && $0 == closer {
      if (FNR - start + 1 > max) print FILENAME ":" start ": " name " " FNR - start + 1
      open = 0
    }
  ' src/*.rs | sort -t ' ' -k 3 -n -r

[group("z")]
init-here:
  cargo install expandem
[group("z")]
[working-directory: 'typekin_testing']
init-there:
  just init
init: init-here init-there

examples := 'examples'

[group("z")]
z-expand what: (
    zz-expand
    examples / (what + '.rs')
    examples / (what + '_exp.rs')
  )

[group("z")]
zz-expand from to:
  rm -rf '{{ to }}'
  touch '{{ to }}'
  expandem --fix '{{ from }}' 'typekin::**' > '{{ to }}'

[group("z")]
@new it:
  target="$(printf '%s' '{{ it }}' | tr '_' '-')"; \
  file="$(printf '%s' '{{ it }}' | tr '-' '_')"; \
  just z-new "$target" "$file"

[group("z")]
z-new it file:
  printf 'fn main(){}\n' > 'examples/my_{{ file }}.rs'
  printf 'fn main(){}\n' > 'examples/my_{{ file }}_exp.rs'
  printf \
    '[[example]]\nname = "my_%s"\npath = "examples/my_%s.rs"\n[[example]]\nname = "my_%s_exp"\npath = "examples/my_%s_exp.rs"\n' \
    '{{ file }}' '{{ file }}' '{{ file }}' '{{ file }}' \
    >> Cargo.toml
  sed -i '/^expandem:/ s/$/ {{ it }}-expand/' justfile;
  printf \
    "\n[group(\"example\")]\n%s-expand: (z-expand 'my_%s')\n[group(\"example\")]\n%s: %s-expand fmt\n" \
    '{{ it }}' '{{ file }}' '{{ it }}' '{{ it }}' \
    >> justfile

[parallel]
expandem: u32-expand u16-expand non-expand sequence-expand tcp-expand plain-expand i128-expand flag-expand friend-expand friendship-expand friendships-expand text-expand u32-stable-const-expand blind-trust-expand f64-expand unit-expand f32-point-expand lit-expand proven-expand within-expand packed-expand
  @just fmt

[group("example")]
friend-expand: (z-expand 'my_friend')
[group("example")]
friend: friend-expand fmt

[group("example")]
friendship-expand: (z-expand 'my_friendship')
[group("example")]
friendship: friendship-expand fmt

[group("example")]
friendships-expand: (z-expand 'my_friendships')
[group("example")]
friendships: friendships-expand fmt

[group("example")]
u32-expand: (z-expand 'my_u32')
[group("example")]
u32: u32-expand fmt

[group("example")]
u16-expand: (z-expand 'my_u16')
[group("example")]
u16: u16-expand fmt

[group("example")]
i128-expand: (z-expand 'my_i128')
[group("example")]
i128: i128-expand fmt

[group("example")]
non-expand: (z-expand 'my_u32_non_const')
[group("example")]
non: non-expand fmt

[group("example")]
sequence-expand: (z-expand 'my_sequence')
[group("example")]
sequence: sequence-expand fmt

[group("example")]
tcp-expand: (z-expand 'my_tcp')
[group("example")]
tcp: tcp-expand fmt

[group("example")]
flag-expand: (z-expand 'my_flag')
[group("example")]
flag: flag-expand fmt

[group("example")]
plain-expand: (z-expand 'my_plain')
[group("example")]
plain: plain-expand fmt

[group("example")]
text-expand: (z-expand 'my_text')
[group("example")]
text: text-expand fmt

[group("example")]
u32-stable-const-expand: (z-expand 'my_u32_stable_const')
[group("example")]
u32-stable-const: u32-stable-const-expand fmt

[group("example")]
blind-trust-expand: (z-expand 'my_blind_trust')
[group("example")]
blind-trust: blind-trust-expand fmt

[group("example")]
f64-expand: (z-expand 'my_f64')
[group("example")]
f64: f64-expand fmt

[group("example")]
unit-expand: (z-expand 'my_unit')
[group("example")]
unit: unit-expand fmt

[group("example")]
f32-point-expand: (z-expand 'my_f32_point')
[group("example")]
f32-point: f32-point-expand fmt

[group("example")]
lit-expand: (z-expand 'my_lit')
[group("example")]
lit: lit-expand fmt

[group("example")]
proven-expand: (z-expand 'my_proven')
[group("example")]
proven: proven-expand fmt

[group("example")]
within-expand: (z-expand 'my_within')
[group("example")]
within: within-expand fmt

[group("example")]
packed-expand: (z-expand 'my_packed')
[group("example")]
packed: packed-expand fmt

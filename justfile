set shell := ["bash", "-e", "-u", "-o", "pipefail", "-c"]

targets := "wasm32-unknown-unknown aarch64-unknown-linux-gnu armv7-unknown-linux-gnueabihf i686-unknown-linux-gnu"
export RUST_BACKTRACE := 'full'
export RUSTFLAGS := '-Zmacro-backtrace'

[group("z")]
@def:
  just -l

[group("z")]
markdown:
  rumdl fmt --deny-config-warnings --no-cache *.md &>/dev/null
  rumdl fmt --deny-config-warnings --no-cache adr/*.md &>/dev/null
  rumdl fmt --deny-config-warnings --no-cache arena/*.md &>/dev/null || true

[group("z")]
fmt: markdown
  cargo fmt

test *what: fmt
  time cargo +nightly fmt -- --check
  time cargo +nightly clippy --locked --lib -- -D warnings
  runner="$(if cargo nextest --version &>/dev/null; then echo 'nextest run'; else echo 'test'; fi)"; \
  time cargo +nightly --offline --locked $runner {{ what }}

test-fast:
  just test --no-default-features

clean:
  cargo clean

build: fmt
  cargo build

clippy: fmt
  cargo clippy --offline --locked --frozen

fix: fmt
  cargo clippy --offline --locked --frozen --fix

init:
  cargo install expandem
  for target in {{ targets }}; do \
    rustup +nightly target add "$target"; done
  rustup +nightly component add llvm-tools

[group("z")]
z-expand it:
  from='examples/{{ it }}.rs';\
  to='examples/{{ it }}_exp.rs';\
  rm -rf "$to";\
  touch "$to";\
  if ! expandem --fix "$from" 'typekin::**' >> "$to"; then echo 'failed: {{ it }}'; exit 1; fi

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
all: u32-expand u16-expand non-expand sequence-expand tcp-expand plain-expand i128-expand flag-expand friend-expand friendship-expand friendships-expand text-expand u32-stable-const-expand blind-trust-expand f64-expand unit-expand f32-point-expand lit-expand proven-expand within-expand packed-expand nominal-expand
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

[group("example")]
nominal-expand: (z-expand 'my_nominal')
[group("example")]
nominal: nominal-expand fmt

[group("example")]
panic-at-expand: (z-expand 'my_panic_at')
[group("example")]
panic-at: panic-at-expand fmt

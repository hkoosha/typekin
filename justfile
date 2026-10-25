set shell := ["bash", "-e", "-u", "-o", "pipefail", "-c"]

export RUST_BACKTRACE := 'full'
export RUSTFLAGS := '-Zmacro-backtrace'

[group("z")]
@def:
  just -l

clean:
  cargo clean
  cd ./typekin_testing && pwd && just clean

fmt:
  rumdl fmt --deny-config-warnings --no-cache *.md
  rumdl fmt --deny-config-warnings --no-cache arena/*.md &>/dev/null || true
  cargo fmt

test-here: fmt
  time cargo test --offline

[working-directory: 'typekin_testing']
test-there-fast:
  just test-fast

[working-directory: 'typekin_testing']
test-there-slow:
  just test-slow

test-fast: test-here test-there-fast
test-slow: test-here test-there-slow
alias test := test-fast

build: fmt
  cargo build

clippy: fmt
  cargo clippy --offline --locked --frozen

fix: fmt
  cargo clippy --offline --locked --frozen --fix

init:
  cargo install expandem

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
  sed -i '/^all:/ s/$/ {{ it }}-expand/' justfile;
  printf \
    "\n[group(\"example\")]\n%s-expand: (z-expand 'my_%s')\n[group(\"example\")]\n%s: %s-expand fmt\n" \
    '{{ it }}' '{{ file }}' '{{ it }}' '{{ it }}' \
    >> justfile

[parallel]
all: u32-expand u16-expand non-expand sequence-expand plain-expand i128-expand flag-expand friend-expand friendship-expand friendships-expand text-expand u32-stable-const-expand unchecked-expand
  just fmt

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
unchecked-expand: (z-expand 'my_unchecked')
[group("example")]
unchecked: unchecked-expand fmt

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
  cargo fmt

test-here:
  cargo test
test-there:
  cd ./typekin_testing && pwd && just test
test: test-here test-there

build: fmt
  cargo build

clippy: fmt
  cargo clippy

init:
  cargo install expandem

# --------------------------------------

[group("example")]
friend-expand: (z-expand 'my_friend')
[group("example")]
friend: friend-expand fmt

[group("example")]
friendship-expand: (z-expand 'my_friendship')
[group("example")]
friendship: friendship-expand fmt

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
flag-expand: (z-expand 'my_flag')
[group("example")]
flag: flag-expand fmt

[group("example")]
plain-expand: (z-expand 'my_plain')
[group("example")]
plain: plain-expand fmt

[parallel]
all: u32-expand u16-expand non-expand plain-expand i128-expand flag-expand friend-expand friendship-expand
  just fmt

# ==============================================================================

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
  expandem '{{ from }}' 'typekin::**' > '{{ to }}'


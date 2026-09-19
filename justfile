set shell := ["bash", "-e", "-u", "-o", "pipefail", "-c"]

export RUST_BACKTRACE := '1'
export RUSTFLAGS := '-Zmacro-backtrace'

[group("z")]
@def:
  just -l

clean:
  cargo clean

fmt:
  cargo fmt

test:
  cargo test

build: fmt
  cargo build

alias run := prettify
@prettify file:
  cargo -q run -qp typekin -- '{{ file }}'


# --------------------------------------

[group("example")]
friend-expand: (z-expand 'my_friend')
[group("example")]
friend: friend-expand fmt

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
thingy-expand: (z-expand 'my_thingy')
[group("example")]
thingy: thingy-expand fmt

[group("example")]
flag-expand: (z-expand 'my_flag')
[group("example")]
flag: flag-expand fmt

[group("example")]
plain-expand: (z-expand 'my_plain')
[group("example")]
plain: plain-expand fmt

all: u32-expand u16-expand non-expand plain-expand i128-expand flag-expand fmt
  just fmt


# ==============================================================================

examples := 'crates' / 'typekin' / 'examples'

[group("z")]
z-expand what:
  touch '{{ examples / what }}_exp.rs'
  work="$(mktemp)" && \
  cargo -q expand --ugly --color never --package typekin --example '{{ what }}' > "$work" && \
  cp "$work" '{{ examples / what }}_exp.rs'
  work="$(mktemp)" && \
  just prettify '{{ examples / what }}_exp.rs' > "$work" && \
  cp "$work" '{{ examples / what }}_exp.rs'

fast-test:
  CARGO_TARGET_DIR=/tmp/typekin_perf_ok_outer cargo +nightly test -p typekin_perf_ok & \
  CARGO_TARGET_DIR=/tmp/typekin_perf_must_fail_outer cargo +nightly test -p typekin_perf_must_fail & \
  CARGO_TARGET_DIR=/tmp/typekin_perf_types_outer cargo +nightly test -p typekin_perf_types & \
  CARGO_TARGET_DIR=/tmp/typekin_outer cargo +nightly test -p typekin & \
  wait

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
u32-expand: (z-expand 'my_u32')
[group("example")]
u32-pretty: (z-pretty 'my_u32')
[group("example")]
u32: u32-expand u32-pretty fmt

[group("example")]
i128-expand: (z-expand 'my_i128')
[group("example")]
i128-pretty: (z-pretty 'my_i128')
[group("example")]
i128: i128-expand i128-pretty fmt

[group("example")]
plain-expand: (z-expand 'my_u32_non_const')
[group("example")]
plain-pretty: (z-pretty 'my_u32_non_const')
[group("example")]
plain: plain-expand plain-pretty fmt

[group("example")]
thingy-expand: (z-expand 'my_thingy')
[group("example")]
thingy-pretty: (z-pretty 'my_thingy')
[group("example")]
thingy: thingy-expand thingy-pretty fmt

[group("example")]
flag-expand: (z-expand 'my_flag')
[group("example")]
flag-pretty: (z-pretty 'my_flag')
[group("example")]
flag: flag-expand flag-pretty fmt

all: u32 plain i128 flag
  just fmt


# ==============================================================================

examples := 'crates' / 'typekin' / 'examples' 

[group("z")]
z-expand what:
  touch '{{ examples / what }}_exp.rs'
  work="$(mktemp)" && \
  cargo -q expand -p typekin --example '{{ what }}' > "$work" && \
  cp "$work" '{{ examples / what }}_exp.rs'

[group("z")]
z-pretty what:
  work="$(mktemp)" && \
  just prettify '{{ examples / what }}_exp.rs' > "$work" && \
  cp "$work" '{{ examples / what }}_exp.rs'


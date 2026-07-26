set shell := ["bash", "-e", "-u", "-o", "pipefail", "-c"]

examples := 'crates' / 'typekin' / 'examples' / 'my_'

[group("z")]
@def:
  just -l

[group("z")]
z-expand from to:
  work="$(mktemp)" && \
    cargo expand -p typekin --example '{{ from }}' > "$work" && \
    cargo run -p typekin -- "$work" > '{{ to }}'

[group("z")]
z-expand-example what: \
    (z-expand
      'my_' + what
      examples + what + '_expanded.rs'
    ) \
    fmt

# ==============================================================================

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
  cargo run -qp typekin -- {{ file }}

[group("example")]
u32: (z-expand-example 'u32')


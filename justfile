set shell := ["bash", "-e", "-u", "-o", "pipefail", "-c"]

import 'j_example.justfile'

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
  cargo fmt

test-here:
  time cargo test --offline

test-there: fmt
  cd ./typekin_testing && pwd && just test

test: test-here test-there

build: fmt
  cargo build

clippy: fmt
  cargo clippy

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
  expandem '{{ from }}' 'typekin::**' > '{{ to }}'

new it:
  [[ '{{ it }}' =~ ^[a-z][a-z0-9_-]*$ ]] || (echo 'bad it: {{ it }}' 1>&2; exit 1)
  [[ ! -e 'examples/my_{{ it }}.rs'   ]] || (echo 'already exists: examples/my_{{ it }}.rs' 1>&2; exit 1)
  printf 'fn main(){}\n' > 'examples/my_{{ it }}.rs'
  printf 'fn main(){}\n' > 'examples/my_{{ it }}_exp.rs'
  printf \
    '[[example]]\nname = "my_%s"\npath = "examples/my_%s.rs"\n[[example]]\nname = "my_%s_exp"\npath = "examples/my_%s_exp.rs"\n' \
    '{{ it }}' '{{ it }}' '{{ it }}' '{{ it }}' \
    >> Cargo.toml
  sed -i "/^all:/ s/$/ {{ it }}-expand/" j_examples.justfile
  printf \
    "\n[group(\"example\")]\n%s-expand: (z-expand 'my_%s')\n[group(\"example\")]\n%s: %s-expand fmt\n" \
    '{{ it }}' '{{ it }}' '{{ it }}' '{{ it }}' \
    >> j_examples.justfile

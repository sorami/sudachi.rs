#!/bin/bash
set -ex

# This script is assumed to be used inside https://github.com/pypa/manylinux.

DIR=$(dirname "$(readlink -f "$0")")

curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y --no-modify-path --component llvm-tools-preview
export PATH="$HOME/.cargo/bin:$PATH"

cd "$DIR/.."

PROFDATA=/tmp/sudachi-profdata

# Compile Binary that will generate PGO data
RUSTFLAGS="-C profile-generate=$PROFDATA -C opt-level=3" \
  cargo build --release -p sudachi-cli --target=x86_64-unknown-linux-gnu

# Download Kyoto Leads corpus original texts
curl -L https://github.com/ku-nlp/KWDLC/releases/download/release_1_0/leads.org.txt.gz | gzip -dc > leads.txt

# Generate Profile
target/x86_64-unknown-linux-gnu/release/sudachi -o /dev/null leads.txt
target/x86_64-unknown-linux-gnu/release/sudachi --wakati --mode=A -o /dev/null leads.txt
target/x86_64-unknown-linux-gnu/release/sudachi --all --mode=B -o /dev/null leads.txt

# Generate Merged PGO data
"$HOME/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-profdata" \
  merge -o /tmp/sudachi-profdata.merged "$PROFDATA"

cd "$DIR"

export RUSTFLAGS='-C profile-use=/tmp/sudachi-profdata.merged -C opt-level=3'
export CARGO_BUILD_TARGET=x86_64-unknown-linux-gnu

# see following link for the list of cpython bin
# https://github.com/pypa/manylinux?tab=readme-ov-file#image-content
# TODO: after supporting py313t, "/opt/python/cp{37,38,39,310,311,312,313}-*/bin" would suffice.
for PYBIN in /opt/python/cp*-cp{37m,38,39,310,311,312,313}/bin; do
    "${PYBIN}/pip" install -U setuptools wheel setuptools-rust
    find . -iname 'sudachipy*.so'
    rm -f build/lib/sudachipy/sudachipy*.so
    "${PYBIN}/pip" wheel . --no-build-isolation -vvv --wheel-dir ./dist
done

for whl in dist/*.whl; do
    auditwheel repair "$whl" -w dist/
done

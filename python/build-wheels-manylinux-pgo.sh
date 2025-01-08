#!/bin/bash
set -ex

# Build wheels with PGO, inside a manylinux container (https://github.com/pypa/manylinux)

DIR=$(dirname "$(readlink -f "$0")")

curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y --no-modify-path --component llvm-tools-preview
export PATH="$HOME/.cargo/bin:$PATH"

cd "$DIR/.."

TARGET_TRIPLE=$(rustc -vV | awk '/^host/ {print $2}')
PROFDATA=/tmp/sudachi-profdata

# Compile Binary that will generate PGO data
RUSTFLAGS="-C profile-generate=$PROFDATA -C opt-level=3" \
  cargo build --release -p sudachi-cli --target=$TARGET_TRIPLE

# Download Kyoto Leads corpus original texts
curl -L https://github.com/ku-nlp/KWDLC/releases/download/release_1_0/leads.org.txt.gz | gzip -dc > leads.txt

# Generate Profile
target/$TARGET_TRIPLE/release/sudachi -o /dev/null leads.txt
target/$TARGET_TRIPLE/release/sudachi --wakati --mode=A -o /dev/null leads.txt
target/$TARGET_TRIPLE/release/sudachi --all --mode=B -o /dev/null leads.txt

# Generate Merged PGO data
"$HOME/.rustup/toolchains/stable-$TARGET_TRIPLE/lib/rustlib/$TARGET_TRIPLE/bin/llvm-profdata" \
  merge -o /tmp/sudachi-profdata.merged "$PROFDATA"

cd "$DIR"

export RUSTFLAGS='-C profile-use=/tmp/sudachi-profdata.merged -C opt-level=3'
export CARGO_BUILD_TARGET=$TARGET_TRIPLE

# see following link for the list of cpython bin
# https://github.com/pypa/manylinux?tab=readme-ov-file#image-content
for PYBIN in /opt/python/cp{37,38,39,310,311,312,313}-*/bin; do
    "${PYBIN}/pip" install -U setuptools wheel setuptools-rust
    find . -iname 'sudachipy*.so'
    rm -f build/lib/sudachipy/sudachipy*.so
    "${PYBIN}/pip" wheel . --no-build-isolation -vvv --wheel-dir ./dist
done

for whl in dist/*.whl; do
    auditwheel repair "$whl" -w dist/
done

#!/bin/bash
set -ex

# Build wheels, inside a manylinux container (https://github.com/pypa/manylinux)
# see also "build-wheels-manylinux-pgo.sh"

DIR=$(dirname "$(readlink -f "$0")")

curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y --no-modify-path --component llvm-tools-preview
export PATH="$HOME/.cargo/bin:$PATH"

TARGET_TRIPLE=$(rustc -vV | awk '/^host/ {print $2}')

cd "$DIR"

export RUSTFLAGS='-C opt-level=3'
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

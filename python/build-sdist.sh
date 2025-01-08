#!/bin/bash
set -ex

## Create a symlink for sudachi.rs and resource to embed
ln -sf ../sudachi sudachi-lib
ln -sf ../resources resources

## Resolve workspace.package value in Cargo.toml
pip install tomlkit

mv Cargo.toml Cargo.sudachipy.toml
python modify_cargotoml_for_sdist.py \
    ../Cargo.toml Cargo.sudachipy.toml --out Cargo.toml

mv sudachi-lib/Cargo.toml Cargo.sudachilib.toml
python modify_cargotoml_for_sdist.py \
    ../Cargo.toml Cargo.sudachilib.toml --out sudachi-lib/Cargo.toml

## Modify to include the symlink
sed -i 's/\.\.\/sudachi/\.\/sudachi-lib/' Cargo.toml


# Build the source distribution
python -m build --sdist


# clean up changes
## Revert cargo.toml
rm Cargo.toml sudachi-lib/Cargo.toml
mv Cargo.sudachipy.toml Cargo.toml
mv Cargo.sudachilib.toml sudachi-lib/Cargo.toml

## rm files
rm LICENSE sudachi-lib resources

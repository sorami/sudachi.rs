#   Copyright (c) 2025 Works Applications Co., Ltd.
#
#   Licensed under the Apache License, Version 2.0 (the "License");
#   you may not use this file except in compliance with the License.
#   You may obtain a copy of the License at
#
#       http://www.apache.org/licenses/LICENSE-2.0
#
#    Unless required by applicable law or agreed to in writing, software
#   distributed under the License is distributed on an "AS IS" BASIS,
#   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
#   See the License for the specific language governing permissions and
#   limitations under the License.

# This script resolves workspace.package variables.
# Keep original Cargo.toml file to revert changes.
# see https://doc.rust-lang.org/cargo/reference/workspaces.html#the-package-table

import argparse
from pathlib import Path
import tomlkit


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("base", type=Path,
                        help="Cargo.toml with workspace.package")
    parser.add_argument("target", type=Path, help="Cargo.toml to modify")
    parser.add_argument("--out", type=Path, default=None,
                        help="output file")
    args = parser.parse_args()
    return args


args = parse_args()

# load Cargo.toml
with args.base.open() as fi:
    ws_toml = tomlkit.parse(fi.read())
with args.target.open() as fi:
    tgt_toml = tomlkit.parse(fi.read())

# edit workspace.package variables
keys = [k for k, v in tgt_toml["package"].items()
        if v == {"workspace": True}]
for key in keys:
    tgt_toml["package"][key] = ws_toml["workspace"]["package"][key]

# save (overwrite)
outfile = args.out if args.out is not None else args.target
with outfile.open("w") as fo:
    tomlkit.dump(tgt_toml, fo)

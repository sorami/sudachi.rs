# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

Also check [python changelog](python/CHANGELOG.md).

## [Unreleased]

## [0.6.10](https://github.com/WorksApplications/sudachi.rs/releases/tag/v0.6.10) (2025-01-10)

### Other

- Introduce cibuildwheel to build workflow (#295)

## [0.6.9](https://github.com/WorksApplications/sudachi.rs/releases/tag/v0.6.9) (2024-11-20)

### Added

- freebsd support (#222 by @KonstantinDjairo, #251)
- Add rust minimum support version (#255)
- Add option for embedded config and fallback resources (#262 by @Kuuuube)

### Changed

- `fetch_dictionary.sh` targets latest dictionary by default (#240)
- update dependencies (#241, #246)
- Migrate from structopt to clap (#248 by @tkhshtsh0917)

## [0.6.8](https://github.com/WorksApplications/sudachi.rs/releases/tag/v0.6.8) (2023-12-14)

### Highlights

- Produce builds for Python 3.12 (#236)
- Add a simple [configuration API](https://worksapplications.github.io/sudachi.rs/python/api/sudachipy.html#config-config)
- Add surface projections (#230)

### Surface Projections

- For chiTra compatibility SudachiPy can now directly produce different tokens in the surface field.
- Original surface is accessible via [`Morheme.raw_surface()`](https://worksapplications.github.io/sudachi.rs/python/api/sudachipy.html#sudachipy.Morpheme.raw_surface) method
- It is possible to customize projection dictionary-wise, via Config object, passing it on a dictionary creation, or for a single pre-tokenizer.
  - [Config API](https://worksapplications.github.io/sudachi.rs/python/api/sudachipy.html#sudachipy.config.Config.projection)
  - [Pretokenizer API](https://worksapplications.github.io/sudachi.rs/python/api/sudachipy.html#sudachipy.Dictionary.pre_tokenizer)

## [0.6.7](https://github.com/WorksApplications/sudachi.rs/releases/tag/v0.6.7) (2023-02-16)

### Highlights

- Provide binary wheels for Python 3.11
- Add `Dictionary.lookup()` method which allows you to enumerate morphemes from the dictionary without performing analysis.

## [0.6.6](https://github.com/WorksApplications/sudachi.rs/releases/tag/v0.6.6) (2022-07-25)

### Highlights

- Add [boundary matching mode](https://github.com/WorksApplications/Sudachi/blob/develop/docs/oov_handlers.md) to regex oov handler
- macOS binary builds are now unversal2 (arm+x64)

### MacOS

- Binary builds are universal2
- Caveat: we don't run tests on arm because there are no public arm instances, so builds may be broken without any warning

## [0.6.5](https://github.com/WorksApplications/sudachi.rs/releases/tag/v0.6.5) (2022-06-21)

### Highlights

- Fixed invalid POS tags which appeared when using user-defined POS tags both in user dictionaries and OOV handlers.
  You are not affected by this bug if you did not use user-defined POS in OOV handlers.

## [0.6.4](https://github.com/WorksApplications/sudachi.rs/releases/tag/v0.6.4) (2022-06-16)

### Highlights

- Remove Python 3.6 support which reached end-of-life status on [2021-12-23](https://endoflife.date/python)
- OOV handler plugins support user-defined POS, [similar to Java version](https://github.com/WorksApplications/Sudachi/releases/tag/v0.6.0)
- Added Regex OOV handler

### Regex OOV Handler

- For details, see [Java version changelog](https://github.com/WorksApplications/Sudachi/releases/tag/v0.6.0)
- In Rust/Python Regexes do not support backtracking and backreferences
- `maxLength` setting defines maximum length in unicode codepoints, not in utf-8 bytes as in Java (will be changed to codepoints later)

## [0.6.3](https://github.com/WorksApplications/sudachi.rs/releases/tag/v0.6.3) (2022-02-10)

### Highlights

- Fixed path resolution algorithm for resources. They are now resolved in the following order (first existing file wins):
  1. Absolute paths stay as they are
  2. Relative to "path" value of the config file
  3. Relative to "resource_dir" parameter of the config object during creation
     - For SudachiPy it is the parameter of `Dictionary` constructor
  4. Relative to the location of the configuration file
  5. Relative to the current directory

### Python

- `Dictionary` now has `__repr__()` function which displays absolute paths to dictionaries in use.
- `Dictionary` now has `pos_of()` function which returns a POS tuple for a given POS id.
- `PosMatcher` supports set operations
  - union (`m1 | m2`)
  - intersection (`m1 & m2`)
  - difference (`m1 - m2`)
  - negation (`~m1`)

## [0.6.2](https://github.com/WorksApplications/sudachi.rs/releases/tag/v0.6.2) (2021-12-09)

### Fixes

- Fix analysis differences with 0.5.4

## [0.6.1](https://github.com/WorksApplications/sudachi.rs/releases/tag/v0.6.1) (2021-12-08)

### Highlights

- Added Fuzzing (see `sudachi-fuzz` subdirectory), Sudachi.rs seems to be pretty robust towards arbitrary inputs (no crashes and panics)
  - Issues like https://github.com/WorksApplications/sudachi.rs/issues/182 should never occur more
- ~5% analysis speed improvement over 0.6.0
- Added support for Unicode combining symbols, now Sudachi.rs/py should be much better with emoji (🎅🏾) and more complex Unicode (İstanbul)

### Rust

- Added partial dictionary read functionality, it is now possible to skip reading certain fields if they are not needed
- Improved startup times, especially for debug builds

### Python

- See [Python changelog](./python/CHANGELOG.md)

## [0.6.0](https://github.com/WorksApplications/sudachi.rs/releases/tag/v0.6.0) (2021-11-11)

### Highlights

- Full feature parity with Java version
- ~15% analysis speed improvement over 0.6.0-rc1

### Rust

- Added dictionary build functionality
  - https://github.com/WorksApplications/sudachi.rs/pull/143
- Added an option to perform analysis without sentence splitting
  - Use it with `--split-sentences=no`

### Python

- Added bindings for dictionary build (undocumented and not supported as API).
  - See https://github.com/WorksApplications/sudachi.rs/issues/157
- `sudachipy build` and `sudachipy ubuild` should work once more
  - Report on build times and dictionary part sizes can differ from the original SudachiPy

## [0.6.0-rc1](https://github.com/WorksApplications/sudachi.rs/releases/tag/v0.6.0-rc1) (2021-10-26)

### Highlights

- First release of Sudachi.rs
- SudachiPy compatible Python bindings
- ~30x speed improvement over original SudachiPy
- Dictionary build mode will be done before 0.6.0 final (See #13)

### Rust

- Analysis: feature parity with Python and Java version
- Dictionary build is not supported in rc1
- ~2x faster than Java version (with sentence splitting)
- No public API at the moment (contact us if you want to use Rust version directly, internals will significantly change and names are not finalized)

### Python

- Mostly compatible with SudachiPy 0.5.4
- We provide binary wheels for popular platforms
- ~30x faster than 0.5.4
- IgnoreYomigana input text plugin is now supported (and enabled by default)
- We provide [binary wheels for convenience (and additional speed on Linux)](https://worksapplications.github.io/sudachi.rs/python/wheels.html)

### Known Issues

- List of deprecated SudachiPy API:
  - `MorphemeList.empty(dict: Dictionary)`
    - This also needs a dictionary as an argument.
  - `Morpheme.split(mode: SplitMode)`
  - `Morpheme.get_word_info()`
  - Most of instance attributes are not exported: e.g. `Dictionary.grammar`, `Dictionary.lexicon`.
    - See [API reference page](https://worksapplications.github.io/sudachi.rs/python/) for supported APIs.
- Dictionary Build is not supported: `sudachipy build` and `sudachipy ubuild` will not work, please use 0.5.3 in another virtual environment for the time being until the feature is implemented: #13

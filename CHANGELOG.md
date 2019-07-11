# Changelog
All notable changes to this project will be documented in this file.

## [Unreleased]
### Added
- Add datetime subtypes [#144](https://github.com/snipsco/snips-nlu-ontology/pull/144)
- Add geographical entities [#145](https://github.com/snipsco/snips-nlu-ontology/pull/145)
- Support Music entities in all languages [#147](https://github.com/snipsco/snips-nlu-ontology/pull/147)

## [0.64.8] - 2019-07-10
### Fixed
- Fix issue with libc and `Send` trait

## [0.64.7] - 2019-06-18
### Fixed
- Portuguese builtin entity examples

## [0.64.6] - 2019-04-08
### Added
- Add converters from C to Rust for ontology objects [#135](https://github.com/snipsco/snips-nlu-ontology/pull/135)

### Fixed
- Update ffi and kotlin bindings [#136](https://github.com/snipsco/snips-nlu-ontology/pull/136)

## [0.64.5] - 2019-04-02
### Fixed
- Make the json serialization of kotlin object the same as the rust one [#133](https://github.com/snipsco/snips-nlu-ontology/pull/133)

## [0.64.4] - 2019-02-28
### Fixed
- Portuguese builtin entity examples

## [0.64.3] - 2019-02-27
### Fixed
- Portuguese builtin entity examples

## [0.64.2] - 2019-02-27
### Fixed
- Portuguese builtin entities support

## [0.64.1] - 2019-02-27
### Fixed
- Portuguese builtin entity examples

## [0.64.0] - 2019-02-27
### Added
- Support for Portuguese (PT-PT and PT-BR)

### Changed
- Rename the `probability` output field to `confidence_score` in the intent
- When we serialize the slots to JSON the `confidenceScore` field is dropped if null

## [0.63.2] - 2019-02-06
### Changed
- Kotlin wrapper: update kotlin to 1.3.11

## [0.63.1] - 2019-01-29
### Fixed
- Bug in kotlin binding when converting `CSlots` into `List<Slot>`

## [0.63.0] - 2019-01-28
### Changed
- The `intent` and `slots` attributes of `IntentParserResult` are no longer optional
- The `intent_name` attribute of `IntentClassifierResult` is now optional
- The `range` attribute of `Slot` is no longer optional
- New optional `confidence_score` attribute in `Slot` object

### Removed
- Entity parser crate has been moved to the `snips-nlu-parsers` repository

## [0.62.0] - 2018-11-16
### Changed
- Update `gazetteer-parser` dependency

## [0.61.3] - 2019-01-17
### Added
- Bump rustling to `0.17.7`: fix resolution of decimal numbers

## [0.61.2] - 2018-12-14
### Added
- Bump rustling to `0.17.6`: better support for Italian and Spanish

## [0.61.1] - 2018-10-15
### Added
- Support music builtin entities in english

### Fixed
- Crash with gazetteer entity parser

## [0.61.0] - 2018-10-11
### Fixed
- Wrong enum values in ffi for `snips/musicArtist` and `snips/musicAlbum`

## [0.60.0] - 2018-10-08
### Changed
- Improve error handling in python wrapper

### Fixed
- Missing builtin entities in kotlin wrapper

## [0.59.0] - 2018-10-02
### Added
- Support for 3 new builtin entities: `snips/musicAlbum`, `snips/musicArtist` and `snips/musicTrack`
- Introduction of a `GazetterParser`, which allows to parse gazetteer entities
- API to persist and load a `BuiltinEntityParser`

### Changed
- The `BuiltinEntityParser` parsing API now returns a `Result`
- The `BuiltinEntityParser` object is now built with a `BuiltinEntityParserLoader`


## [0.58.0] - 2018-09-27
### Added
- Limited support of Italian for: AmountOfMoney, Duration, Number, Ordinal, Temperature, Time, Percentage
- Documentation for C types

## [0.57.3] - 2018-08-07
### Changed
- Bump `rustling-ontology` to `0.17.4`

## [0.57.2] - 2018-07-13

### Fixed
- Kotlin ffi

## [0.57.1] - 2018-07-05

### Fixed
- Destructor of BuiltinEntityParser python wrapper
- jna string encodings
- Crash when parsing dates with years overflowing 32 bits

### Changed
- Bump `rustling-ontology` to `0.17.2`

## [0.57.0] - 2018-06-07

### Removed
- Caching of `BuiltinEntityParser` objects

## [0.56.1] - 2018-06-05

### Added
- Builtin entity ontology export API

### Changed
- Bump `snips-nlu-utils` to `0.6.1`

### Removed
- Builtin entity caching

## [0.56.0] - 2018-05-09

### Fixed
- Fixed the null values in the Kotlin binding

## [0.55.0] - 2018-05-02

### Changed
- Updated ffi signatures

## [0.54.3] - 2018-04-20

### Changed
- Updated Rustling ontology to `0.17.0`

## [0.54.2] - 2018-04-10

### Added
- Examples for builtin entities in all languages
- Japanese support for all builtin entities

### Fixed
- Issue with the entity kinds order used in BuiltinEntityParser

## [0.54.1] - 2018-04-03

### Added
- Script to update version automatically

### Fixed
- Fixed the parsing of entities for languages where tokens are not space separated
- Japanese language label

### Changed
- Updated Rustling ontology to `0.16.4`

[Unreleased]: https://github.com/snipsco/snips-nlu-ontology/compare/0.64.8...HEAD
[0.64.8]: https://github.com/snipsco/snips-nlu-ontology/compare/0.64.7...0.64.8
[0.64.7]: https://github.com/snipsco/snips-nlu-ontology/compare/0.64.6...0.64.7
[0.64.6]: https://github.com/snipsco/snips-nlu-ontology/compare/0.64.5...0.64.6
[0.64.5]: https://github.com/snipsco/snips-nlu-ontology/compare/0.64.4...0.64.5
[0.64.4]: https://github.com/snipsco/snips-nlu-ontology/compare/0.64.3...0.64.4
[0.64.3]: https://github.com/snipsco/snips-nlu-ontology/compare/0.64.2...0.64.3
[0.64.2]: https://github.com/snipsco/snips-nlu-ontology/compare/0.64.1...0.64.2
[0.64.1]: https://github.com/snipsco/snips-nlu-ontology/compare/0.64.0...0.64.1
[0.64.0]: https://github.com/snipsco/snips-nlu-ontology/compare/0.63.2...0.64.0
[0.63.2]: https://github.com/snipsco/snips-nlu-ontology/compare/0.63.1...0.63.2
[0.63.1]: https://github.com/snipsco/snips-nlu-ontology/compare/0.63.0...0.63.1
[0.63.0]: https://github.com/snipsco/snips-nlu-ontology/compare/0.62.0...0.63.0
[0.62.0]: https://github.com/snipsco/snips-nlu-ontology/compare/0.61.3...0.62.0
[0.61.3]: https://github.com/snipsco/snips-nlu-ontology/compare/0.61.2...0.61.3
[0.61.2]: https://github.com/snipsco/snips-nlu-ontology/compare/0.61.1...0.61.2
[0.61.1]: https://github.com/snipsco/snips-nlu-ontology/compare/0.61.0...0.61.1
[0.61.0]: https://github.com/snipsco/snips-nlu-ontology/compare/0.60.0...0.61.0
[0.60.0]: https://github.com/snipsco/snips-nlu-ontology/compare/0.59.0...0.60.0
[0.59.0]: https://github.com/snipsco/snips-nlu-ontology/compare/0.58.0...0.59.0
[0.58.0]: https://github.com/snipsco/snips-nlu-ontology/compare/0.57.3...0.58.0
[0.57.3]: https://github.com/snipsco/snips-nlu-ontology/compare/0.57.2...0.57.3
[0.57.2]: https://github.com/snipsco/snips-nlu-ontology/compare/0.57.1...0.57.2
[0.57.1]: https://github.com/snipsco/snips-nlu-ontology/compare/0.57.0...0.57.1
[0.57.0]: https://github.com/snipsco/snips-nlu-ontology/compare/0.56.1...0.57.0
[0.56.1]: https://github.com/snipsco/snips-nlu-ontology/compare/0.56.0...0.56.1
[0.56.0]: https://github.com/snipsco/snips-nlu-ontology/compare/0.55.0...0.56.0
[0.55.0]: https://github.com/snipsco/snips-nlu-ontology/compare/0.54.3...0.55.0
[0.54.3]: https://github.com/snipsco/snips-nlu-ontology/compare/0.54.2...0.54.3
[0.54.2]: https://github.com/snipsco/snips-nlu-ontology/compare/0.54.1...0.54.2
[0.54.1]: https://github.com/snipsco/snips-nlu-ontology/compare/0.54.0...0.54.1

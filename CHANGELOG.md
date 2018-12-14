# Changelog
All notable changes to this project will be documented in this file.

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

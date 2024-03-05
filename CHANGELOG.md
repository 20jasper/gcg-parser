# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.1](https://github.com/20jasper/gcg-parser/compare/v0.5.0...v0.5.1) - 2024-03-05

### Fixed
- package metadata: categories ([#49](https://github.com/20jasper/gcg-parser/pull/49))

## [0.5.0](https://github.com/20jasper/gcg-parser/compare/v0.4.0...v0.5.0) - 2024-03-05

### Documentation
- Player doctest

### Refactor
- [**breaking**] split GcgError into gcg::error and token::error

### Added
- Player struct and parsing
- InvalidLine error ([#45](https://github.com/20jasper/gcg-parser/pull/45))

### Fixed
- display print instead of debug InvalidLine Error ([#46](https://github.com/20jasper/gcg-parser/pull/46))

### Remove
- [**breaking**] remove line field from MissingToken error

## [0.4.0](https://github.com/20jasper/gcg-parser/compare/v0.3.0...v0.4.0) - 2024-03-03

### Added
- [**breaking**] Add `Coordinates` struct to parse event coordinates ([#41](https://github.com/20jasper/gcg-parser/pull/41))

## [0.3.0](https://github.com/20jasper/gcg-parser/compare/v0.2.0...v0.3.0) - 2024-02-16

### Added
- [**breaking**] parse #description pragma

## [0.2.0](https://github.com/20jasper/gcg-parser/compare/v0.1.4...v0.2.0) - 2024-02-10

### Deprecated
- remove Player from public API
- change GcgError position field to token_index

### Added
- parse multiple lines. Adds error variant
- and add line_index field

### Fixed
- handle unknown pragma. Adds error variant

### Documentation
- update changelog format

## [0.1.4](https://github.com/20jasper/gcg-parser/compare/v0.1.3...v0.1.4) - 2024-02-10

### Other
- update crate level documentation
- add intradoc link to Player::build

## [0.1.3](https://github.com/20jasper/gcg-parser/compare/v0.1.2...v0.1.3) - 2024-02-09

### Added
- feat debug print text in GcgError message

### Other
- add errors heading for Player::build
- add doc test for Player::build

## [0.1.2](https://github.com/20jasper/gcg-parser/compare/v0.1.1...v0.1.2) - 2024-02-09

### Other
- update crates.io documentation link and remove duplicate keywords

## [0.1.1](https://github.com/20jasper/gcg-parser/compare/v0.1.0...v0.1.1) - 2024-02-09

### Added
- install thiserror, anyhow, and displaydoc

### Fixed
- Player struct and missing token error


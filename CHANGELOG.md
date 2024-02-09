# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1](https://github.com/20jasper/gcg-parser/compare/v0.1.0...v0.1.1) - 2024-02-09

### Added
- install thiserror, anyhow, and displaydoc

### Fixed
- Player struct and missing token error

### Other
- add changelog to prettierignore
- Remove lints denying panicing in a function returning a result and missing docs
- add test command and allow all warnings in dev and test commands
- Use PAT in release action to allow workflows to run
- fail on warnings and check for semver changes in prepublish command

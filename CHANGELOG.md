# Changelog

All notable changes to BitCards will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project intends to follow [Semantic Versioning](https://semver.org/).

## Unreleased

### Added

- Stable Rust and Cargo project structure for the BitCards generator and art lab.
- Versioned deterministic hash stream based on SHA-256.
- Strong models for Card Types, rarity, computer-themed classes, attacks, and
  configurable generation rules.
- Deterministic generation of names, gameplay statistics, attacks, supply, and
  ASCII creature artwork.
- Robot, Glitch, Daemon, Virus, Bug, and Null creature generators.
- Canonical binary serialization and cryptographic Card Type hashes.
- Fixed-width terminal card renderer with a universal card back and ANSI color.
- Display-only rarity and card-finish previews.
- CLI commands for generating individual cards and previewing card galleries.
- Fixed determinism vectors, artwork collision checks, and renderer tests.
- `.bca` draft artwork format and validation/gallery tooling.
- Architecture, Set issuance, artwork-format, and contributor documentation.

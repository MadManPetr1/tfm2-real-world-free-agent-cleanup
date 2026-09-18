# Changelog

## [0.3.0] - 2026-09-18

### Changed

- Migrated the native cleanup to the Stable Mod API required by TFM2 0.6.0.
- Replaced client-side scanning with authoritative server record enumeration.
- Preserved the conservative duplicate and verified stale-roster cleanup scope.
- Existing careers remain supported; backing up before first use is still recommended.

All notable public changes to Real World Free Agent Cleanup will be documented
here.

The project uses [Semantic Versioning](https://semver.org/).

## [Unreleased]

## [0.2.4] - 2026-09-02

### Changed

- Rebuilt the native DLL against the Teamfight Manager 2 `0.5.8` Mod SDK.
- Updated the declared base range to `>=0.5.8, <0.5.9`.

### Compatibility

- Tested on Teamfight Manager 2 `0.5.8`; conservative cleanup rules and
  existing-career support are unchanged.

## [0.2.3] - 2026-08-26

### Changed

- Rebuilt the native DLL against the Teamfight Manager 2 `0.5.7` Mod SDK.
- Restricted the declared base range to `>=0.5.7, <0.5.8` because the native
  SDK libraries changed and older game versions have not been runtime-tested
  with this build.

### Compatibility

- Tested on Teamfight Manager 2 `0.5.7`; conservative cleanup rules and
  existing-career support are unchanged.

## [0.2.2] - 2026-08-20

### Changed

- Rebuilt the native DLL against the Teamfight Manager 2 `0.5.6` Mod SDK.
- Restricted the declared base range to `>=0.5.6, <0.5.7` because the native
  SDK libraries changed and older game versions have not been runtime-tested
  with this build.

### Compatibility

- Tested on Teamfight Manager 2 `0.5.6`; conservative cleanup rules and
  existing-career support are unchanged.

## [0.2.1] - 2026-08-12

### Changed

- Rebuilt the native DLL against the Teamfight Manager 2 `0.5.5` Mod SDK.
- Restricted the declared base range to `>=0.5.5, <0.5.6` because the native
  SDK libraries changed and older game versions have not been runtime-tested
  with this build.

### Compatibility

- Tested on Teamfight Manager 2 `0.5.5`; conservative cleanup rules and
  existing-career support are unchanged.

## [0.2.0] - 2026-08-05

### Changed

- Standardized the Cargo crate, mod ID, installed folder, and DLL as
  `tfm2_real_world_free_agent_cleanup`.
- Preserved the existing conservative and repeat-safe cleanup behavior.

## [0.1.8] - 2026-08-05

### Changed

- Rebuilt the native DLL against the Teamfight Manager 2 `0.5.4` Mod SDK.
- Extended the supported base range to `>=0.5.2, <0.5.5` without changing
  conservative cleanup rules or existing-career behavior.

### Compatibility

- Runtime-tested on Teamfight Manager 2 `0.5.4`.

## [0.1.7] - 2026-08-03

### Added

- Unified Better Mod Menu author profile and profile icon.
- Refreshed 256 px pixel-art thumbnail.

### Changed

- Simplified player documentation and release presentation.
- Cleaned source and release packaging without changing cleanup rules.

### Compatibility

- Cleanup behavior and the tested TFM2 `0.5.2`-`0.5.3` range are unchanged.

## [0.1.6] - 2026-07-29

### Changed

- Rebuilt the native DLL against the Teamfight Manager 2 `0.5.2` Mod SDK as
  the compatibility baseline.
- Expanded the supported game range to `>=0.5.2, <0.5.4`.
- Added an explicit `real_world_free_agent_cleanup` package identity so manual
  and Workshop installations resolve the same mod ID.

### Compatibility

- Verified that the exact `0.5.2`-baseline DLL loads, registers, and reaches
  the rendered title screen on Teamfight Manager 2 `0.5.3`.

## [0.1.5] - 2026-07-29

### Changed

- Rebuilt the native mod against the Teamfight Manager 2 `0.5.3` Mod SDK.
- Updated the supported game range to `>=0.5.3, <0.5.4`.
- Updated the local build wrapper to link the SDK's LLVM bitcode with its
  pinned Rust LLVM linker.

### Performance

- Reduced steady-state client snapshot scans from roughly once per second to
  once every five seconds.
- Normalized player and team names without allocating a temporary string
  vector.

### Safety

- Excluded retired contracted records from client-side verified transition
  matching, aligning it with the authoritative server cleanup.

## [0.1.4] - 2026-07-28

### Changed

- Replaced the initial Workshop thumbnail with the creator-provided pixel-art
  revision.

## [0.1.3] - 2026-07-28

### Changed

- Added complete public documentation, contribution guidance, security
  reporting, release presentation, and packaging validation.
- Identified Real World Database '26 by Crown as the primary compatibility
  target and credited the original Workshop pack.
- Added original Workshop thumbnail artwork matching the shared mod visual
  style.
- Relicensed new project versions under MPL-2.0 so distributed changes to
  covered files remain shareable.
- Clarified that MPL-2.0 covers the source project but does not grant
  trademark or original artwork rights.

## [0.1.2] - 2026-07-28

### Added

- Narrow, team-pair-verified corrections for Zyko's stale DarkZero Dragonsteel
  record and ZekaS's stale Vivo Keyd Stars Academy record.
- Tests proving a stale contracted record is removed only when its reviewed
  canonical team record is also present.

### Safety

- Contracted athletes are never deduplicated globally by name.

## [0.1.1] - 2026-07-28

### Added

- Client-side removal of confirmed duplicates so third-party scouting panels
  do not continue displaying retired records.

## [0.1.0] - 2026-07-28

### Added

- Exact normalized-name detection for generated free agents that conflict with
  active contracted players.
- Authoritative retirement through the game server extension.
- Per-save cleanup count and name records in the mod save namespace.

[Unreleased]: https://github.com/MadManPetr1/tfm2-real-world-free-agent-cleanup/compare/v0.2.3...HEAD
[0.2.3]: https://github.com/MadManPetr1/tfm2-real-world-free-agent-cleanup/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/MadManPetr1/tfm2-real-world-free-agent-cleanup/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/MadManPetr1/tfm2-real-world-free-agent-cleanup/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/MadManPetr1/tfm2-real-world-free-agent-cleanup/compare/v0.1.8...v0.2.0
[0.1.8]: https://github.com/MadManPetr1/tfm2-real-world-free-agent-cleanup/compare/v0.1.7...v0.1.8
[0.1.7]: https://github.com/MadManPetr1/tfm2-real-world-free-agent-cleanup/compare/v0.1.6...v0.1.7
[0.1.6]: https://github.com/MadManPetr1/tfm2-real-world-free-agent-cleanup/compare/v0.1.5...v0.1.6
[0.1.5]: https://github.com/MadManPetr1/tfm2-real-world-free-agent-cleanup/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/MadManPetr1/tfm2-real-world-free-agent-cleanup/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/MadManPetr1/tfm2-real-world-free-agent-cleanup/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/MadManPetr1/tfm2-real-world-free-agent-cleanup/tree/v0.1.2

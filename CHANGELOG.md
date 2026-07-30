# Changelog

All notable public changes to Real World Free Agent Cleanup will be documented
here.

The project uses [Semantic Versioning](https://semver.org/).

## [Unreleased]

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

[Unreleased]: https://github.com/MadManPetr1/tfm2-real-world-free-agent-cleanup/compare/v0.1.6...HEAD
[0.1.6]: https://github.com/MadManPetr1/tfm2-real-world-free-agent-cleanup/compare/v0.1.5...v0.1.6
[0.1.5]: https://github.com/MadManPetr1/tfm2-real-world-free-agent-cleanup/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/MadManPetr1/tfm2-real-world-free-agent-cleanup/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/MadManPetr1/tfm2-real-world-free-agent-cleanup/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/MadManPetr1/tfm2-real-world-free-agent-cleanup/tree/v0.1.2

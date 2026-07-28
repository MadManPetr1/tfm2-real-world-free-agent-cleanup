# Real World Free Agent Cleanup

Conservative duplicate-player cleanup for **Teamfight Manager 2** careers
created from real-world database packs.

[Rules](#cleanup-rules) · [Installation](#installation) · [Save safety](#save-safety) · [Building](#building-from-source)

> [!IMPORTANT]
> Version **0.1.2** is built for Teamfight Manager 2 **0.5.2**.

## What it fixes

Real-world database imports can combine current rosters, historical roster
entries, academy registrations, and the game's generated free-agent pool. That
can produce two athlete records for one person.

This mod removes only cases it can identify conservatively:

- an active free agent whose normalized name exactly matches an active
  contracted player;
- a small list of independently verified roster-transition duplicates where
  both the old and current team records are simultaneously present.

## Cleanup rules

### Generated free-agent collisions

When an active free agent has the same normalized name as an active contracted
player, the contracted player is preserved and the free-agent copy is retired.

Normalization ignores letter case and repeated surrounding whitespace. It does
not use fuzzy matching.

### Verified roster transitions

Version 0.1.2 includes these narrow corrections:

| Player | Preserve | Remove stale record |
| --- | --- | --- |
| Zyko | Supernova | DarkZero Dragonsteel |
| ZekaS | Vivo Keyd Stars | Vivo Keyd Stars Academy |

A correction runs only when both the preserved and stale team records are
present. The mod does not globally delete contracted players merely because
they share a handle.

## How it works

- The server-side extension marks a confirmed duplicate athlete as retired,
  using the game's supported athlete lifecycle instead of deleting database
  table entries.
- The client-side extension removes the same duplicate from the local athlete
  snapshot so scouting panels do not continue displaying the retired record.
- The operation is repeat-safe: already retired records are not retired again.
- The original `.tfm2db` database pack is never edited.

## Installation

### Steam Workshop

Subscribe on the Teamfight Manager 2 Workshop, enable
**Real World Free Agent Cleanup** in the in-game Mods menu, then restart the
game when prompted.

### Manual GitHub release

1. Download `real-world-free-agent-cleanup-vX.Y.Z.zip` from this repository's
   Releases page. Do not download GitHub's automatic “Source code” archive.
2. Extract the included `real_world_free_agent_cleanup` folder into:

   ```text
   ...\SteamLibrary\steamapps\common\Teamfight Manager2\mods\
   ```

3. Confirm this structure:

   ```text
   Teamfight Manager2\mods\real_world_free_agent_cleanup\mod.mod_info
   Teamfight Manager2\mods\real_world_free_agent_cleanup\real_world_free_agent_cleanup.dll
   ```

4. Enable the mod and restart the game.

## Save safety

> [!WARNING]
> This mod intentionally changes athlete state in the loaded career. Back up
> the latest `save_*.data` file before first use.

The mod does not delete save files, overwrite the imported `.tfm2db`, or apply
fuzzy identity matching. Disable it and restore the pre-cleanup save if a
database-specific correction is not appropriate for your career.

Career saves are normally stored in:

```text
%APPDATA%\TeamSamoyed\TeamfightManager2\data
```

## Requirements and limitations

- Teamfight Manager 2 `0.5.2`
- The matching `0.5.2` Mod SDK for source builds
- Designed for imported real-world database careers
- Exact-name free-agent matching only
- Contracted-player corrections require an explicitly reviewed team transition

This is not a general identity-merging engine. Different real players can
share a handle, so ambiguous contracted duplicates are deliberately left
untouched.

## Building from source

The Mod SDK is not redistributed here. Install the matching SDK with the game,
then run:

```powershell
.\build_local.ps1 -SdkDir "C:\path\to\Teamfight Manager2\mod-sdk"
```

To validate and create a player-ready archive:

```powershell
.\scripts\validate_repo.ps1
.\scripts\package_release.ps1 -SdkDir "C:\path\to\Teamfight Manager2\mod-sdk"
```

## Project layout

- `src/lib.rs` — authoritative cleanup and client-side scouting compatibility
- `mod.mod_info` — mod metadata and supported game range
- `build_local.ps1` — SDK-aware native build
- `scripts/` — repository validation and release packaging
- `docs/PRESENTATION.md` — ready-to-use public listing copy

## Reporting another duplicate

Open an issue with:

- the exact handle and role;
- both displayed teams or the free-agent state;
- a screenshot with private save information removed;
- reliable roster-history evidence if both records are contracted;
- the database pack name/version and game version.

Do not upload career saves publicly.

## License and attribution

The original source code, scripts, and documentation are released under the
[Mozilla Public License 2.0](LICENSE). Distributed changes to covered files
must remain available under MPL-2.0.

MPL-2.0 does not grant trademark rights in the project name. Any original
artwork added to an official release will carry its own asset notice; see
[NOTICE](NOTICE.md).

This project does not redistribute a real-world database pack and is not
affiliated with or endorsed by Team Samoyed, Riot Games, tournament operators,
teams, or players.

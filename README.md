# Real World Free Agent Cleanup

An unofficial companion cleanup made primarily for
**[Real World Database '26](https://steamcommunity.com/sharedfiles/filedetails/?id=3733195966)**
by Crown.

[Rules](#cleanup-rules) · [Installation](#installation) · [Save safety](#save-safety) · [Building](#building-from-source)

> [!IMPORTANT]
> Version **0.1.4** is built for Teamfight Manager 2 **0.5.2**.

This mod exists because Real World Database '26 imports real players and
rosters while Teamfight Manager 2 can still supply overlapping generated free
agents and older roster records. It removes those conflicts without modifying
or redistributing Crown's database pack.

## What it fixes

The main target is Real World Database '26. Its imported current rosters,
historical entries, and academy registrations can overlap with the game's
built-in generated free-agent pool or with another roster entry for the same
person.

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

[Subscribe to Real World Free Agent Cleanup](https://steamcommunity.com/sharedfiles/filedetails/?id=3773383684),
enable it in the in-game Mods menu, then restart the game when prompted.

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
- Primarily designed for Real World Database '26 v1.1.0 by Crown
- May also help compatible imported real-world database careers
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
- `thumbnail.png` — 512×512 in-game and Workshop thumbnail
- `assets/thumbnail-master.png` — original high-resolution artwork
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

This is an unofficial companion project. It does not redistribute Real World
Database '26 and is not affiliated with or endorsed by Crown, Team Samoyed,
Riot Games, tournament operators, teams, or players.

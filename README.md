<div align="center">

<img src="thumbnail.png" alt="Real World Free Agent Cleanup icon" width="128">

# Real World Free Agent Cleanup

A conservative companion cleanup for
**[Real World Database '26](https://steamcommunity.com/sharedfiles/filedetails/?id=3733195966)**
by Crown.

**RRFAC 0.2.3 · TFM2 0.5.7**

</div>

Real-world database imports can overlap with generated free agents or leave an
older roster record beside the current one. RRFAC retires only conflicts it can
identify safely and does not modify or redistribute Crown's database.

## What it fixes

- Active generated free agents whose normalized name exactly matches an active
  contracted player.
- A small, reviewed list of roster transitions where both the stale and current
  records appear together.

Matching is deliberately narrow: case and surrounding whitespace are ignored,
but fuzzy name matching is never used.

### Reviewed roster transitions

| Player | Keep | Retire stale record |
| --- | --- | --- |
| Zyko | Supernova | DarkZero Dragonsteel |
| ZekaS | Vivo Keyd Stars | Vivo Keyd Stars Academy |

Each correction runs only when both exact records are present. The operation is
repeat-safe and never edits the original `.tfm2db` file.

## Install

### Steam Workshop

[Subscribe to RRFAC](https://steamcommunity.com/sharedfiles/filedetails/?id=3773383684),
enable it after importing Real World Database '26, and restart when prompted.

### GitHub release

1. Download `tfm2-real-world-free-agent-cleanup-v0.2.3.zip` from
   [GitHub Releases](https://github.com/MadManPetr1/tfm2-real-world-free-agent-cleanup/releases).
   Do not use GitHub's automatic source-code archive.
2. Extract `tfm2_real_world_free_agent_cleanup` into:

   ```text
   ...\SteamLibrary\steamapps\common\Teamfight Manager2\mods\
   ```

3. Enable the cleanup mod and restart the game.

## Save safety

> [!WARNING]
> RRFAC intentionally changes athlete state in the loaded career. Back up the
> latest `save_*.data` file before first use.

The mod does not delete save files, overwrite the imported database, or remove
ambiguous contracted players. If a correction is unsuitable for a career,
disable the mod and restore the pre-cleanup save.

## Compatibility

- Teamfight Manager 2 `0.5.7`
- Primarily designed for Real World Database '26 v1.1.0
- Release DLL built against the `0.5.7` Mod SDK
- Existing careers are supported, with a backup recommended before first use

## Reporting another duplicate

Open a data-correction request with the handle, role, both displayed teams or
free-agent state, database/game versions, and reliable roster-history evidence.
Matching names alone are not enough. Do not upload career saves publicly.

## Build

```powershell
.\build_local.ps1 -SdkDir "C:\path\to\Teamfight Manager2\mod-sdk-0.5.7"
.\scripts\validate_repo.ps1
.\scripts\package_release.ps1 -SdkDir "C:\path\to\Teamfight Manager2\mod-sdk-0.5.7"
```

## License

Source code, scripts, and documentation are licensed under the
[Mozilla Public License 2.0](LICENSE). Project branding and original artwork
are not covered by MPL-2.0; see [NOTICE.md](NOTICE.md).

This unofficial companion project is not affiliated with or endorsed by Crown,
Team Samoyed, Riot Games, tournament operators, teams, or players.

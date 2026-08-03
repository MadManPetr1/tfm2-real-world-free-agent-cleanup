# Public presentation copy

## GitHub repository

**Repository**

`tfm2-real-world-free-agent-cleanup`

**Website**

https://steamcommunity.com/sharedfiles/filedetails/?id=3773383684

**Description**

Unofficial companion cleanup for Crown's Real World Database '26, removing
generated free-agent collisions and narrowly verified stale roster records.

**Topics**

`teamfight-manager-2`, `teamfight-manager2`, `mod`, `rust`, `database`,
`free-agents`, `roster-management`, `steam-workshop`

## Steam Workshop

**Title**

Real World Free Agent Cleanup

**Short description**

Clean up generated free-agent collisions and stale roster-transition duplicates
in careers using Crown's Real World Database '26.

**Full description**

Real World Free Agent Cleanup was made primarily as an unofficial companion for
Crown's Real World Database '26.

The database pack imports real players and rosters, but Teamfight Manager 2 can
still supply overlapping generated free agents or older roster entries. This
mod removes those conflicts without changing or redistributing the database
pack.

CONSERVATIVE RULES

• Preserve an active contracted player over an exact-name generated free agent  
• Correct only explicitly reviewed contracted roster transitions  
• Never globally delete contracted players by handle  
• Never modify the original database pack  
• Remove confirmed retired duplicates from scouting panels

VERIFIED TRANSITIONS IN 0.1.2

• Preserve Zyko on Supernova; remove the stale DarkZero Dragonsteel record  
• Preserve ZekaS on Vivo Keyd Stars; remove the stale academy record

SAVE SAFETY

The mod changes athlete state in the loaded career. Back up the latest save
before first use. Disable the mod and restore that backup if a correction is
not appropriate for your database.

COMPATIBILITY

Version 0.1.7 supports Teamfight Manager 2 0.5.2 and 0.5.3.

SOURCE AND CONTRIBUTIONS

Source code and documentation are available under MPL-2.0. New contracted
corrections require evidence that both records are the same real person and
that one exact team entry is stale.

Real World Database '26 is created by Crown:
https://steamcommunity.com/sharedfiles/filedetails/?id=3733195966

This unofficial companion mod does not include the database pack and is not
affiliated with or endorsed by Crown, Team Samoyed, Riot Games, teams, or
players.

## Screenshot plan

Use real in-game captures during the later shared visual pass:

1. Before: contracted player beside the duplicate free agent.
2. After: the duplicate row removed.
3. Verified transition example showing the two stale/current team records.
4. Clean scouting table after correction.

Blur private save names or paths.

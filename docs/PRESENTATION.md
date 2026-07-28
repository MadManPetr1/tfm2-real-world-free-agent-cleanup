# Public presentation copy

## GitHub repository

**Repository**

`tfm2-real-world-free-agent-cleanup`

**Description**

Conservative duplicate-player cleanup for Teamfight Manager 2 careers created
from real-world database packs.

**Topics**

`teamfight-manager-2`, `teamfight-manager2`, `mod`, `rust`, `database`,
`free-agents`, `roster-management`, `steam-workshop`

## Steam Workshop

**Title**

Real World Free Agent Cleanup

**Short description**

Remove exact generated free-agent collisions and narrowly verified stale
roster-transition duplicates from imported real-world database careers.

**Full description**

Real World Free Agent Cleanup addresses duplicate athlete records created when
real-world roster data, historical team entries, academy registrations, and
the game's generated free agents overlap.

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

Version 0.1.2 is built for Teamfight Manager 2 0.5.2.

SOURCE AND CONTRIBUTIONS

Source code and documentation are available under MPL-2.0. New contracted
corrections require evidence that both records are the same real person and
that one exact team entry is stale.

This mod does not include a real-world database pack and is not affiliated
with or endorsed by Team Samoyed, Riot Games, teams, or players.

## Screenshot plan

Use real in-game captures during the later shared visual pass:

1. Before: contracted player beside the duplicate free agent.
2. After: the duplicate row removed.
3. Verified transition example showing the two stale/current team records.
4. Clean scouting table after correction.

Blur private save names or paths.

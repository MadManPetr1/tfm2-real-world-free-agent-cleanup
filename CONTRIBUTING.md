# Contributing

Bug reports, database compatibility findings, documentation improvements, and
focused pull requests are welcome.

## Contribution terms

By submitting a contribution, you agree to license it under the
[Mozilla Public License 2.0](LICENSE). Submit only work you created or have the
right to contribute.

MPL-2.0 does not grant trademark rights in the project name. Forks should use
their own name and artwork.

## Reporting a duplicate

Free-agent collisions need the handle, role, contracted team, database pack,
and game version.

Contracted-player corrections have a higher evidence bar. Include:

1. evidence that both records represent the same real person;
2. the previous and current team;
3. dated roster-history sources;
4. an explanation of which record should be preserved;
5. confirmation that both exact team records appear simultaneously in-game.

Name equality alone is not enough because different players can share a
competitive handle.

## Pull requests

- Keep cleanup rules narrow, explicit, and repeat-safe.
- Never add fuzzy contracted-player deletion.
- Preserve the server-authoritative/client-display split.
- Run `cargo fmt --check`.
- Run the unit tests with the matching Mod SDK.
- Run `.\scripts\validate_repo.ps1` on Windows.
- Update `CHANGELOG.md` under **Unreleased**.
- Explain save-data impact and rollback behavior.

The native mod depends on the matching Teamfight Manager 2 Mod SDK, which is
not redistributed in this repository.

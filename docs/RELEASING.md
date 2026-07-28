# Release guide

## 1. Verify

- Test on the declared Teamfight Manager 2 version.
- Test a new career and an existing affected career.
- Confirm only exact normalized-name duplicates are removed from free agency.
- Confirm every contracted-player correction is still listed explicitly.
- Back up test saves before runtime verification.
- Run `.\scripts\validate_repo.ps1`.

Do not broaden the version range in `mod.mod_info` until that game version has
been tested.

## 2. Package

```powershell
.\scripts\package_release.ps1 -SdkDir "C:\path\to\Teamfight Manager2\mod-sdk"
```

Inspect `builds\real-world-free-agent-cleanup-vX.Y.Z.zip`. Its top-level folder
must be `real_world_free_agent_cleanup`, and that folder must contain
`real_world_free_agent_cleanup.dll`.

## 3. Publish

- Use `docs/PRESENTATION.md` for the repository description, release copy, and
  Workshop copy.
- Tag the exact source used to build the archive.
- Attach the matching archive to the GitHub release.
- Test the packaged build against backed-up saves before publishing.
- Keep the Workshop and GitHub version numbers aligned.

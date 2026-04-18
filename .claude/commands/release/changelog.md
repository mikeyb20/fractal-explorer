# /changelog — Generate Changelog

## Instructions

Generate a changelog entry from git history. Follow the Keep a Changelog
format (https://keepachangelog.com/). Be concise — users care about what
changed and why, not implementation details.

## Step 1: Gather History

```bash
LAST_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "")
if [ -n "$LAST_TAG" ]; then
  git log $LAST_TAG..HEAD --oneline --no-merges
else
  git log --oneline --no-merges
fi
```

Also check for plan files in `docs/plans/` (and `docs/plans/archive/`)
that correspond to this release for higher-level context on what features
were added.

## Step 2: Categorize

Group commits into:
- **Added:** new features or capabilities
- **Changed:** modifications to existing functionality
- **Fixed:** bug fixes
- **Removed:** removed features or deprecated items
- **Security:** security-related changes
- **Deprecated:** features that will be removed in a future release

Ignore:
- Merge commits
- WIP commits (these are intermediate, the final commit matters)
- Formatting-only changes
- Internal refactors with no user-visible impact (unless significant)

## Step 3: Write Entry

```markdown
## [<version>] - <YYYY-MM-DD>

### Added
- <user-visible description of what was added>

### Changed
- <user-visible description of what changed>

### Fixed
- <user-visible description of what was fixed>
```

Write descriptions from the USER's perspective, not the developer's.
"Added pathfinding for ground units" not "Implemented A* algorithm in
pathfinding_system.cpp with ground/aerial discriminator".

## Step 4: Update File

Prepend the new entry to CHANGELOG.md (create if it doesn't exist).
Preserve all existing entries.

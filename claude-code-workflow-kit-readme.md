# Claude Code Workflow Kit

A complete, opinionated development workflow for Claude Code. 19 reusable
skills that cover planning, implementation, validation, integration, release,
and documentation — designed for solo developers working across C++, Rust,
Go, and React/TypeScript projects.

---

## What's In the Box

This kit has three layers, each building on the last:

| Document | Purpose | When to Read |
|----------|---------|-------------|
| `WORKFLOW.md` | Philosophy and process — how to think about development with Claude Code | Once, before adopting. Reference when deciding approach for a specific task. |
| `SKILL-MAP.md` | Skill specifications — what each skill does, when it fires, what it produces | When customizing skills or understanding what a specific command does. |
| `README.md` | Operations — how to install, adopt, maintain, and customize | During setup and when onboarding a new project. |

**The skills themselves** live in `.claude/commands/` as markdown files.
They're the only artifacts Claude Code directly executes. Everything else
is reference documentation for you.

---

## Prerequisites

- Claude Code installed and authenticated
- Git repository initialized
- A project with a build and test command (even if the test suite is empty)
- Familiarity with Claude Code basics: `/clear`, Plan Mode (`Shift+Tab` ×2),
  `/model`, `--worktree` flag

**Choose your setup path:**

| Your Situation | Section to Follow |
|----------------|-------------------|
| Clean project, no existing `.claude/` setup | Quick Start (Existing Repo) |
| Project with existing `.claude/`, commands, or skills | Migrating an Existing Project |
| Brand-new project, no code yet | Setting Up a New Repo From Scratch |

All three paths converge at "Installing Skills" → "Post-Installation Calibration."

---

## Quick Start (Existing Repo)

The fastest way to set up is to run the bootstrap prompt in Claude Code.
Open Claude Code in your project root and paste the following:

```
I want you to set up the Claude Code Workflow Kit in this project. Here's what I need:

1. ANALYZE this project:
   - Detect the primary language(s) and build system
   - Find existing build, test, lint, and run commands
   - Note the project structure and key modules
   - Check for existing CLAUDE.md, .claude/ directory, or docs/

2. CREATE the directory structure:
   .claude/
   ├── commands/
   │   ├── planning/
   │   ├── session/
   │   ├── build/
   │   ├── validate/
   │   ├── test/
   │   ├── integrate/
   │   ├── release/
   │   └── docs/
   docs/
   ├── plans/
   │   └── archive/
   └── decisions/
   scripts/

3. CREATE or UPDATE CLAUDE.md (keep under 60 lines) using this skeleton:
   # <Project Name>
   
   ## Architecture
   <One paragraph based on what you found — project purpose, core patterns, key modules>
   
   ## Stack
   - Language: <detected>
   - Build: <detected command>
   - Test: <detected command>
   - Lint: <detected command or "not configured">
   - Run: <detected command or "not applicable">
   
   ## Conventions
   - <Infer from existing code: naming patterns, error handling, module structure>
   
   ## Gotchas
   - <Leave empty for now — will be populated as we discover Claude's mistakes>
   
   ## Key Docs
   - Workflow: see docs/WORKFLOW.md
   - Plans: see docs/plans/

4. CREATE scratchpad.md at project root:
   # Scratchpad
   
   ## Current State
   - Working on: —
   - Branch: main
   - Last completed step: —
   - Next step: —
   
   ## Decisions Made During Implementation
   
   ## Open Questions
   
   ## Known Issues

5. CREATE scripts/validate-turn.sh adapted to THIS project's build system:
   #!/bin/bash
   # Auto-validation hook: runs after each Claude turn
   # Adapted for <detected build system>
   
   <project-specific build command> || { echo "BUILD FAILED"; exit 1; }
   <project-specific test command> || { echo "TESTS FAILED"; exit 1; }
   echo "Build and tests passing."
   
   Then chmod +x scripts/validate-turn.sh

6. CREATE scripts/check-scope.sh:
   #!/bin/bash
   # Scope enforcement hook for parallel workstreams
   # Reads ALLOWED_FILES from .claude/scope.txt
   # Rejects edits to files not in the allow list
   
   FILE_PATH="$1"
   SCOPE_FILE=".claude/scope.txt"
   
   if [ ! -f "$SCOPE_FILE" ]; then
     exit 0  # No scope file = no restrictions
   fi
   
   if ! grep -qxF "$FILE_PATH" "$SCOPE_FILE"; then
     echo "SCOPE VIOLATION: $FILE_PATH is not in the allowed scope"
     echo "Allowed files are listed in $SCOPE_FILE"
     exit 1
   fi
   
   Then chmod +x scripts/check-scope.sh

7. UPDATE .gitignore to include:
   # Claude Code workflow
   scratchpad*.md
   .claude/worktrees/
   .claude/scope.txt

8. CREATE .claude/settings.json with hooks (if it doesn't exist, create it;
   if it does, merge these hooks into the existing configuration):
   {
     "hooks": {
       "Stop": [
         {
           "command": "scripts/validate-turn.sh"
         }
       ]
     }
   }
   
   NOTE: Do NOT add the PostToolUse formatter hook or PreToolUse scope hook
   by default. Those are opt-in — the README explains when to enable them.

9. REPORT what you created and any project-specific notes:
   - What language/build system was detected
   - What commands are configured for build/test
   - Any existing configuration that was preserved
   - Recommendations for project-specific CLAUDE.md additions

Do NOT create the actual skill command files yet — those will be installed
separately. Just set up the infrastructure.
```

This gives you the directory structure, project-aware CLAUDE.md, hooks, and
support scripts. Next, place the reference documents and install the skills.

### Place Reference Documents

Copy the two reference docs into your repo's `docs/` directory:

- `docs/WORKFLOW.md` — the workflow philosophy document (claude-code-workflow-v2.md)
- `docs/SKILL-MAP.md` — the skill reference and specifications (claude-code-skill-map-v3.md)

These are committed to the repo so any session can reference them. The skills
themselves read the plan and scratchpad, not these reference docs — but having
them in-repo means you (and future collaborators) can always look up how the
workflow operates and what each skill does.

```bash
cp /path/to/claude-code-workflow-v2.md docs/WORKFLOW.md
cp /path/to/claude-code-skill-map-v3.md docs/SKILL-MAP.md
git add docs/WORKFLOW.md docs/SKILL-MAP.md
git commit -m "docs: add workflow and skill map reference documents"
```

---

## Migrating an Existing Project

If your project already has a `.claude/` directory, existing commands/skills,
CLAUDE.md, or documentation in `docs/`, use this migration path instead of
the clean-install Quick Start. The migration preserves valuable custom work
while establishing the workflow kit as the authoritative structure.

### Migration Principles

1. **This workflow takes priority over existing skills with similar names.**
   If you have an existing `/plan` or `/review` command, the workflow kit
   version replaces it. The workflow skills are designed as an integrated
   system — mixing in ad-hoc skills with the same names breaks the chain.

2. **Existing custom skills that DON'T overlap are preserved.** If you have
   a `/deploy` or `/db-migrate` command that the workflow kit doesn't include,
   it stays. It just moves into the new directory structure.

3. **Existing documentation is preserved and integrated.** Architecture docs,
   design docs, and READMEs stay where they are. The workflow kit adds
   structure around them, not in place of them.

4. **CLAUDE.md is merged, not replaced.** Your existing project-specific
   Gotchas, conventions, and architecture description are high-value.
   The migration adds the workflow kit's structural requirements while
   keeping your content.

### Migration Prompt

Run this in Claude Code at your project root:

```
I want to migrate this existing project to the Claude Code Workflow Kit.
This project already has some Claude Code configuration. Here's what I need:

## PHASE 1: AUDIT EXISTING STATE

Before changing anything, analyze and report:

1. CLAUDE.md:
   - Does it exist? What does it contain?
   - How many lines is it? (target: under 60)
   - Does it have build/test/lint commands?

2. .claude/ directory:
   - List all existing files in .claude/ (commands, settings, etc.)
   - For each existing command/skill file:
     a. What is its name?
     b. What does it do? (one sentence)
     c. Does it overlap with any workflow kit skill? (the 19 skills are:
        plan, plan-review, init, handoff, cleanup, implement, build-interfaces,
        review, done-check, sweep, adversarial, fresh-eyes, gen-tests,
        predict-conflicts, integrate-review, pre-release, changelog,
        catalog-debt, docs, doc-decision)
     d. Mark as: REPLACE (overlaps with kit skill), KEEP (unique custom skill),
        or MERGE (partially overlaps, needs manual review)

3. .claude/settings.json:
   - Does it exist? What hooks are configured?
   - Any custom settings that need to be preserved?

4. docs/ directory:
   - What documentation already exists?
   - Is there an architecture doc? Where is it?
   - Are there existing plan files, ADRs, or design docs?

5. Scratchpad / state files:
   - Any existing scratchpad.md, notes.md, or similar state files?

6. scripts/ directory:
   - Any existing build/test/validation scripts?

7. .gitignore:
   - What Claude-related entries already exist?

REPORT all findings before proceeding. Do NOT make changes yet.
Wait for my approval on the migration plan.

## PHASE 2: MIGRATION PLAN

Based on the audit, create a migration plan:

### Skills to REPLACE (existing skill → workflow kit skill)
For each: name the existing file, name the replacement, note what's lost (if anything)

### Skills to KEEP (move to new directory structure)
For each: current path → new path under .claude/commands/

### Skills to MERGE (needs manual attention)
For each: what the existing skill does that the kit skill doesn't, and recommendation

### CLAUDE.md changes
- What to keep from existing CLAUDE.md
- What to add from the workflow kit skeleton
- What to remove (over 60 lines? redundant with defaults?)

### Hook changes
- What existing hooks to preserve
- What new hooks to add (Stop validation hook)
- Any conflicts

### Documentation structure changes
- Existing docs to keep in place
- New directories to create (docs/plans/, docs/decisions/, etc.)
- Where WORKFLOW.md and SKILL-MAP.md will go

### .gitignore additions
- New entries needed for scratchpad*.md, .claude/worktrees/, .claude/scope.txt

Present this plan and wait for approval before executing.

## PHASE 3: EXECUTE MIGRATION (only after approval)

1. Create directory structure:
   .claude/commands/{planning,session,build,validate,test,integrate,release,docs}/
   docs/plans/archive/
   docs/decisions/
   scripts/

2. Handle existing skills:
   - REPLACE skills: back up to .claude/commands/_archive/<original-name>.md
     then note they've been replaced. Do NOT delete — archive for reference.
   - KEEP skills: move to the appropriate category directory under .claude/commands/
     If no category fits, create .claude/commands/custom/
   - MERGE skills: create the kit version, archive the original, and add a
     comment at the top of the kit version noting what custom functionality
     from the original should be reviewed for integration

3. Update CLAUDE.md:
   - Preserve existing project-specific content (architecture, gotchas, conventions)
   - Restructure to match the workflow kit skeleton if needed
   - Add Key Docs section pointing to docs/WORKFLOW.md, docs/plans/
   - Trim to under 60 lines — move verbose content to dedicated doc files
   - Add build/test/lint/run commands if missing

4. Update .claude/settings.json:
   - Preserve existing hooks
   - Add Stop hook for scripts/validate-turn.sh (if no equivalent exists)
   - If an existing Stop hook does something similar, merge the functionality

5. Create scripts:
   - scripts/validate-turn.sh — adapted to this project's build system
   - scripts/check-scope.sh — scope enforcement for parallel work
   - chmod +x both

6. Create scratchpad.md with blank template

7. Update .gitignore with workflow kit entries

8. Commit the migration:
   git add -A
   git commit -m "chore: migrate to Claude Code Workflow Kit

   Skills replaced: <list>
   Skills preserved: <list>
   Skills archived: <list>
   New infrastructure: hooks, scripts, directory structure"

9. REPORT:
   - What was migrated
   - What was archived (and where)
   - What needs manual review (MERGE items)
   - What the CLAUDE.md looks like now
   - Recommendations for next steps
```

### After Migration: Resolve MERGE Items

If the audit identified any MERGE skills, review them manually:

1. Open the archived original: `.claude/commands/_archive/<n>.md`
2. Open the new kit version: `.claude/commands/<category>/<n>.md`
3. Identify functionality in the original that the kit version lacks
4. If the functionality is project-specific and valuable, add it to the
   kit version as a project-specific section (clearly marked)
5. If the functionality conflicts with the kit's design, prefer the kit
   version — it's designed as part of an integrated system

### Verifying the Migration

After the migration commit, verify everything works:

```bash
# Verify hook works
claude -p "echo hello"  # Should trigger validate-turn.sh

# Verify skills are accessible
# In a Claude Code session, type / and check that plan, init, implement, etc. appear

# Verify CLAUDE.md is loaded
claude -p "What project is this and what are the build commands?"
```

---

## Installing Skills

**Note:** If you used the Migration prompt above, skill installation was handled
as part of Phase 3. Skip to "Post-Installation Calibration" below.

Skills are the `.claude/commands/*.md` files that Claude Code executes when
you type `/plan`, `/review`, etc. They're extracted from the SKILL-MAP.md
reference document.

**Option A: Have Claude extract them (recommended)**

After running the bootstrap above, in the same or a new session:

```
Read the skill map at docs/SKILL-MAP.md. For each of the 19 skills defined:

1. Extract the "Command spec" content (the markdown inside the code block)
2. Create the corresponding file in .claude/commands/<category>/<name>.md
3. The file tree is documented at the top of the skill map

Create all 19 files. Do not modify the skill content — copy it exactly from
the spec. Commit when done with message "chore: install workflow skill commands"
```

**Option B: Incremental installation (if you want to start small)**

See the "Adoption Path" section below. Install only the skills for your
current adoption tier.

---

## Post-Installation Calibration

**This step is critical.** Skills ship with generic file paths like
`docs/plans/`, `docs/architecture.md`, and `scratchpad.md`. Your project's
actual architecture docs, module structure, and naming conventions may differ.
Calibration ensures every skill references real files that exist in your repo.

Run this prompt after installing skills (whether via clean install or migration):

```
I've installed the Claude Code Workflow Kit skills in .claude/commands/.
Now I need you to calibrate them to this specific project.

## STEP 1: MAP THE PROJECT

Analyze the project and build a reference map:

1. Architecture documentation:
   - Where is the main architecture doc? (could be docs/architecture.md,
     docs/ARCHITECTURE.md, README.md architecture section, or doesn't exist yet)
   - Where are module-specific docs? (docs/ecs-guide.md, docs/api.md, etc.)
   - List every .md file in docs/ and note its purpose

2. Project structure:
   - What are the top-level source directories? (src/, lib/, cmd/, etc.)
   - What are the main modules/packages?
   - Where do tests live? (tests/, __tests__/, inline, etc.)
   - Where does configuration live?

3. Existing conventions:
   - What's the commit message format? (conventional commits, custom, none)
   - What's the branch naming convention?
   - Where do new features get added? (which directories)

4. Build system:
   - What are the exact build, test, lint, and run commands?
   - Are there any project-specific validation steps?

## STEP 2: CALIBRATE SKILLS

For each installed skill file in .claude/commands/, read through it and
update any generic references to match this project:

### Path references to update:
- `docs/plans/` → verify this directory exists, create if not
- `docs/plans/archive/` → verify exists, create if not
- `docs/decisions/` → verify exists, create if not
- `docs/architecture.md` → replace with actual architecture doc path
- `scratchpad.md` → verify .gitignore includes it
- CLAUDE.md `Key Docs` section → must point to files that actually exist

### Convention references to update:
- Commit message format in /implement → match project's actual convention
- Test framework references in /gen-tests → match project's actual framework
- Doc comment style in /docs → match project's actual documentation style
  (/// for Rust, /** */ for C++/TS, // for Go, etc.)
- Build/test commands in /implement → should say "Run the project's build
  command" and reference CLAUDE.md, but verify CLAUDE.md has the right commands

### Language-specific calibration:
- If C++ project: verify /review safety checks mention the right standard (C++17, C++20, etc.)
- If Rust project: verify /gen-tests mentions proptest only if it's a dependency
- If React project: verify /gen-tests mentions the correct test library
- If Go project: verify /gen-tests mentions table-driven test patterns
- Remove or de-emphasize language sections that don't apply to this project

### Architecture-awareness calibration:
- /plan should know the project's module structure so it can suggest
  correct file placements for new code
- /review should know the project's key abstractions so it can check
  for layering violations specific to this codebase
- /sweep architecture type should reference the documented architecture,
  not generic patterns
- /fresh-eyes should be pointed at the files that matter most

## STEP 3: UPDATE CLAUDE.md KEY DOCS

Verify the Key Docs section in CLAUDE.md points to real files:

```markdown
## Key Docs
- Architecture: see <actual path or "not yet created">
- Workflow: see docs/WORKFLOW.md
- Plans: see docs/plans/
- Decisions: see docs/decisions/
```

If architecture docs don't exist yet, note it — the first /plan session
is a good time to create them.

## STEP 4: VERIFY HOOK COMPATIBILITY

1. Run scripts/validate-turn.sh manually and confirm it works:
   ```bash
   bash scripts/validate-turn.sh
   ```
2. If it fails, fix the build/test commands

3. Check that .claude/settings.json hooks reference scripts that exist

## STEP 5: REPORT

List every change made during calibration:
- Skills modified: <list with what changed>
- CLAUDE.md updated: <what changed>
- Files created: <any missing directories or docs>
- Hooks verified: <pass/fail>
- Remaining items: <anything that needs manual attention>

Commit calibration changes:
git commit -am "chore: calibrate workflow kit skills to project structure"
```

### Why Calibration Matters

Skills that reference nonexistent files fail silently — the agent either
hallucinations content for the file path or skips the step entirely. Both
are worse than an error. Calibration ensures:

- `/plan` knows where to create plan files and what module structure to follow
- `/review` checks against the actual architecture, not a generic one
- `/gen-tests` uses the right test framework and file placement
- `/docs` updates the real documentation files, not hypothetical ones
- `/implement` uses the correct build and test commands
- CLAUDE.md `Key Docs` points to files the agent can actually read

---

## Directory Structure After Setup

```
your-project/
├── .claude/
│   ├── commands/
│   │   ├── planning/
│   │   │   ├── plan.md
│   │   │   └── plan-review.md
│   │   ├── session/
│   │   │   ├── init.md
│   │   │   ├── handoff.md
│   │   │   └── cleanup.md
│   │   ├── build/
│   │   │   ├── implement.md
│   │   │   └── build-interfaces.md
│   │   ├── validate/
│   │   │   ├── review.md
│   │   │   ├── done-check.md
│   │   │   ├── sweep.md
│   │   │   ├── adversarial.md
│   │   │   └── fresh-eyes.md
│   │   ├── test/
│   │   │   └── gen-tests.md
│   │   ├── integrate/
│   │   │   ├── predict-conflicts.md
│   │   │   └── integrate-review.md
│   │   ├── release/
│   │   │   ├── pre-release.md
│   │   │   ├── changelog.md
│   │   │   └── catalog-debt.md
│   │   └── docs/
│   │       ├── docs.md
│   │       └── doc-decision.md
│   ├── settings.json          ← hook configuration
│   ├── scope.txt              ← (created per-workstream, gitignored)
│   └── worktrees/             ← (auto-managed, gitignored)
├── docs/
│   ├── WORKFLOW.md            ← workflow philosophy and process
│   ├── SKILL-MAP.md           ← skill reference and specifications
│   ├── plans/                 ← active plan files
│   │   └── archive/           ← completed plans
│   ├── decisions/             ← ADR files
│   └── debt-backlog.md        ← (generated by /catalog-debt)
├── scripts/
│   ├── validate-turn.sh       ← Stop hook: build + test validation
│   └── check-scope.sh         ← PreToolUse hook: scope enforcement
├── scratchpad.md              ← session state (gitignored)
├── CLAUDE.md                  ← project configuration for Claude
├── CHANGELOG.md
└── README.md
```

**What gets committed vs. gitignored:**

| Artifact | Committed | Rationale |
|----------|-----------|-----------|
| `.claude/commands/**` | ✅ Yes | Skills are project configuration, shared across machines |
| `.claude/settings.json` | ✅ Yes | Hook config is project configuration |
| `docs/WORKFLOW.md` | ✅ Yes | Process documentation |
| `docs/SKILL-MAP.md` | ✅ Yes | Skill reference documentation |
| `docs/plans/*.md` | ✅ Yes | Architectural decisions and context |
| `docs/plans/archive/` | ✅ Yes | Historical record |
| `docs/decisions/*.md` | ✅ Yes | ADRs are permanent records |
| `scripts/*.sh` | ✅ Yes | Build tooling |
| `CLAUDE.md` | ✅ Yes | Project configuration |
| `scratchpad*.md` | ❌ No | Transient session state, per-developer |
| `.claude/worktrees/` | ❌ No | Ephemeral working directories |
| `.claude/scope.txt` | ❌ No | Per-session scope constraints |

---

## Adoption Path

**Do not try to use all 19 skills on day one.** Adopt in tiers, starting
with the skills that give the most value for the least ceremony.

### Tier A: Essentials (Start Here)

Install and use these 5 skills first. They cover 80% of the value for solo
development without parallel workstreams.

| Skill | Why It's Essential |
|-------|--------------------|
| `/plan` | Prevents the #1 failure mode: building without clear scope |
| `/implement` | Structured build loop with commit discipline and self-review |
| `/handoff` | Session state persistence — never re-explain context |
| `/init` | Clean session startup from persisted state |
| `/review` | Fresh-session code review catches what the builder misses |

**Workflow with Tier A only:**

```
/plan → approve → /init → /implement → /handoff (if pausing)
→ /init (if resuming) → /review (fresh session) → merge
```

This is enough for all Tier 0 and Tier 1 work.

### Tier B: Quality Gate (Add After 1-2 Weeks)

Once you're comfortable with Tier A, add these for stronger validation:

| Skill | Why to Add It |
|-------|---------------|
| `/done-check` | Formal gate prevents "close enough" merges |
| `/gen-tests` | Autonomous test generation — great for parallel execution |
| `/docs` | Keeps documentation from drifting after every feature |
| `/cleanup` | Prevents repo clutter from accumulating |

### Tier C: Parallel Development (Add When Needed)

Add these when you start running parallel workstreams (Tier 2 work):

| Skill | Why to Add It |
|-------|---------------|
| `/build-interfaces` | Contract-first pattern for parallel safety |
| `/predict-conflicts` | Saves hours of merge pain |
| `/integrate-review` | Catches cross-workstream integration issues |
| `/plan-review` | Tier 2 plans are complex enough to need external review |

### Tier D: Advanced Validation (Add for Critical Code)

These are high-value for specific situations, not every feature:

| Skill | When to Use |
|-------|-------------|
| `/sweep` | Post-merge on large changes, periodic maintenance |
| `/adversarial` | Critical paths, security-sensitive code, concurrency |
| `/fresh-eyes` | Mature codebases, post-refactor, onboarding prep |

### Tier E: Release Management (Add When Releasing)

| Skill | When to Use |
|-------|-------------|
| `/pre-release` | Before any tagged release |
| `/changelog` | Generating release notes |
| `/catalog-debt` | Post-sprint debt management |
| `/doc-decision` | Architectural decisions worth recording |

---

## Customizing Per Project

### Language-Specific Adjustments

The skills are language-agnostic by default. Customize these per-project:

**CLAUDE.md** — The primary customization point. The build/test/lint commands
here determine what `/implement` runs after each change and what the Stop hook
validates. Get these right first.

**scripts/validate-turn.sh** — Adapt to your build system:

```bash
# Rust
cargo build 2>&1 || { echo "BUILD FAILED"; exit 1; }
cargo test 2>&1 || { echo "TESTS FAILED"; exit 1; }

# C++ with CMake
cmake --build build 2>&1 || { echo "BUILD FAILED"; exit 1; }
cd build && ctest 2>&1 || { echo "TESTS FAILED"; exit 1; }

# React/TypeScript
npm run build 2>&1 || { echo "BUILD FAILED"; exit 1; }
npm test -- --watchAll=false 2>&1 || { echo "TESTS FAILED"; exit 1; }

# Go
go build ./... 2>&1 || { echo "BUILD FAILED"; exit 1; }
go test ./... 2>&1 || { echo "TESTS FAILED"; exit 1; }
```

### Optional Hooks

These are NOT enabled by default because they have tradeoffs:

**PostToolUse auto-formatter** — Runs your formatter after every file edit.
Keeps code clean but consumes context tokens. Enable if your project has
strict formatting and the context cost is acceptable.

```json
{
  "hooks": {
    "PostToolUse": [
      {
        "tool": "edit",
        "command": "rustfmt $TOOL_INPUT_PATH"
      }
    ]
  }
}
```

**PreToolUse scope enforcement** — Blocks edits outside an allowed file list.
Only useful during parallel workstreams. Enable per-session by creating
`.claude/scope.txt` with one file path per line before starting a workstream.

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "tool": "edit",
        "command": "scripts/check-scope.sh $TOOL_INPUT_PATH"
      }
    ]
  }
}
```

### Adapting Skills

Skills are just markdown files. You can and should customize them:

**Safe to customize:**
- Output format templates (change the report structure)
- Language-specific sections (add your framework's patterns)
- Commit message format (match your project conventions)
- File paths (if your docs live somewhere other than `docs/`)

**Dangerous to customize:**
- Removing the self-review step from `/implement` (it catches real bugs)
- Removing the "STOP if plan is wrong" rule (silent divergence is expensive)
- Combining `/review` and `/done-check` into one skill (they serve different purposes)
- Removing per-workstream scratchpad logic (concurrent writes will conflict)

### Creating Custom Skills

The 19 kit skills cover the general development workflow. Your project will
likely need additional skills for domain-specific tasks. Create these in
`.claude/commands/custom/` to keep them separate from kit skills.

**When to create a custom skill vs. extend an existing one:**

- The task is project-specific and runs as its own session → **create custom skill**
  (e.g., `/deploy`, `/db-migrate`, `/seed-data`, `/run-benchmarks`)
- The task is a variation of an existing skill → **extend the existing skill**
  with a project-specific section (e.g., adding ECS-specific checks to `/review`)
- The task is a workflow shortcut combining multiple skills → **create a custom
  orchestration skill** (e.g., `/ship` that runs `/done-check` → `/docs` → merge)

**Custom skill template:**

Create `.claude/commands/custom/<skill-name>.md`:

```markdown
# /<skill-name> — <One-Line Description>

## Context
<!-- What this skill needs to know before executing -->
- Project area: <which modules/files this operates on>
- Dependencies: <other skills or state that must exist first>
- Architecture ref: <path to relevant architecture doc>

## Instructions

<Clear, step-by-step instructions for the agent. Follow the same
patterns used in kit skills:>

## Step 1: <Name>
<What to do, what to check>

## Step 2: <Name>
<What to do, what to produce>

## Output
<What artifact this skill produces: file, commit, report>

## Constraints
<What the agent must NOT do>
```

**Custom skill conventions:**

- Place in `.claude/commands/custom/` — never mix with kit skill directories
- Name descriptively: `/db-migrate` not `/migrate`, `/run-e2e` not `/test2`
- Reference architecture docs by their actual path, not generically
- Include a Context section listing what files/modules the skill operates on
- Include Constraints — what the agent should avoid doing
- If the skill modifies code, include the commit message format
- If the skill depends on a kit skill running first, say so explicitly

**Naming conflicts:** If you need a custom skill whose natural name conflicts
with a kit skill (e.g., you want a project-specific `/review` that does
something different), DON'T shadow the kit skill. Instead:
- Name it more specifically: `/review-sql` or `/review-security`
- Or extend the existing `/review` skill with a project-specific section
- The kit skill names are reserved — they're part of the integrated workflow
  and other skills reference them by name

### Keeping Skills Current After Architecture Changes

Skills reference specific file paths, module names, documentation locations,
and conventions. When your project's architecture evolves, skills can become
stale. This is the documentation-code drift problem applied to your workflow.

**When to re-calibrate:**

- After adding or renaming major modules or directories
- After changing the build system or test framework
- After moving or renaming documentation files
- After changing commit or branch naming conventions
- After significant refactoring that changes the module graph

**Re-calibration prompt:**

```
The project structure has changed. I need you to re-calibrate the workflow
kit skills to match the current codebase.

1. Read every file in .claude/commands/ (all categories)
2. For each file, check every path reference and verify it still points
   to a file or directory that exists
3. Check every module/package name reference against the actual codebase
4. Check CLAUDE.md Key Docs section — do all referenced files exist?
5. Check scripts/validate-turn.sh — do the build/test commands still work?

For each stale reference found:
- Show: <file> line <N>: references <old path> → should be <new path>
- Or: <file> line <N>: references <module> which no longer exists

After listing all findings, update every stale reference and commit:
git commit -am "chore: re-calibrate workflow skills after architecture changes"
```

**Proactive approach:** Add a reminder to your monthly maintenance checklist
(see Maintenance section) to run the re-calibration prompt. Or better: run
`/docs` after every feature that changes the project structure — it syncs
architecture docs, and the next calibration will pick up the updated paths.

---

## Setting Up a New Repo From Scratch

For brand-new projects with no existing code, use this extended bootstrap:

```
I'm starting a new project from scratch. Here's what I need:

PROJECT DETAILS:
- Name: <name>
- Language: <language/framework>
- Purpose: <one sentence>
- Architecture: <any known patterns, e.g., ECS, clean arch, microservices>

Do the following:

1. Initialize the project with the appropriate toolchain:
   - <language-specific init: cargo init, npm init, cmake setup, go mod init>
   - Set up the build system
   - Set up a test framework with one placeholder test
   - Set up a linter

2. Run the full Claude Code Workflow Kit bootstrap:
   - Create .claude/commands/ directory structure
   - Create CLAUDE.md from the detected project configuration
   - Create scratchpad.md
   - Create scripts/validate-turn.sh adapted to this project
   - Create scripts/check-scope.sh
   - Configure .claude/settings.json with Stop hook
   - Update .gitignore

3. Create initial documentation:
   - docs/WORKFLOW.md (copy from the workflow reference)
   - docs/plans/ and docs/plans/archive/ directories
   - docs/decisions/ directory

4. Make initial commit:
   git add -A
   git commit -m "chore: initialize project with Claude Code Workflow Kit"

5. Report what was created and confirm the build/test commands work.
```

After running this, proceed to "Installing Skills" and then
"Post-Installation Calibration". For a brand-new project, calibration is
lighter since paths are freshly created, but it's still worth running
to verify hook compatibility and build/test commands work correctly.

---

## Workflow Cheat Sheet

Pin this somewhere visible until the workflow becomes habitual.

```
╔═══════════════════════════════════════════════════════════╗
║                  WHICH TIER AM I?                        ║
╠═══════════════════════════════════════════════════════════╣
║  1-2 files, obvious fix?     → Tier 0: Just do it       ║
║  3-10 files, one concern?    → Tier 1: /plan first      ║
║  10+ files or parallel?      → Tier 2: Full planning    ║
║  Not sure?                   → Start at Tier 0           ║
╠═══════════════════════════════════════════════════════════╣
║                                                          ║
║  TIER 1 FLOW                                             ║
║  /plan → /plan-review → /init → /implement →             ║
║  /gen-tests → /review → /done-check → /docs → merge     ║
║                                                          ║
╠═══════════════════════════════════════════════════════════╣
║                                                          ║
║  ALWAYS                                                  ║
║  • /handoff before ending a session                      ║
║  • /init when starting a session                         ║
║  • /review in a FRESH session (not the build session)    ║
║  • Commit after every subtask, not at the end            ║
║                                                          ║
╠═══════════════════════════════════════════════════════════╣
║                                                          ║
║  WHEN THINGS GO WRONG                                    ║
║  • Plan is wrong     → /handoff → update plan → /init   ║
║  • Context full      → /handoff → new session → /init   ║
║  • Scope grew        → /handoff --escalate → /plan      ║
║  • 2 failed fixes    → /clear → rewrite the prompt      ║
║                                                          ║
╚═══════════════════════════════════════════════════════════╝
```

---

## Maintenance

### Weekly (5 minutes)

- Review `scratchpad.md` — clear stale entries, resolve open questions
- Skim `.claude/worktrees/` — remove any forgotten worktrees (`git worktree prune`)

### Monthly (15 minutes)

- Review CLAUDE.md:
  - Delete instructions Claude already follows by default
  - Add Gotchas for any mistakes Claude made repeatedly
  - Verify build/test/lint commands are still current
- Run `/catalog-debt` if you haven't since the last release
- Archive completed plan files: `mv docs/plans/*.md docs/plans/archive/`
- Re-calibrate skills if project structure changed (see "Keeping Skills
  Current After Architecture Changes" in Customizing Per Project)

### Per Release

- Run `/pre-release` for the audit checklist
- Run `/changelog` to generate release notes
- Run `/catalog-debt` to build the next cycle's debt backlog
- Run `/cleanup` to archive plans, prune branches, reset scratchpads

### Skill Updates

Skills are versioned with your project. When you improve a skill based on
experience (e.g., adding a project-specific check to `/review`), commit the
change with `chore: update /review skill — add PostgreSQL-specific checks`.

Track skill improvements the same way you track code improvements — they're
part of your codebase.

---

## Sharing Across Projects

The skills are designed to be project-agnostic. To share across repos:

**Option 1: Copy the `.claude/commands/` directory** into each repo. Each project
gets its own copy that can be customized independently. Simple, no dependencies.

**Option 2: Maintain a shared repo** with the canonical skills, and copy/symlink
into projects. More maintenance overhead but ensures consistency. Best if you
have 5+ projects using the same workflow.

**Option 3: Use a global `~/.claude/commands/` directory** for skills that are
truly project-agnostic (like `/handoff`, `/init`, `/cleanup`). Project-specific
skills (like `/implement` with language-specific test commands) stay in the
project's `.claude/commands/`. Claude Code merges both.

Recommendation: Start with Option 1. Move to Option 2 or 3 only after
you've stabilized the skills through a few projects and know which are
truly universal vs. project-specific.

---

## Troubleshooting

### "Claude ignores my skill instructions"

CLAUDE.md has a practical instruction budget of ~100-150 directives before
compliance degrades. If your CLAUDE.md is too long, skill instructions
compete for attention. Prune CLAUDE.md aggressively — it should be under 60 lines.

### "The Stop hook makes Claude slow"

The validate-turn.sh hook runs after every Claude response. If your build
or test suite is slow (>30 seconds), this adds up. Options:
- Make the hook only run tests relevant to the changed files
- Disable the hook during exploratory sessions and re-enable for implementation
- Move validation to the skill level (inside `/implement`) instead of the hook

### "Scratchpad conflicts in parallel worktrees"

This was a known issue, resolved by per-workstream scratchpads. Each worktree
writes to `scratchpad-<workstream>.md`, not the shared `scratchpad.md`. If
you're seeing conflicts, verify that `/init` and `/handoff` are using the
workstream-specific scratchpad. The main `scratchpad.md` should only be
written by the main session.

### "Skills are too verbose / too many steps for simple tasks"

Use the tier system. Tier 0 tasks (1-2 files, obvious fix) don't use skills
at all. If you find yourself running `/plan` for a one-line bug fix, you've
over-tiered it. Start lower and escalate only when complexity surprises you.

### "I don't know which sweep types to run"

The `/sweep` skill has a built-in selection heuristic. If you run `/sweep`
without specifying a type, it reads the plan and recent diff to recommend
1-2 sweep types based on the nature of the change. Trust the heuristic
until you have a reason to override it.

### "Skills reference files that don't exist"

Skills ship with generic paths that need calibration to your project. If
a skill references `docs/architecture.md` but your architecture doc is at
`docs/system-design.md`, the skill will fail silently. Run the
Post-Installation Calibration prompt (see that section above) to fix all
path references at once. Add re-calibration to your monthly maintenance
if your project structure changes frequently.

### "I have existing skills that conflict with kit skills"

The migration prompt handles this — it audits existing skills, identifies
conflicts, and gives you a REPLACE/KEEP/MERGE decision for each one. The
rule is: kit skills take priority for overlapping names because they're
designed as an integrated system. Your custom skills are preserved under
`.claude/commands/custom/` or in `_archive/` if they were replaced. See
the "Migrating an Existing Project" section for the full process.

---

## Design Decisions and PM Review Notes

These notes capture the rationale behind key design choices. They're here so
future-you (or anyone adapting this kit) understands why things are the way
they are and can make informed changes.

### Why 19 skills and not fewer?

We started at 22 and consolidated aggressively. Each merge was evaluated
against the question "would these ever be run independently in different
sessions?" If yes, they stay separate. If no, they merge. The 19 that remain
each have a distinct session context, trigger condition, and output artifact.
Further merging would sacrifice the ability to use them independently.

### Why not auto-install skills in the bootstrap prompt?

The bootstrap prompt creates infrastructure (directories, CLAUDE.md, hooks,
scripts) but does NOT install skill files. This is intentional:

1. Skill installation is a large operation (19 files, ~2500 lines of content
   extracted from the skill map). Combining it with project analysis would
   strain the context window.
2. Incremental adoption is encouraged — not everyone needs all 19 skills
   immediately. Separating installation lets you install only the skills
   for your current adoption tier.
3. The bootstrap prompt needs to be project-aware (detecting build system,
   adapting scripts). The skill files are project-agnostic. Mixing these
   concerns makes the prompt harder to maintain.

### Why scratchpad files are gitignored

Scratchpads are transient session state — "where am I right now" and "what
should I do next." They're developer-specific and change every session.
Committing them would create constant noise in the git history and merge
conflicts between developers (or between your own parallel sessions).

Plan files ARE committed because they represent architectural decisions that
have lasting value. The plan is the "what should we build." The scratchpad
is the "where are we in building it."

### Why the Stop hook runs tests but the PreToolUse scope hook is opt-in

The Stop hook (build + test validation) catches broken builds immediately
and should always be active. The cost is a few seconds per Claude turn.

The PreToolUse scope hook (file edit restriction) is only useful during
parallel workstreams, adds friction during normal development, and requires
creating a scope file before each session. Making it always-on would slow
down Tier 0 and Tier 1 work for no benefit.

### Why validation is review THEN done-check, not the reverse

`/review` finds bugs. `/done-check` confirms completeness. If done-check
runs first and finds everything "complete," but review then finds a bug that
requires code changes, the done-check results are invalidated and must be
re-run. Running review first means bugs are fixed before the completeness
gate, so done-check only needs to run once.

### Why `/adversarial` doesn't overlap with `/review`

Early versions had significant overlap — both checked edge cases, null inputs,
and error handling. This meant running both was ~60% redundant work. The final
version draws a hard line: `/review` covers correctness (does the code do
what the spec says, are errors handled, are basic edge cases covered).
`/adversarial` covers hostility (what happens when the environment misbehaves,
operations interleave unexpectedly, or inputs are crafted to exploit
assumptions). Neither references the other's findings.

### Why the tier system exists

Without tiers, every task gets the same ceremony. A one-line bug fix goes
through planning, implementation, validation, and documentation — adding
30 minutes of overhead to a 2-minute change. The tier system matches
ceremony to complexity. The key insight is the escalation path: starting
too low costs only a pause to plan. Starting too high wastes time on
unnecessary ceremony. So the default is always "start lower."

### What this kit does NOT solve

- **Multi-developer coordination**: This is designed for solo development.
  Multi-developer workflows need PR-based review processes, branch protection,
  and team coordination that this kit doesn't address.
- **CI/CD integration**: The hooks and scripts run locally. Connecting them
  to a CI pipeline (GitHub Actions, etc.) is a natural extension but not
  included here.
- **Project management**: Plans are technical specifications, not project
  management artifacts. If you need sprint planning, backlog grooming, or
  timeline estimation, those are separate concerns.
- **Model fine-tuning**: The skills are prompt-engineered for Claude's
  current capabilities. As models improve, some skills (especially
  `/implement`'s self-review step and `/review`'s safety checks) may
  need to be adjusted.

---

## Reference

| Document | Location | Purpose |
|----------|----------|---------|
| Workflow | `docs/WORKFLOW.md` | Process philosophy, tier definitions, cross-cutting concerns, error recovery |
| Skill Map | `docs/SKILL-MAP.md` | Complete skill specs, interaction map, workflow diagrams |
| Project Config | `CLAUDE.md` | Project-specific Claude Code configuration |
| Skills | `.claude/commands/` | The skills themselves (executable by Claude Code) |
| Plans | `docs/plans/` | Active and archived development plans |
| Decisions | `docs/decisions/` | Architecture Decision Records |
| Debt Backlog | `docs/debt-backlog.md` | Prioritized tech debt (generated by `/catalog-debt`) |
| Scratchpad | `scratchpad*.md` | Transient session state (gitignored) |

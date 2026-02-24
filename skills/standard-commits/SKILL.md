# Skill: standard-commits

Conventional Commits, Semantic Versioning, and Git Branching Strategy for Knowledge Framework projects.

## Overview

This skill defines the canonical standards for commit messages, version numbering, and branch naming across all projects managed by `kn` (Knowledge Framework).

Every agent MUST follow these conventions. Non-conforming commits may be rejected by CI.

---

## 1. Conventional Commits

All commit messages MUST follow [Conventional Commits](https://www.conventionalcommits.org/) format.

### Format

```text
<type>(<scope>): <message>
```

- **`type`**: Category of change (see table below)
- **`scope`**: Component or module affected (optional but recommended)
- **`message`**: Clear, concise description in imperative mood ("add", not "added")

### Examples

```text
feat(auth): add JWT refresh token endpoint
fix(api): resolve timeout in user lookup
refactor(db): extract connection pool into module
perf(query): optimize batch insert with prepared statements
build(cargo): update dependencies to latest compatible versions
ci(actions): add dev branch to CI workflow
chore(deps): clean up unused imports
docs(readme): update installation instructions
test(auth): add integration tests for login flow
```

### Breaking Changes

Append `!` after the type (before colon) to indicate an incompatible change:

```text
feat(api)!: change response format to JSON:API spec
fix(db)!: rename users table to accounts
```

Breaking changes trigger a **MAJOR** version bump.

### Accepted Commit Types

| Type       | Description                                    | SemVer Bump |
|------------|------------------------------------------------|-------------|
| `feat`     | New functionality                              | **MINOR**   |
| `fix`      | Bug fix (including urgent production fixes)     | **PATCH**   |
| `refactor` | Code restructuring, no behavior change         | **PATCH**   |
| `perf`     | Performance optimization                       | **PATCH**   |
| `build`    | Build system (Cargo, Docker, install scripts)  | **PATCH**   |
| `ci`       | CI/CD configuration (GitHub Actions, workflows)| **PATCH**   |
| `chore`    | General maintenance, dependencies, cleanup     | **PATCH**   |
| `docs`     | Documentation changes                          | **PATCH**   |
| `style`    | Formatting, linting, whitespace                | **PATCH**   |
| `test`     | Test additions or modifications                | **PATCH**   |
| `any!`     | Any type with `!` = breaking change            | **MAJOR**   |

### Multi-line Commits

For complex changes, use a body and footer:

```text
feat(auth): add OAuth2 integration

Implement OAuth2 authorization code flow with Google and GitHub providers.
Includes token refresh, scope validation, and session management.

Closes: knowledge-abc.3
Co-authored-by: Backend Agent <knowledge-vlf@beads>
```

---

## 2. Semantic Versioning (SemVer)

Versions follow [Semantic Versioning](https://semver.org/): **`vMAJOR.MINOR.PATCH`**

### Version Determination Logic

When analyzing commits from the last tag to HEAD:

1. **If ANY commit has `!`** (breaking change) -> **MAJOR** bump
   ```
   MAJOR += 1, MINOR = 0, PATCH = 0
   ```

2. **If ANY commit is `feat`** (and no breaking) -> **MINOR** bump
   ```
   MINOR += 1, PATCH = 0
   ```

3. **If commits are `fix|refactor|perf|build|ci|chore|docs|style|test`** -> **PATCH** bump
   ```
   PATCH += 1
   ```

4. **If NO recognized types** -> No tag generated

### Tag Strategy by Branch

| Branch | Tag Format         | Example          | Trigger           |
|--------|--------------------|------------------|-------------------|
| `dev`  | `vX.Y.Z-rc.N`     | `v1.5.0-rc.3`   | Push/merge to dev |
| `prod` | `vX.Y.Z`           | `v1.5.0`         | Push/merge to prod|

#### Release Candidate (RC) Tags on `dev`

1. Calculate next version `vX.Y.Z` from commits since last stable tag
2. Find latest `vX.Y.Z-rc.N` for that version
3. Increment N: `vX.Y.Z-rc.(N+1)`
4. If no RC exists yet: `vX.Y.Z-rc.1`

**Example:**
```
Last stable tag:    v1.4.2
Commits include:    feat(api): add search endpoint
Calculated version: v1.5.0
Last RC:            v1.5.0-rc.2
New tag:            v1.5.0-rc.3
```

#### Stable Tags on `prod`

1. Calculate version `vX.Y.Z` from commits since last stable tag
2. Create tag `vX.Y.Z` (no suffix)
3. This triggers the release workflow (binary builds, GitHub Release)

**Example:**
```
Last stable tag:    v1.4.2
Commits include:    feat(api): add search endpoint
New tag:            v1.5.0
```

### First Version

If no tags exist, the base version is `v0.0.0`. The first commit determines the initial tag:
- `feat` -> `v0.1.0` (or `v0.1.0-rc.1` on dev)
- `fix` -> `v0.0.1` (or `v0.0.1-rc.1` on dev)

---

## 3. Git Branching Strategy

### Branch Hierarchy

```
prod (stable releases)
  |
  dev (integration, RC tags)
    |
    epic/knowledge-abc (epic integration branch)
      |
      knowledge-abc/backend  (agent work branch)
      knowledge-abc/frontend (agent work branch)
      knowledge-abc/devops   (agent work branch)
      ...
```

### Branch Naming Convention

| Branch Type    | Pattern                        | Created From | PR Target    | Example                        |
|----------------|--------------------------------|--------------|--------------|--------------------------------|
| Production     | `prod`                         | -            | -            | `prod`                         |
| Integration    | `dev`                          | `prod`       | `prod`       | `dev`                          |
| Epic           | `epic/<epic-id>`               | `dev`        | `dev`        | `epic/knowledge-j3a`          |
| Agent work     | `<epic-id>/<agent-role>`       | `epic/<id>`  | `epic/<id>`  | `knowledge-j3a/backend`       |
| Independent    | `task/<task-id>`               | `dev`        | `dev`        | `task/knowledge-j3a.5`        |
| Hotfix         | `hotfix/<issue-id>`            | `prod`       | `prod`+`dev` | `hotfix/knowledge-abc`        |
| Release        | `release/v<version>`           | `dev`        | `prod`       | `release/v1.5.0`              |

### Workflow: Feature Development

```bash
# 1. Planner creates epic branch from dev
git checkout dev
git pull origin dev
git checkout -b epic/knowledge-j3a

# 2. Push epic branch
git push -u origin epic/knowledge-j3a

# 3. Agent creates work branch from epic
git checkout epic/knowledge-j3a
git checkout -b knowledge-j3a/backend
git push -u origin knowledge-j3a/backend

# 4. Agent works, commits with conventional format
git add .
git commit -m "feat(api): implement user authentication endpoint"
git push

# 5. Agent creates PR: agent-branch -> epic-branch
gh pr create \
  --base epic/knowledge-j3a \
  --head knowledge-j3a/backend \
  --title "feat(api): implement user authentication" \
  --body "Closes knowledge-j3a.2"

# 6. After all agent PRs merged to epic, create PR: epic -> dev
gh pr create \
  --base dev \
  --head epic/knowledge-j3a \
  --title "feat: implement user authentication (knowledge-j3a)" \
  --body "Epic: knowledge-j3a - Complete user authentication system"

# 7. Merge to dev
# 8. When ready for release, create PR: dev -> prod
# 9. Merge to prod and tag manually
```

### Workflow: Hotfix

```bash
# 1. Create hotfix branch from prod
git checkout prod
git pull origin prod
git checkout -b hotfix/knowledge-xyz
git push -u origin hotfix/knowledge-xyz

# 2. Fix and commit (use `fix` type — urgency is conveyed by the hotfix/ branch)
git commit -m "fix(auth): fix token expiration check"
git push

# 3. Create PR to prod AND dev
gh pr create --base prod --head hotfix/knowledge-xyz --title "fix(auth): fix token expiration"
# After merge to prod, cherry-pick or merge to dev too
```

### Workflow: Release

```bash
# 1. Create release branch from dev (optional, for release prep)
git checkout dev
git checkout -b release/v1.5.0

# 2. Final adjustments (CHANGELOG, version bumps)
git commit -m "chore: prepare release v1.5.0"

# 3. PR to prod
gh pr create --base prod --head release/v1.5.0 --title "release: v1.5.0"

# 4. Merge and tag release manually
```

---

## 4. PR Review Workflow

### Human Review Cycle

1. **Agent creates PR** (agent-branch -> epic-branch, or epic -> dev)
2. **Human reviews** and leaves comments on the PR
3. **Planner reads comments** via `gh` CLI:
   ```bash
   gh pr view <PR#> --comments
   gh api repos/OWNER/REPO/pulls/<PR#>/comments
   ```
4. **Planner creates issues** in beads for each review item
5. **Agents resolve** the review items and push fixes
6. **Planner verifies** fixes and approves the PR:
   ```bash
   gh pr review <PR#> --approve -b "All review items resolved"
   ```
7. **PR is merged** (squash merge recommended for clean history)

### Merge Strategy

| PR Type              | Merge Method | Rationale                              |
|----------------------|--------------|----------------------------------------|
| Agent -> Epic        | Squash       | Clean single commit per agent's work   |
| Epic -> Dev          | Merge        | Preserve epic's commit history         |
| Dev -> Prod          | Merge        | Preserve full history for stable tag   |
| Hotfix -> Prod       | Squash       | Single clean fix commit                |

---

## 5. Agent Commit Rules

### MUST

- Use conventional commit format for EVERY commit
- Include scope when the change is in a specific module
- Reference beads issue ID in commit body or PR description
- Push to your agent branch, never directly to epic/dev/prod

### MUST NOT

- Force push to shared branches (epic, dev, prod)
- Commit directly to dev or prod (use PRs)
- Use vague commit messages ("fix stuff", "update code", "wip")
- Skip the type prefix

### SHOULD

- Keep commits atomic (one logical change per commit)
- Use imperative mood ("add feature" not "added feature")
- Include `Closes: <issue-id>` in commit body for auto-closing
- Sign commits if GPG is configured

---

## 6. CI/CD Integration

### GitHub Actions Workflows

| Workflow            | Trigger                  | Action                              |
|---------------------|--------------------------|-------------------------------------|
| `ci.yml`            | Push/PR to dev, prod     | Run tests, clippy, fmt              |
| `release.yml`       | Tag `v*`                 | Build binaries, create GH Release   |

---

## References

- [Semantic Versioning (SemVer)](https://semver.org/)
- [Conventional Commits](https://www.conventionalcommits.org/)
- [GitHub Actions](https://docs.github.com/en/actions)
- [GitHub CLI (gh)](https://cli.github.com/manual/)

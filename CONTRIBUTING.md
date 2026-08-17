# Contributing to Knowledge Framework

Thanks for considering a contribution. This document covers everything you need: where to send a change, how to name your branch, how to write the commit message, and how work gets specified before it is built.

This is a personal framework the maintainer uses for their own projects, so the roadmap is opinionated. Bug reports and fixes are always welcome; before investing time in a large feature, please open an issue to check it fits the direction.

## Quick answers

| Question | Answer |
|---|---|
| Found a bug? | [Open a bug report](https://github.com/kobogithub/knowledge/issues/new/choose) |
| Want a feature? | [Open a feature request](https://github.com/kobogithub/knowledge/issues/new/choose) first — before writing code |
| Small fix ready to send? | Branch `task/<short-desc>` from `dev`, PR into `dev` |
| Where do PRs go? | `dev`, never `prod` |
| Commit format? | `<type>(<scope>): <message>` — see [Commit messages](#commit-messages) |

## Getting set up

```bash
git clone https://github.com/kobogithub/knowledge.git
cd knowledge/cli
cargo build --release
cargo test
```

Before opening a pull request, run what CI runs:

```bash
cargo fmt --check --manifest-path cli/Cargo.toml
cargo clippy --manifest-path cli/Cargo.toml --all-targets --all-features -- -D warnings
cargo test --release --manifest-path cli/Cargo.toml
shellcheck install.sh
```

## Branching

Branches form a hierarchy. Work flows upward; nothing is committed directly to `prod`, `dev` or an `epic/*` branch.

```
prod                          stable releases
 └─ dev                       integration
     └─ epic/<feature-id>     one initiative (e.g. epic/002-public-ready-repo)
         └─ <feature-id>/<role>   a slice of that initiative
```

| Branch type | Pattern | Branch from | PR target |
|---|---|---|---|
| Production | `prod` | — | — |
| Integration | `dev` | `prod` | `prod` |
| Feature | `epic/<feature-id>` | `dev` | `dev` |
| Work slice | `<feature-id>/<role>` | `epic/<feature-id>` | `epic/<feature-id>` |
| Independent change | `task/<short-desc>` | `dev` | `dev` |
| Hotfix | `hotfix/<short-desc>` | `prod` | `prod` **and** `dev` |
| Release | `release/v<version>` | `dev` | `prod` |

**As an outside contributor**, you almost always want the independent-change row: fork the repository, branch `task/<short-desc>` from `dev`, and open a pull request against `dev`. The `epic/` and `<feature-id>/<role>` branches exist for multi-part initiatives coordinated inside the repository.

Rules:

1. Never commit directly to `prod`, `dev` or `epic/*`.
2. Never force-push a shared branch.
3. Always open a pull request; nothing merges by direct push.

## Commit messages

Every commit message must use [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <message>
```

| Type | Use for | Version bump |
|---|---|---|
| `feat` | New functionality | MINOR |
| `fix` | Bug fix | PATCH |
| `refactor` | Restructuring with no behaviour change | PATCH |
| `perf` | Performance work | PATCH |
| `build` | Build system (Cargo, Docker, packaging) | PATCH |
| `ci` | CI/CD workflows | PATCH |
| `chore` | Maintenance, dependencies, cleanup | PATCH |
| `docs` | Documentation only | PATCH |
| `style` | Formatting, linting | PATCH |
| `test` | Tests | PATCH |
| any with `!` | Breaking change | **MAJOR** |

Examples:

```
feat(stack): add htmx-go preset
fix(init): stop overwriting an existing kn.toml
docs(readme): put installation before the feature catalog
build(cargo): bump clap to 4.6
feat(cli)!: rename kn sync to kn apply
```

The full reference — including SemVer rules and tagging — lives in the [`standard-commits` skill](./skills/standard-commits/SKILL.md).

## How work is specified

This project is spec-driven: substantial work is specified before it is implemented. Each initiative lives in [`specs/`](./specs/) as a numbered folder containing:

| File | What it holds |
|---|---|
| `spec.md` | What the change must achieve, in user-facing terms — no implementation detail |
| `plan.md` | How it will be built, with the research behind each decision |
| `tasks.md` | Dependency-ordered checkboxes, grouped by user story |

The cycle is `/speckit-specify` → `/speckit-plan` → `/speckit-tasks` → `/speckit-implement`, using [spec-kit](https://github.com/github/spec-kit). See [`specs/002-public-ready-repo/`](./specs/002-public-ready-repo/) for a worked example.

**You do not need to write a spec for a bug fix or a small change.** Go straight to a `task/` branch. Specs are for work large enough to need decomposition — if you are unsure, open an issue and ask.

Architectural decisions are recorded as ADRs in [`docs/adr/`](./docs/adr/). If your change reverses or supersedes a recorded decision, say so in the pull request; a new ADR may be needed.

## Pull requests

Opening a pull request pre-fills a template asking for three things:

1. **What changed and why** — enough that a reviewer need not reconstruct it from the diff.
2. **The related initiative**, if any — link the `specs/` folder or the issue.
3. **How you verified it** — the commands you ran and what they reported. "Tests pass" is weaker than the output.

Keep pull requests focused. A change that fixes a bug and also reformats three files is two pull requests.

## Reporting bugs

Use the [bug report form](https://github.com/kobogithub/knowledge/issues/new/choose). It asks for your `kn --version`, your operating system, and the exact steps to reproduce, because those three are what make a report actionable.

If you have found a security vulnerability, please **do not** open a public issue — email the maintainer directly.

## Documentation

Documentation lives in two places: [`README.md`](./README.md) (and its [Spanish translation](./README_ES.md)) for anything a new user needs, and [`docs/`](./docs/README.md) for everything else.

If you change one README, change the other. They are expected to present the same structure and the same claims; a contribution that updates only one leaves the project stating two different things.

## License

By contributing, you agree that your contributions are licensed under the [MIT License](./LICENSE).

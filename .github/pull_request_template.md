## What changed and why

<!--
  Enough that a reviewer does not have to reconstruct the intent from the diff.
  If this fixes an issue, link it: "Fixes #123".
-->

## Related initiative

<!--
  Link the specs/ folder if this is part of an initiative, e.g. specs/002-public-ready-repo/.
  Write "None — standalone change" for a bug fix or small change. Not every PR needs a spec.
-->

## How this was verified

<!--
  The commands you ran and what they reported. Paste output where it helps —
  it is more convincing than "tests pass".
-->

```
```

## Checklist

- [ ] Branch follows the naming in [CONTRIBUTING.md](../CONTRIBUTING.md) and targets the right base (`dev` for most changes, never `prod`)
- [ ] Commits use the conventional format `<type>(<scope>): <message>`
- [ ] `cargo fmt --check`, `cargo clippy -- -D warnings` and `cargo test --release` pass
- [ ] Both `README.md` and `README_ES.md` updated, if either changed
- [ ] Documentation links resolve (no reference to a file that does not exist)

## Anything reviewers should know

<!--
  Trade-offs you made, alternatives you rejected, parts you are unsure about,
  or follow-up work you deliberately left out of scope.
-->

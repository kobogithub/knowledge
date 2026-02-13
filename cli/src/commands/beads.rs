use anyhow::{Context, Result};
use clap::{Args, Subcommand, ValueEnum};
use colored::*;
use std::fs;
use std::path::Path;

#[derive(Args)]
pub struct BeadsCommand {
    #[command(subcommand)]
    action: BeadsAction,
}

#[derive(Subcommand)]
enum BeadsAction {
    /// Generate an issue template
    Template(TemplateArgs),
}

#[derive(Args)]
struct TemplateArgs {
    /// Type of template to generate
    #[arg(value_enum)]
    template_type: TemplateType,

    /// Output file path (prints to stdout if not specified)
    #[arg(short, long)]
    output: Option<String>,

    /// Force overwrite if file exists
    #[arg(short, long)]
    force: bool,
}

#[derive(Debug, Clone, ValueEnum)]
enum TemplateType {
    Epic,
    Task,
    Bug,
    Feature,
    Chore,
}

impl BeadsCommand {
    pub fn execute(&self) -> Result<()> {
        match &self.action {
            BeadsAction::Template(args) => generate_template(args),
        }
    }
}

fn generate_template(args: &TemplateArgs) -> Result<()> {
    let template = get_template(&args.template_type);

    if let Some(output_path) = &args.output {
        // Write to file
        let path = Path::new(output_path);

        if path.exists() && !args.force {
            eprintln!(
                "{}",
                format!("Error: File '{}' already exists. Use --force to overwrite.", output_path)
                    .red()
            );
            std::process::exit(1);
        }

        // Create parent directory if needed
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .context("Failed to create output directory")?;
            }
        }

        fs::write(path, template)
            .context("Failed to write template file")?;

        println!(
            "{}",
            format!("✓ Template written to {}", output_path).green()
        );
    } else {
        // Print to stdout
        println!("{}", template);
    }

    Ok(())
}

fn get_template(template_type: &TemplateType) -> String {
    match template_type {
        TemplateType::Epic => EPIC_TEMPLATE.to_string(),
        TemplateType::Task => TASK_TEMPLATE.to_string(),
        TemplateType::Bug => BUG_TEMPLATE.to_string(),
        TemplateType::Feature => FEATURE_TEMPLATE.to_string(),
        TemplateType::Chore => CHORE_TEMPLATE.to_string(),
    }
}

const EPIC_TEMPLATE: &str = r#"# [Epic Title]

## Overview

Brief description of the epic and its business value.

## Goals

- [ ] Goal 1
- [ ] Goal 2
- [ ] Goal 3

## Scope

### In Scope
- Feature/capability 1
- Feature/capability 2

### Out of Scope
- Items explicitly not included
- Deferred to future iterations

## User Stories

As a [user type], I want to [action] so that [benefit].

## Success Criteria

- Measurable outcome 1
- Measurable outcome 2
- Performance/quality metrics

## Tasks Breakdown

This epic should be broken down into:
- [ ] Task 1: [Description]
- [ ] Task 2: [Description]
- [ ] Task 3: [Description]

## Dependencies

- Dependency on [other epic/task]
- External dependency: [system/service]

## Timeline

- Phase 1: [Timeframe]
- Phase 2: [Timeframe]
- Target completion: [Date]

## Notes

Additional context, research, or references.
"#;

const TASK_TEMPLATE: &str = r#"# [Task Title]

## Description

Clear description of what needs to be done.

## Acceptance Criteria

- [ ] Criterion 1
- [ ] Criterion 2
- [ ] Criterion 3

## Implementation Notes

Technical details, approach, or considerations:
- Key point 1
- Key point 2

## Testing

- Unit tests: [What to test]
- Integration tests: [What to test]
- Manual verification: [Steps]

## Dependencies

- Blocked by: [task-id]
- Blocks: [task-id]

## Estimated Effort

[time estimate in hours/days]

## Resources

- Documentation: [links]
- Reference implementation: [links]
"#;

const BUG_TEMPLATE: &str = r#"# [Bug Title]

## Description

Clear description of the bug and its impact.

## Steps to Reproduce

1. Step 1
2. Step 2
3. Step 3

## Expected Behavior

What should happen.

## Actual Behavior

What actually happens.

## Environment

- OS: [e.g., macOS 13, Ubuntu 22.04]
- Browser/Tool: [e.g., Chrome 120, CLI v0.1.0]
- Version: [software version]

## Screenshots/Logs

```
[Paste relevant logs or error messages]
```

## Severity

- [ ] Critical - Blocks functionality
- [ ] High - Major impact
- [ ] Medium - Noticeable issue
- [ ] Low - Minor cosmetic issue

## Possible Fix

If you have ideas on how to fix:
- Root cause analysis
- Suggested approach

## Related Issues

- Related to: [issue-id]
- Duplicate of: [issue-id]
"#;

const FEATURE_TEMPLATE: &str = r#"# [Feature Title]

## Problem Statement

What problem does this feature solve?

## Proposed Solution

Describe the feature and how it addresses the problem.

## User Stories

As a [user type], I want to [action] so that [benefit].

## Requirements

### Functional Requirements
- [ ] Requirement 1
- [ ] Requirement 2
- [ ] Requirement 3

### Non-Functional Requirements
- Performance: [requirements]
- Security: [requirements]
- Accessibility: [requirements]

## Design

### UI/UX Mockups
[Links to designs or describe layout]

### API Design
```
[API endpoints, data structures, etc.]
```

## Implementation Plan

1. Phase 1: [Description]
2. Phase 2: [Description]
3. Phase 3: [Description]

## Testing Strategy

- Unit tests
- Integration tests
- User acceptance testing

## Rollout Plan

- Beta testing: [approach]
- Gradual rollout: [strategy]
- Monitoring: [metrics to track]

## Alternatives Considered

- Alternative 1: [Why not chosen]
- Alternative 2: [Why not chosen]

## Success Metrics

- Metric 1: [target]
- Metric 2: [target]
"#;

const CHORE_TEMPLATE: &str = r#"# [Chore Title]

## Description

Description of the maintenance, refactoring, or technical work.

## Motivation

Why is this work necessary?

## Tasks

- [ ] Task 1
- [ ] Task 2
- [ ] Task 3

## Impact

What systems/code will be affected?

## Testing

How to verify the changes don't break anything:
- [ ] Existing tests pass
- [ ] Manual verification steps

## Rollback Plan

If something goes wrong:
1. Step 1
2. Step 2

## Notes

Additional context or considerations.
"#;

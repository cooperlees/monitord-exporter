# AGENTS.md

Guidance for AI agents working in this repository. See `CLAUDE.md` for the
project overview, build commands, and code conventions.

## Dependency update PRs

Dependabot opens one PR per crate, but several of our dependencies are
version-locked to each other and **must be bumped together**. Individually,
each of those PRs fails CI (mismatched trait/type versions across crates).
This happens regularly, so when handling dependency PRs:

1. Look for coupled PRs among the open ones. Known coupled sets:
   - `opentelemetry`, `opentelemetry_sdk`, `opentelemetry-otlp` (same minor
     version) + `tracing-opentelemetry` (its minor tracks one ahead, e.g.
     otel 0.33 <-> tracing-opentelemetry 0.34)
   - `tonic` must match what `opentelemetry-otlp` depends on
   - `zbus` must match what `monitord` depends on
2. Combine them into a single PR: push all bumps onto one of the existing
   Dependabot branches, rename that PR to describe the whole set (e.g.
   "Update OpenTelemetry crates to 0.33"), and close the redundant PRs with a
   comment pointing at the combined one.
3. Build and test with `--all-features` (the OTel deps are behind the `otlp`
   feature, so a default build won't catch breakage).
4. Read the release notes / changelogs and adopt any new APIs or behavior
   changes that make sense here, or document why a default is fine.

---
type: state
observed_at: 2026-09-07
---
# Focused Seshat acceptance

Scope: the owner's Mac, source branch `codex/seshat-core-only`, local ad-hoc signed application. No public release or notarization is claimed.

## Source boundary

The application retains 149 Cargo workspace packages. 102 unreachable packages were removed, including the AI/provider/agent/MCP, collaboration UI/call/audio, remote workspace/server/container, terminal UI, debugger UI, REPL, extension host/installer, full workbench settings/sidebar and updater implementations. See [the removed-package inventory](removed-crates.json) and [the maintained boundary](../../../.lattice/spec/seshat-foundation.md). Shared editor/project protocol and schema abstractions remain, as documented there. Upstream deployment and repository-automation workflows were removed.

`script/seshat-verify` checks source/package absence and targeted application, Markdown, preview, read-only editor, Vim undo, Git staging and workspace-retention behavior. Relevant test findings: workspace retention must be independent of the removed Agent sidebar; opening a preview must focus its Markdown child so immediate Cmd+A/C works; preview session tests need separate databases because parallel tests reuse the same fixture paths.

## Native appearance

The title bar, toolbar and settings are focused on file editing. The file tree opens on the left; inline Git blame is off by default. Settings expose System/Light/Dark, text size, Vim, line wrapping and Git. Native preference toggles were observed and restored to 15 px, Vim on, wrapping off and Git on.

| Before | Seshat Dark settings | Seshat Light settings |
| --- | --- | --- |
| ![Before](before-core.png) | ![Dark](preferences-dark.png) | ![Light](preferences-light.png) |

## Native behavior

Final bundle interaction results are pending. The test fixture contains only task-created files in a temporary local Git repository with no remotes. Quit/relaunch checks use persistent state; a stateless launch cannot satisfy session acceptance.

## Limits

This keeps source editing and Markdown preview as separate surfaces. Typora-style in-place rich editing and bounded-memory loading of very large logs are still separate product work. Logs currently use the ordinary read-only text buffer. No Intel, Windows or Linux package execution is claimed. The original Zed installation and unrelated user documents are preserved.

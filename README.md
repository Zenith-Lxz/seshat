> [!IMPORTANT]
> Remove this line to confirm you've reviewed this PR before submitting.

# Seshat

A focused macOS editor for local files, built on Zed's editor and GPUI. Open a folder, edit text or Markdown, browse logs, and use Git when needed.

- Fast text editing with syntax colors, splits, undo and Vim.
- Markdown Edit/Preview modes with one source buffer, preserved undo, and rendered-text selection/copy.
- Read-only `.log` files and numeric rotated logs by default.
- Separate project and standalone file trees, an outline, bottom search (Cmd+F / Cmd+Shift+F), and optional Git changes/history/commits.
- Classic Seshat graphite and warm-paper themes, with a small settings panel for appearance, text size, Vim, line wrapping and Git.

AI providers and agents, collaboration UI, remote workspaces, containers, integrated terminal, debugger UI, notebooks, extension installation and upstream update infrastructure are removed from the application. Shared low-level editor interfaces are described in [the product contract](.lattice/spec/seshat-foundation.md).

## Local build

Use the installed mise Rust version pinned in `rust-toolchain.toml`:

```sh
mise exec rust@1.97.1 -- cargo check --locked -p zed
mise exec rust@1.97.1 -- script/seshat-verify
mise exec rust@1.97.1 -- script/seshat-bundle
```

The bundle is written to `target/seshat-bundle/Seshat.app`. Quit the running local Seshat before replacing it. The wrapper keeps the previous bundle as a ZIP backup and signs the new one locally. It does not install into Applications or publish a release. Configuration and session data are separate from Zed.

This macOS build is for local use; notarization and other-platform packages are separate work. In-place rich Markdown editing and a bounded-memory large-log viewer remain future work.

## Sources and license

Forked from [Zed](https://github.com/zed-industries/zed) `v1.18.1`, commit `bebe92f469834a287f5a57ed78e8d51a918b8ada`. Copyright notices and upstream attribution are preserved. The application is GPL-3.0-or-later; components marked Apache-2.0 retain that license. See [LICENSE-GPL](LICENSE-GPL), [LICENSE-APACHE](LICENSE-APACHE), and package manifests.

The original Swift application remains in the separate `seshat-reader` repository. Maintained Seshat rules, contracts, decisions and acceptance evidence are indexed in [the project knowledge map](.lattice/knowledge.md). The `docs/src` tree contains upstream reference documentation and may describe features removed from this fork.

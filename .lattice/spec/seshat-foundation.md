---
type: contract
---
# Seshat foundation

- App display name is Seshat; the stable macOS bundle identifier is `dev.seshat.editor`. Its configuration, state, caches, logs and instance handshake are separate from Zed.
- Keep real session persistence enabled. A stateless launch is only an experiment and cannot pass session acceptance.
- Start directly in the editing workspace. Vim is enabled by default and remains configurable. AI, language servers, telemetry and automatic updates are disabled by default. Save does not automatically format, trim trailing spaces or add a final newline.
- The fork never polls for or installs an upstream Zed update, including when a local setting enables automatic updates. No upstream account authentication is started during launch.
- `.log`, case variants and numeric rotated logs open read-only through the existing `read_only_files` setting. The owner can explicitly override that setting. This is ordinary text-buffer loading, not a large-log streaming viewer.
- Markdown preview receives Cmd+A and Cmd+C as native keyboard actions and copies rendered text across blocks. The operation leaves the Markdown source unchanged.
- Preserve ordinary text editing, upstream Vim behavior, undo, search, file watching and explicit saves. Recheck the native journey after changing defaults or shell presentation.

## Build and acceptance

Use the toolchain pinned by `rust-toolchain.toml` through mise. Cargo.lock is authoritative. `cargo build --locked -p zed` is the development build; `script/seshat-bundle` builds release-fast and produces an ad-hoc signed `target/seshat-bundle/Seshat.app`. Its version currently records the upstream base; no public Seshat release is declared.

The local wrapper does not install Cargo plugins, alter keychains, install into Applications, upload artifacts or bundle Git. Git features use Git already present on the host. A clean source-built app must be verified with native keyboard interaction, disk readback and a quit/relaunch session check. CLI tests and code signing are separate evidence.

This foundation does not accept full WYSIWYG Markdown, specialized huge-log handling, a completely stripped dependency graph, notarization, Intel execution or cross-platform packages. Those need their own implementation and acceptance.

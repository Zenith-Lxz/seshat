---
type: contract
---
# Seshat foundation

- App display name is Seshat; the stable macOS bundle identifier is `dev.seshat.editor`. Its configuration, state, caches, logs and instance handshake are separate from Zed.
- Keep real session persistence enabled. A stateless launch is only an experiment and cannot pass session acceptance.
- Start directly in the editing workspace. Vim is enabled by default and remains configurable. AI application services, language-server adapters, telemetry collection/upload and automatic updates are removed. Save does not automatically format, trim trailing spaces or add a final newline. Per-language Prettier integration is disabled by default so merely opening Markdown does not preinstall its formatter.
- The fork never polls for or installs an upstream Zed update, including when a local setting enables automatic updates. No upstream account authentication is started during launch.
- `.log`, case variants and numeric rotated logs open read-only through the existing `read_only_files` setting. The owner can explicitly override that setting. This is ordinary text-buffer loading, not a large-log streaming viewer.
- Markdown preview receives Cmd+A and Cmd+C as native keyboard actions and copies rendered text across blocks. The operation leaves the Markdown source unchanged.
- Preserve ordinary text editing, upstream Vim behavior, undo, search, file watching and explicit saves. Recheck the native journey after changing defaults or shell presentation.

## Build and acceptance

Use the toolchain pinned by `rust-toolchain.toml` through mise. Cargo.lock is authoritative. `cargo build --locked -p zed` is the development build; `script/seshat-bundle` builds release-fast and produces an ad-hoc signed `target/seshat-bundle/Seshat.app`. Its version currently records the upstream base; no public Seshat release is declared.

The local wrapper does not install Cargo plugins, alter keychains, install into Applications, upload artifacts or bundle Git. Git features use Git already present on the host. A clean source-built app must be verified with native keyboard interaction, disk readback and a quit/relaunch session check. CLI tests and code signing are separate evidence.

The wrapper refuses to replace a running local application. Prior generated bundles are kept as ZIP backups, so duplicate application identities do not interfere with native app discovery.

This foundation does not accept full WYSIWYG Markdown, specialized huge-log handling, notarization, Intel execution or cross-platform packages. Those need their own implementation and acceptance.

## Focused application boundary

The application only initializes local editing, syntax grammars, file browsing, outline/search, previews, native preferences and Git UI. AI providers/agents/MCP, collaboration UI/calls/audio, remote workspace UI/server, dev containers, integrated terminal UI, debugger UI, notebooks/REPL, extension host/installer and upstream deployment workflows are removed from source and Cargo workspace membership. `script/seshat-verify` checks that the removed source packages cannot return as transitive dependencies.

The existing editor and project models retain shared LSP, DAP, remote transport, terminal data and collaboration protocol abstractions. These support upstream buffer/editor interfaces and tests; their presence is not an exposed workbench feature. Built-in languages register syntax grammars without LSP adapters or extension installers. Shared settings and authentication schemas retain model and usage data types (`language_model_core`, `cloud_llm_client`); LLM token refresh and authenticated model requests are removed. The shared client retains empty telemetry identity hooks; there is no event collector, upload timer or installation identifier creation.

Settings cannot restore removed implementations. Legacy workbench shortcuts are filtered, shipped keymaps omit removed actions, and remote/container/agent/extension links fail explicitly. Local-file links and deliberate Git URLs remain supported. Upstream documents and schemas may retain reference vocabulary for compatibility.

The new native preferences panel writes through the existing settings store and exposes appearance (System/Light/Dark), text size, Vim, wrapping and Git. Seshat Light/Dark are neutral variants of the attributed One themes. The title bar contains only file opening, the folder name and the file panel control; status and toolbar surfaces retain editing and preview functions.

Local workspace retention is independent of sidebar availability. Removing the Agent sidebar must not discard a previous workspace, skip dirty-buffer prompts, or lose restored sessions.

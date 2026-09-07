---
type: contract
---
# Seshat foundation

- App display name is Seshat; the stable macOS bundle identifier is `dev.seshat.editor`. Its configuration, state, caches, logs and instance handshake are separate from Zed.
- Keep real session persistence enabled. Normal Quit retains unsaved buffers for restoration; closing an individual dirty tab asks whether to save, discard or cancel. A stateless launch is only an experiment and cannot pass session acceptance.
- Start directly in the editing workspace. Vim is enabled by default and remains configurable. AI application services, language-server adapters, telemetry collection/upload and automatic updates are removed. Save does not automatically format, trim trailing spaces or add a final newline. Per-language Prettier integration is disabled by default so merely opening Markdown does not preinstall its formatter.
- The fork never polls for or installs an upstream Zed update, including when a local setting enables automatic updates. No upstream account authentication is started during launch.
- `.log`, case variants and numeric rotated logs open read-only through the existing `read_only_files` setting. The owner can explicitly override that setting. This is ordinary text-buffer loading, not a large-log streaming viewer.
- Markdown preview receives Cmd+A and Cmd+C as native keyboard actions and copies rendered text across blocks. The operation leaves the Markdown source unchanged.
- Preserve ordinary text editing, upstream Vim behavior, undo, search, file watching and explicit saves. Recheck the native journey after changing defaults or shell presentation.

## Build and acceptance

Use the toolchain pinned by `rust-toolchain.toml` through mise. Cargo.lock is authoritative. `cargo build --locked -p zed` is the development build; `script/seshat-bundle` builds release-fast and produces an ad-hoc signed `target/seshat-bundle/Seshat.app` (under `CARGO_TARGET_DIR` when set). Its version currently records the upstream base; no public Seshat release is declared.

The local wrapper does not install Cargo plugins, alter keychains, install into Applications, upload artifacts or bundle Git. Git features use Git already present on the host. A clean source-built app must be verified with native keyboard interaction, disk readback and a quit/relaunch session check. CLI tests and code signing are separate evidence.

The wrapper refuses to replace a running local application. Prior generated bundles are kept as ZIP backups, so duplicate application identities do not interfere with native app discovery.

This foundation does not accept full WYSIWYG Markdown, specialized huge-log handling, notarization, Intel execution or cross-platform packages. Those need their own implementation and acceptance.

## Focused application boundary

The application only initializes local editing, syntax grammars, file browsing, outline/search, previews, native preferences and Git UI. AI providers/agents/MCP, collaboration UI/calls/audio, remote workspace UI/server, dev containers, integrated terminal UI, debugger UI, notebooks/REPL, extension host/installer and upstream deployment workflows are removed from source and Cargo workspace membership. `script/seshat-verify` checks that the removed source packages cannot return as transitive dependencies.

The existing editor and project models retain shared LSP, DAP, remote transport, terminal data and collaboration protocol abstractions. These support upstream buffer/editor interfaces and tests; their presence is not an exposed workbench feature. Built-in languages register syntax grammars without LSP adapters or extension installers. Shared settings and authentication schemas retain model and usage data types (`language_model_core`, `cloud_llm_client`); LLM token refresh and authenticated model requests are removed. The shared client retains empty telemetry identity hooks; there is no event collector, upload timer or installation identifier creation.

Settings cannot restore removed implementations. Legacy workbench shortcuts are filtered, shipped keymaps omit removed actions, and remote/container/agent/extension links fail explicitly. Local-file links and deliberate Git URLs remain supported. Upstream documents and schemas may retain reference vocabulary for compatibility.

The initial settings file contains only explicit overrides, so the first preference change cannot reset unrelated theme or font defaults. The new native preferences panel writes through the existing settings store and exposes appearance (System/Light/Dark), text size, Vim, wrapping and Git. Seshat Light/Dark use the classic Seshat graphite and warm-paper palettes while retaining attribution for the original theme structure. The title bar shows file/folder opening, the active file name, dirty state and parent path. The sidebar exposes File/Outline navigation. Tabs remain available as a preference but are hidden by default. System UI typography and compact Menlo text match the original application.

Local workspace retention is independent of sidebar availability. Removing the Agent sidebar must not discard a previous workspace, skip dirty-buffer prompts, or lose restored sessions.

## Classic file workflow

Cmd+F opens the bottom search panel for the current source document; Cmd+Shift+F opens it for all project roots and standalone files. The same panel offers current file, all files, opened files and opened logs. Opened scopes derive from workspace items, so a buffer loaded by a previous search does not become an opened file. Searches use unsaved buffers, respect the project search engine's file exclusions, and run explicitly on Return or Search. Options control case, whole words and regex. Empty/invalid/canceled/replaced queries cannot contribute obsolete results; results with no ranges are omitted. Results are grouped by file and activate anchored source positions. Removing a root cancels the pending search and removes that root's result groups.

Markdown has explicit Edit and Preview modes inside the same Editor item. Entering Preview takes the current source snapshot; the rendered view holds no reference back to its owning editor. Returning to Edit restores the same buffer, cursor, scroll and undo state. Preview selection/copy leaves source bytes unchanged. Ordinary open-preview actions use this mode; legacy independent/split preview items remain readable for session compatibility. In-place or continuous live Markdown editing is not an outstanding requirement after the owner's 2026-09-07 change of direction.

The sidebar has separate project and standalone trees. Project roots keep the upstream virtualized navigation model; standalone roots use their own virtualized list and close control. Untitled buffers are also reachable in the standalone section. Closing a standalone root first resolves dirty saves before removing any matching view, then closes matching workspace items and rechecks all panes and worktree identity before removing the root. Concurrent close requests for the same root are coalesced. Cancellation retains every split view, the buffer and the root. The operation does not delete the file. Project roots and their children have no close cross. File names alone never identify a close target.

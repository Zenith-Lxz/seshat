---
type: state
observed_at: 2026-09-06T21:38:30.829888+00:00
---
# Native foundation acceptance

Source and packaged application: `8ab8deafd7ffa7dfe428ac2e3643c1bb31201bab`. The application is built from the Zed v1.18.1 fork, using the release-fast profile on this arm64 Mac. The About window and clean build receipt agree on the commit. The application is locally ad-hoc signed and was not notarized or installed over the older Swift application.

## Observed journeys

1. Open the task-owned file workspace. Keep Restricted Mode; no project trust or new permissions were granted.
2. In `daily.txt`, Vim `ggdd` then Cmd+S removed only the first line. `u` then Cmd+S restored exact UTF-8 bytes, including Chinese, an emoji with modifier, trailing spaces and the absence of a final newline.
3. Append `最终源码验收：8ab8dea。` to Markdown in Insert mode, save, and inspect its existing preview. The new text rendered. Native preview Cmd+A/C followed by paste/save in the previously empty text target copied the heading, inline text, list, quote, all table cells, code and final marker. Undo in the source editor and save restored the original Markdown bytes.
4. Attempt deletion, insertion and save in `app.log`, and deletion/save in `archive.LOG.1`. Both original hashes remained unchanged. The tabs show a lock. The final source build does not show extension installation recommendations.
5. Quit normally; confirm the process exits. Launch the same bundle again with no stateless flag. The workspace, six tabs and selected rotated log were restored. Restricted Mode confirmation still appears on opening this untrusted workspace; it remains an upstream gate.
6. Zed and Seshat both responded through their real native windows while running together. The existing Zed settings file hash remained unchanged. Seshat created its own config, data, logs and cache directories.
7. A clean Seshat profile was used after preserving this task's earlier profile. Opening the fixture set left zero files in the Node scratch, Prettier and downloaded-language directories. This is a check of these journeys, not a claim of general network isolation.

## Regression evidence and limits

The Markdown keyboard test failed with an empty clipboard before the SelectAll action handler was added. Earlier tests created selection ranges directly and therefore missed the missing keyboard route. The full Markdown suite has 156 tests; release-channel tests include the upstream updater guard. This is not the whole upstream test suite.

Native QA also exposed per-language Prettier defaults overriding the intended product behavior. Those integrations are now opt-in. The bundle wrapper previously moved a running app, which broke native-tool app discovery; a process sample showed its main thread waiting for events. The wrapper now refuses a running bundle, rechecks before replacement, and keeps prior output as ZIP instead of duplicate app bundles. The refusal was exercised and happened before compilation/replacement.

The computer-use paste helper once reported a clipboard-read timeout after text had already been inserted. The visible buffer was inspected before proceeding, so the text was not pasted twice. This workflow verifies Unicode paste and save; physical Chinese IME composition is not accepted here.

The optimized build emits upstream future-compatibility and linker unwind-size warnings, retained in its log. No startup-latency, large-file throughput, Intel, Windows, Linux or notarization acceptance is claimed. Typora-style in-place writing and a bounded streaming log viewer remain product work; this increment retains separate Markdown source and preview surfaces and ordinary text-buffer log loading.

## Files

- `native-acceptance.json`: disk checks, profile isolation and native journey results.
- `build-receipt.json`: clean source commit, profile and bundle identity.
- `markdown-keyboard-before.log`: failing keyboard regression before the fix.
- `foundation-tests.log`: bounded automated suite.
- `about-seshat.png`: actual application identity and source attribution.
- `markdown-preview.png`, `copied-text.png`, `copied-text.txt`: the rendered-copy journey in the current build.
- `restored-read-only-log.png`: the same build after normal quit and relaunch.

---
type: intent
---
# Build Seshat on Zed

The owner selected an existing editor base on 2026-09-07 after the Swift implementation repeatedly failed daily editing acceptance. The primary job is opening local text, editing confidently with ordinary keys or Vim, writing Markdown, and reading logs. Optional Git stays useful; AI, a code graph and an IDE workbench are outside the desired daily flow.

## Decision

Fork [Zed](https://github.com/zed-industries/zed) and start from stable `v1.18.1` (`bebe92f469834a287f5a57ed78e8d51a918b8ada`). Keep upstream buffer, editor, undo, Vim, search, Git and GPUI code. Apply a small product patch set to defaults, identity and directly observed workflow gaps. Internal crate names remain upstream-compatible. The old Swift repository is retained independently.

Fork: [Zenith-Lxz/seshat](https://github.com/Zenith-Lxz/seshat). Local product branch: `seshat`; official source remote: `upstream`. The GitHub fork's imported main branch is upstream provenance; product changes are published on a separate branch.

## Alternatives considered

| Base | Fit and cost for this product |
| --- | --- |
| Zed | The best fit for the central text/code editor and Vim experience. An existing native macOS application can be retained. Its broad dependency graph and AI/IDE modules remain a maintenance and binary-size cost even when their entry points are hidden. |
| [Kate / KTextEditor](https://invent.kde.org/frameworks/ktexteditor) | Mature reusable editor framework, but the product would acquire a Qt/KDE C++ application stack and a different macOS integration path. Prefer it if a reusable cross-platform editor component becomes more important than retaining Zed's interaction. |
| [Fresh](https://github.com/sinelaw/fresh) | Its terminal-first product and early desktop surface do not provide the mature native-window starting point requested here. |
| [ColaMD](https://github.com/marswaveai/colamd), [MarkText](https://github.com/marktext/marktext) | Useful Markdown editing references, but a Markdown-oriented application is a weaker base for arbitrary text, code, Vim and large logs. |
| [MiaoYan](https://github.com/tw93/MiaoYan), [Yank Note](https://github.com/purocean/yn) | Useful Markdown reading and workbench references. Their current product focus does not replace a general editor foundation. |

## Boundaries

Zed's Markdown preview is a separate rendered view. Choosing Zed does not deliver Typora-style in-place writing. Its normal file buffer also does not establish bounded loading, tailing and filtering of very large logs. These remain explicit product work with their own real-window acceptance.

Do not copy another project's whole editor into GPUI to close those gaps without a focused prototype and a source-preservation test. Markdown remains authoritative text. Any future block-based writing layer must retain undo, cursor stability, IME, copy semantics and unchanged source outside the user's edit.

The Zed application/editor code remains GPL-3.0-or-later; GPUI's separate license does not relicense the application. Retain upstream attribution and source history.

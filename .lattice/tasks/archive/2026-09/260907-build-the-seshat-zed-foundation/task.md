---
id: 260907-build-the-seshat-zed-foundation
title: Build the Seshat Zed foundation
status: done
owner: codex
branch: codex/seshat-zed-foundation
worktree: 
tree: .
verify: mise exec rust@1.97.1 -- cargo test --locked -p markdown -p release_channel && mise exec rust@1.97.1 -- cargo build --locked --profile release-fast -p zed
verified: pass 2026-09-07T05:40:52+08:00 @8ab8deafd7ffa7dfe428ac2e3643c1bb31201bab
created: 2026-09-07T04:37:21+08:00
claimed: 2026-09-07T04:37:44+08:00
done: 2026-09-07T05:40:52+08:00
---

## Goal

Create the first source-built Seshat application on the owner's Zed fork, preserving upstream editing behavior while giving the product independent identity and focused defaults. Fix the previously observed Markdown keyboard select-all/copy failure. This foundation is one increment toward the broader daily-editor product; WYSIWYG writing and specialized large-log behavior remain explicit subsequent acceptance work.

## Acceptance

- [x] Source builds from pinned Zed 1.18.1 with the existing mise-managed Rust toolchain and Cargo.lock.
- [x] Seshat app identity and local data are separate from installed Zed; session persistence works without stateless mode.
- [x] AI/IDE entry points are reduced and upstream automatic replacement is disabled by product defaults.
- [x] Markdown preview Cmd+A/C copies the complete rendered document, including cross-block Unicode text, through a regression and a packaged native-window journey.
- [x] Native file editing, Vim undo/save and Markdown preview remain operable.
- [x] A locally signed application and source/validation handoff are available. Do not claim Typora or log-viewer parity from this increment.

## Notes / Dead ends

Original Swift implementation remains in ../seshat-reader. The user explicitly authorized choosing and forking an existing editor on 2026-09-07. GitHub fork: Zenith-Lxz/seshat; upstream: zed-industries/zed. Product base: v1.18.1 / bebe92f469834a287f5a57ed78e8d51a918b8ada. Avoid running upstream script/bundle-mac unchanged: it can install a global cargo-bundle plugin and enter upstream signing/deployment workflows. A local-only bundle wrapper should use declared Cargo builds and ad-hoc signing.

## Handoff

The source-built foundation passed the bounded automated checks and native journeys. Final application source: 8ab8dea; bundle: target/seshat-bundle/Seshat.app. Evidence and limitations are in evidence/README.md and evidence/native-acceptance.json.

The original Swift source remains in ../seshat-reader. Continue product work in this Zed fork. In-place Markdown writing, bounded large-log loading/tailing/filtering, deeper removal of IDE dependencies, full IME/performance acceptance, and distribution gates remain outside this completed foundation increment. The application currently records upstream version 1.18.1 and is about 358 MiB; no public binary release is declared.

The owner-authorized source destination is Zenith-Lxz/seshat on the seshat product branch. Git refs record source synchronization; the application receipt retains its exact source commit. Keep generated apps closed before invoking script/seshat-bundle; it deliberately refuses to replace a running application. The source checkout may be used to build with Cargo while the existing application runs, because that does not replace the bundle.

---
type: contract
---
# Seshat fork

Seshat is a daily file editor forked from Zed. Read `.rules` for upstream Rust and GPUI coding instructions; preserve that upstream source. The owner explicitly selected an existing editor base instead of continuing the Swift implementation in `../seshat-reader`.

- Rust/Cargo and the pinned `rust-toolchain.toml` are this repository's build contract. Use the installed mise runtime matching that file. The Swift-only rules in the older repository do not apply here.
- Preserve upstream editor, buffer, undo, Vim, search and Git implementations. The owner authorized removal of unused workbench implementations. Remove their application integration and unreachable source crates together; preserve shared editor abstractions and avoid renaming internal Rust crates.
- Product scope: directly edit ordinary local text and code; offer Markdown writing/reading and read-only logs. No code graph, AI assistant or IDE workbench is required.
- Keep Seshat app identity, configuration, session databases, logs and updates separate from Zed. Never inherit an upstream updater that can replace Seshat with Zed.
- Keep source Markdown authoritative. Rendered selection/copy and source editing are separate observable journeys; neither a screenshot nor a parser test proves both.
- Existing files and dirty buffers must survive external edits, rename and save conflicts. Do not treat Markdown serialization round trips as exact-source preservation without tests.
- Checks: `script/seshat-verify` checks the application boundary and targeted regressions. `cargo check --locked -p zed`, `cargo build --locked -p zed`, targeted `cargo test --locked -p <crate>`, and `./script/clippy` for Clippy. `script/seshat-bundle` builds a local macOS application with the release-fast profile and ad-hoc signing; run it through the pinned mise runtime. Local bundle and native-window acceptance remain separate gates.
- Keep upstream licensing and attribution. The application/editor is GPL-3.0-or-later; GPUI's separate Apache license does not relicense the application.

<!-- lattice:protocol:start -->
## Lattice Protocol (for all agents)

- Session start: `lattice context` is injected by Claude Code and Codex hooks.
  `init` preserves this machine's selected hook mode (project by default;
  `init --hooks user` enrolls in an explicitly installed shared entrypoint).
  Native hook trust still applies. If no bundle appears, run
  `python3 .lattice/lattice context`. It prioritizes the current task; AGENTS.md
  is loaded natively and is not repeated. Reconcile with actual git state.
  Read `.lattice/knowledge.md` for the project's maintained sources. Before
  declaring prior information missing, use `lattice find "<term>"` to search
  docs, shared task notes, and archives; an archive is historical evidence.
- Before material work: `task new` + `task claim`. State the goal, observable
  acceptance, and a real verify command. Trivial edits and read-only work need no
  task. Add a plan only when dependencies or uncertainty make one useful; no
  mandatory phases or review committee. Split parallel work at stable interfaces.
- One task = one branch = one verifiable commit series. Claim creates a branch
  from main (or `git config lattice.main`) and switches the primary tree;
  `--worktree` creates `.lattice/worktrees/<id>` for an independent session.
  Never share a working tree between sessions. `--force` explicitly takes over
  a claim; it cannot adopt a mismatched checkout. `task release` keeps the branch.
- Active tasks, notes, and evidence live in the primary tree's ignored
  `.lattice/runtime/tasks/`. All worktrees share this local board across branch
  switches; it is not replicated by git. Read/edit the task path printed by the
  CLI. `spec/` and `intent/` remain versioned; archive moves the task and evidence
  into `.lattice/tasks/archive/` to commit. Old versioned active task files are
  migration provenance, never the live board. Run `init` after upgrading.
- What the system does now: code + tests > git history > docs. What it should do:
  the user's request, task goal/acceptance, and valid specs. Resolve conflicts
  explicitly; fix code or contract in the same change.
- Follow the repository's authorization limits for commits and merges; this
  protocol grants no additional permission. Within those limits, commit each
  verifiable increment. `task done` checks the checkout and binds verification
  to its commit and contract; failed or stale evidence cannot authorize archive.
  `--evidence` records an explicit manual observation, not an automated test.
- Merge into main, then run `task archive` from the primary tree on main. Archive
  requires fresh task evidence and clean trees; when the merged content differs,
  it runs verification there too. Manually verified changes need fresh
  `archive --evidence` for a different merged tree. Failure retains the task and
  worktree; success removes the worktree/merged branch and prints consolidation.
- Promote stable behavior to `.lattice/spec`, reasoning and real alternatives to
  `.lattice/intent`, and preconditions/pitfalls to AGENTS.md. Keep transient
  progress and dead ends with the active task. Preserve only useful decisions.
  Before pausing or finishing, reconcile the handoff with the final diff and
  observed acceptance: remaining work, constraints, rejected approaches, changed
  operational prerequisites, and evidence limits. Link maintained sources from
  `.lattice/knowledge.md`; update their contents, not only the index. Date live
  observations and name their source and recheck method. Provider-private memory
  is not shared project knowledge. Store credential locations only, never values.
- Reuse build caches where supported; remove disposable task outputs at completion.
  Measure framework changes on real tasks using delivery time, verification
  retries, human intervention, and rework; a passing smoke or doctor is not user
  acceptance. The agent chooses how to plan, implement, and verify.
<!-- lattice:protocol:end -->

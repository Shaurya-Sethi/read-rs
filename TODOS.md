# TODOS

This file captures planned work for read-rs. It focuses on modularizing the codebase, adding tests, and implementing a set of user-facing features (UTF-8 validation, multiple files, stdin fallback, line-numbering and whitespace options).

## High-level checklist

- [x] Write this TODO list (current)
- [ ] Modularize codebase (split `main.rs` into modules/functions)
- [ ] Add tests (unit + integration)
- [ ] Implement additional features (detailed below)

---

## 1) Modularize codebase

Goal: reduce responsibility in `main.rs` by extracting parsing, IO, and printing logic into small, testable modules.

subtasks:

- Create `lib.rs` with public functions: `run(args: impl Iterator) -> Result<(), Error>` and smaller helpers for validation/printing.
- Add modules: `args` (argument parsing and validation), `io` (file/stdin reading + UTF-8 validation), `printer` (formatting, numbering, whitespace handling), `errors` (common error types/messages).
- Keep `main.rs` thin: parse args, call `run`, map errors to exit codes and messages.

Acceptance criteria:

- `main.rs` contains only argument parsing, a call to `run`, and error -> exit handling.
- All new modules have unit tests for core logic.

Priority: High

---

## 2) Add tests

Goal: add a minimal but useful test suite (unit + integration) to protect behavior and allow refactors.

Suggested tests:

- Unit tests for UTF-8 validation helper (valid/invalid bytes).
- Unit tests for `printer` behaviors: numbering, show-tabs, mark line ends, squeeze blank lines.
- Integration tests using test files placed in `tests/` for: multiple files, stdin fallback, head/tail/line/range flags, error handling when requested lines exceed file length.
- CLI smoke test: run the built binary with example args and assert exit codes and stdout.

Acceptance criteria:

- Tests run with `cargo test` and include at least one unit and one integration test.

Priority: High

---

## 3) Additional features (detailed)

All features MUST validate input as UTF-8 before printing. When validation fails for any requested input (file or stdin), exit with a clear error and non-zero exit code. When a requested line/range is beyond file length, exit with a clear error.

Feature list & acceptance criteria:

- Support multiple files
	- Allow multiple file args: `read-rs file1 file2 ...`
	- Validate each file for UTF-8 before output.
	- On any validation failure, stop and exit non-zero with an explanatory message.

- Stdin fallback
	- If no file paths provided, read from stdin and validate UTF-8 before output.
	- Works in pipelines: `echo hello | read-rs`.

- Line numbering option
	- Flag: `-n` / `--number` to print line numbers.
	- Only applies to valid UTF-8 text lines.
	- Format: right-aligned numbers with a single tab or two-space separator (decide impl-consistent formatting).

- Whitespace handling options
	- `--squeeze-blank` : collapse consecutive blank lines into one.
	- `--show-tabs` : render `\t` as a visible glyph (e.g., `→` or `^I`). Choose one glyph and document it.
	- `--mark-ends` : append a visible symbol (e.g., `$`) to the end of each line.

- Line selection options
	- `--line <N>` : print a single line N (1-based). Validate N > 0 and N <= total lines, else error.
	- `--lines <A>-<B>` : print inclusive range. Validate bounds and order.
	- `--head <N>` : print first N lines. If N > total lines, print whole file (or treat as error? -- choose to print whole file but document behaviour). (Assumption: print whole file.)
	- `--tail <N>` : print last N lines. If N > total lines, print whole file.

Notes on behaviour and conflicts:

- If options conflict (e.g., `--line` and `--lines`), prefer the most specific or fail with usage error. (Assumption: treat as error and ask user to choose one.)
- UTF-8 validation must happen before printing; for range requests validate that the underlying bytes are valid UTF-8 and that requested lines exist. If invalid UTF-8 is found anywhere in the requested region or file when printing whole-file, exit with error.

Priority: Medium–High
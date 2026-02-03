# clean-tts-text

A tiny Rust CLI that normalizes text before passing it into an XTTS-style TTS engine so narration stays clean, paced, and predictable. The tool unwraps hard line wraps, removes citation noise, flattens list markers, expands troublesome acronyms, and applies other tiny transformations that can be the difference between multi-second gaps and a smooth read.

## Usage

1. Configure the policy in `config.toml` (a sensible default is already checked into the repo).
2. Run the cleaner over any plain text file and write the normalized version:

```bash
cargo run -- --input examples/matrix.txt --output cleaned/matrix-tts.txt
```

3. Feed the resulting file (`cleaned/matrix-tts.txt` in the example above) to your TTS pipeline. If you want to override the config, add `--config path/to/custom.toml`.

The CLI emits a short summary of bytes and paragraph counts; set `LOG_LEVEL=debug` (or change `logging.level` in the config) for more diagnostics.

## Configuration

`config.toml` is organized into sections that reflect the cleaning stages:

- `[io]` controls newline normalization and whether paragraphs are collapsed to one line per paragraph (recommended for audiobook engines).
- `[unicode]` normalizes punctuation (NFKC plus dash/ellipsis handling) so the model does not invent dramatic pauses.
- `[structure]` determines how wrapped lines are joined and which blank-line patterns mark paragraph boundaries.
- `[markdown]` and `[citations]` strip code fences, inline backticks, markdown links, and numeric footnotes/brackets.
- `[lists]` replaces bullets with commas to avoid choppy readings of enumerations.
- `[abbreviations]` and `[pronunciation]` expand acronyms (e.g. `CSS` → `C S S`) and apply small sentence-friendly replacements.
- `[whitespace]`, `[guardrails]`, and `[experimental]` govern spacing collapses, warning thresholds, and optional punctuation-ray trimming.

Each section is fully documented inside `config.toml` so you can adjust the behavior before running the CLI.

## Example data

`examples/matrix.txt` contains a historical narrative with hard line wraps, citations, and dash-heavy sentences—great for testing that the cleaner removes slit-worthy silence without killing the story.

Use `cargo fmt` to keep the code tidy, and `cargo clippy`/`cargo test` if you add helpers later.

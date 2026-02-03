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
- `[unicode]` normalizes punctuation (`normalization = "nfkc"` by default, but `nfc`/`none` work too) and tame dash/ellipsis handling so the model does not invent dramatic pauses.
- `[structure]` determines how wrapped lines are joined and which blank-line patterns mark paragraph boundaries.
- `[markdown]` and `[citations]` strip code fences, inline backticks, markdown links, and numeric footnotes/brackets.
- `[lists]` replaces bullets with commas to avoid choppy readings of enumerations.
- `[abbreviations]` and `[pronunciation]` expand acronyms (e.g. `CSS` → `C. S. S.` by default) and apply small sentence-friendly replacements; the cleaner now appends digits (so `CSS1` becomes `C. S. S. 1`).  
- `pronunciation.version_mode = "say-decimal"` lets you speak `1.0` as “one point zero,” `2.3.4` as “two point three point four,” etc., while `[number]` controls how the spelled-out components are joined (no commas by default) and whether the noisy “and” appears in years, plus `[selector]` prefixes selectors such as `.card` with “dot” while `[abbreviations]` exposes `letter_separator` and `digit_separator` so digits can be syllabified as “dot 1” without a pause.  
- `[pronunciation]` now also supports brand-specific spellings (MySQL, SQLite, PostCSS, W3C, JSSS, IE4), American year pronunciation (1992 → “one thousand nine hundred ninety two”), and HTML tag handling that spells just the opening tag and drops closing tags. The relevant options live under `pronunciation.brand-map`, `year_mode`, `number`, `abbreviations.letter_separator`, `selector`, and `html_tag_pronunciation`.  
- `[punctuation]` now lets you replace `/` with text (default “ or ”), collapse stop sequences (`,:` or `.,` → whichever stop you prefer via `stop_precedence`), and re-collapse whitespace so repeated spaces become single spaces.  
- `[whitespace]`, `[guardrails]`, and `[experimental]` govern spacing collapses, warning thresholds, and optional punctuation-ray trimming.

Each section is fully documented inside `config.toml` so you can adjust the behavior before running the CLI.

## Example data

`examples/matrix.txt` contains a historical narrative with hard line wraps, citations, and dash-heavy sentences—great for testing that the cleaner removes slit-worthy silence without killing the story.

Use `cargo fmt` to keep the code tidy, and `cargo clippy`/`cargo test` if you add helpers later.

## Alternative profile

`config-expressive.toml` keeps paragraph spacing, uses hyphen-based dashes, dotted acronym spelling, and drops code fences with a spoken placeholder. Run it with:

```bash
cargo run -- --config config-expressive.toml --input examples/matrix.txt --output cleaned/matrix-tts-expressive.txt
```

Inspect `cleaned/matrix-tts-expressive.txt` to compare how the same source behaves under a slightly more expressive normalization pipeline.

Skip to content
Chat history
You said:
im trying to generate audio with ebook2audiobook with daisy studio voice. i think it uses xtts for audio tts generation. help me configure the settings for xtts. these are the default settings. but i dont get why speed is 0.5. should it be 1 for normal rate?
ChatGPT said:

Yeah — in most XTTS/Coqui-style setups, speed = 1.0 is the “neutral / normal” rate, and values below/above that slow down / speed up.

So if your UI is showing 0.5 by default, that usually means “half speed” (very slow) unless the app is using a different internal mapping (some front-ends label a “length scale” as “speed,” or they pick a slow default because audiobook narration sounds better a bit slower).
What you should do for audiobooks

Use your ears + a quick calibration:

    Pick a short paragraph (100–200 words).

    Generate audio at speed 0.5, 0.8, 1.0, 1.1.

    Choose the one that’s closest to a comfortable audiobook pace.

Typical narration targets are roughly 150–180 words/min (varies by genre and your preference). If it sounds “draggy,” go up toward 0.85–1.05.

My practical recommendation:

    Start at speed = 0.9 for non-fiction / clear narration

    speed = 1.0 for normal pacing

    speed = 1.05–1.15 if it feels sluggish or if the voice is naturally slow

If you set it to 1.0 and it suddenly becomes too fast, then this UI is probably using an inverted/renamed parameter — but that’s easy to confirm with one 10-second test.
How the other XTTS sampling knobs affect narration

These do not usually change the words (you’re not asking it to paraphrase), but they can change prosody, pauses, emphasis, and sometimes weird artifacts.

Your defaults (from the screenshot) are extremely “locked down”:

    Temperature = 0.05 (very deterministic/flat)

    Top-k = 10 (very restrictive)

    Top-p = 0.1 (very restrictive)

    Repetition penalty = 1 (neutral)

That combo often produces monotone speech and can sometimes make the model “stick” into odd cadence patterns.
Two good presets to try

Preset A — “Clean audiobook” (stable, not too robotic)

    Temperature: 0.15–0.25

    Top-p: 0.4–0.7

    Top-k: 40–80

    Repetition penalty: 1.05–1.15 (optional; only if you notice repeated phrases/cadence loops)

    Speed: 0.9–1.0

Preset B — “Expressive fiction” (more emotion/variation)

    Temperature: 0.3–0.45

    Top-p: 0.75–0.95

    Top-k: 80–150

    Repetition penalty: 1.0–1.1

    Speed: 0.95–1.05

Quick troubleshooting rules

    Too monotone / robotic: increase temperature first, then top-p.

    Random weird emphasis / unstable delivery: lower temperature and/or lower top-p.

    It rushes through punctuation: reduce speed slightly (e.g. 0.9) and consider nudging temperature up a bit (counterintuitively, ultra-low temp sometimes flattens punctuation timing).

If you tell me whether 0.5 sounds slow or fast in your current output, I’ll map exactly what this specific app is doing and give you a tight “set-and-forget” profile for your narration style.
You said:
Im so sorry. i misled you because the gui was confusing. the actual value is between the two extremes on either end. for exmaple, speed is 0.5 on the lowest and 3 on the highest. so right now its actually somehwere in between. do the analysis again with this in mind. i want good clean audio narration, with a slight nod to more expressive storytelling but not enough to introduce to many pauses 
ChatGPT said:

Got it — the key change is: “0.5” and “3” are the range endpoints for Speed, and the knob is somewhere between them. So we should stop treating 0.5 as “the current value.”
1) Speed: what should “normal” be on a 0.5–3 slider?

Almost certainly:

    ~1.0 = normal-ish

    < 1.0 = slower

    > 1.0 = faster

Audiobook “clean but not sleepy” usually lands around 0.95–1.10.

My recommendation (your goal: clean narration, slightly expressive, minimal extra pauses):

    Speed: 1.05 (or 1.0 if you want very neutral)

If your current knob position sounds too slow, move it so the read feels like a typical audiobook narrator. Don’t chase the number—chase the pace.
2) The sampling knobs, reinterpreted with the real ranges

From your screenshot, the ranges look like:

    Temperature: 0.05 → 10

    Repetition penalty: 1 → 10

    Top-k: 10 → 100

    Top-p: 0.1 → 1

    Speed: 0.5 → 3

What those mean for XTTS-style TTS

These knobs control how “adventurous” the model is when generating the internal sequence that becomes speech. In practice:

    Higher temperature / higher top-p / higher top-k
    → more variation (can sound more human), but also more risk: odd emphasis, extra hesitations, weird cadence, occasional artifacts.

    Lower temperature / lower top-p / lower top-k
    → cleaner and more consistent, but can get flat/robotic.

    Repetition penalty
    → helps prevent “loop-y” phrasing/cadence. Too high can cause strange word choice/cadence corrections.

Your stated preference is basically: keep it stable, allow a little expressiveness, avoid exaggerated pausing. That means: moderate sampling, not extreme.
A solid “Clean narration + slight storytelling” preset (try this first)

    Temperature: 0.9–1.4 (on your 0.05–10 scale)
    Important: If your GUI currently has temp around 5, that’s very high for narration and is likely to increase weirdness. Keep it closer to ~1.

    Top-p: 0.55–0.70

    Top-k: 35–60

    Repetition penalty: 1.05–1.20

    Speed: 1.00–1.08 (start 1.05)

This usually yields: stable pronunciation, natural-ish emphasis, without “dramatic pauses.”
3) Two fast corrections depending on what you hear
If you hear too many pauses / weird hesitations / dramatic cadence

    Temperature ↓ to 0.7–1.0

    Top-p ↓ to 0.45–0.60

    Top-k ↓ to 25–45

    Speed ↑ slightly to 1.05–1.12

If it’s too flat / robotic

    Temperature ↑ slightly to 1.2–1.6

    Top-p ↑ to 0.65–0.80

    Keep top-k around 50–80 (don’t crank it to 100 immediately)

4) The quickest way to lock this in (no guessing)

Generate the same paragraph 3 times with only these differences:

    Clean baseline: temp 0.9, top-p 0.55, top-k 40, rep 1.1, speed 1.05

    Slightly richer: temp 1.2, top-p 0.65, top-k 60, rep 1.1, speed 1.05

    Even safer: temp 0.7, top-p 0.50, top-k 35, rep 1.1, speed 1.08

Pick the best, then keep everything fixed for the whole book so chapters don’t “drift” in style.

If you tell me what voice artifacts you’re currently hearing (flatness? extra pauses? mispronunciations? breathy/warbly audio?), I’ll snap this to an even tighter range for Daisy Studio’s XTTS pipeline specifically.
You said:
ok thank you. the gui makes no sense. i hope those were not the default values because theyu're so damnn extreme lmao. im picking a rather extreme moderate config to test how it sounds:
- temp: 1.4
- rep penalty: 1.2
- top-k: 60
- top-p: 0.7
- speed: 1.1

ill let you know how it sounds
ChatGPT said:

That set is totally reasonable for your target (clean + a bit of life) — it’s “moderate” in the way that actually matters for narration.

A couple quick “if you hear X, do Y” knobs so you can iterate fast after your test:

    If it gets a little too dramatic / odd emphasis / occasional hesitations:

        temp → 1.0–1.2 (first change)

        top-p → 0.6–0.65 (second change)

    If it’s clean but slightly too flat:

        temp → 1.6 (small bump)

        or top-p → 0.75 (small bump)
        (change only one at a time)

    If you hear “loopiness” or repetitive cadence:

        rep penalty → 1.25–1.35

    If pauses feel too long but you like everything else:

        speed → 1.15 (small bump)

        and/or top-p down a hair (e.g. 0.65)

When you report back, the most useful description is: (1) pacing, (2) expressiveness, (3) any weird artifacts (hesitations, mispronunciations, warped audio), (4) punctuation handling. That’s enough to dial it in to a “set it and forget it” profile.
You said:
ok its mostly. i used daisy studious. but it will randomly slip into british accent lol. and it was for a book about the history of CSS and it keeps calling CSS more like "she" S S
ChatGPT said:

Yep — those two issues are classic “front-end + XTTS” problems:

    accent drift (it randomly slides into another English accent)

    initialisms getting misread (“CSS” → something like “she S S”)

Here’s how to fix both with the least drama.
1) Random British accent drift

This is usually the model “wandering” because sampling is a bit too free or because it’s not being forced into a specific language/dialect.
Do these in this order

A. Force the language (most important)
In Daisy Studio / ebook2audiobook there’s often a language dropdown or a per-voice setting. Make sure it is explicitly set to:

    en or en-US (prefer en-US if available)

If it’s on “auto” or “detect,” that alone can cause drift, especially in technical text.

B. Reduce randomness just a little
Your current settings are fine for expressiveness, but for accent stability you can tighten them:

    temp 1.4 → 1.1–1.2

    top-p 0.7 → 0.6–0.65

    keep top-k 60

    rep penalty 1.2 is fine

    speed 1.1 is fine

That small change usually stops “accent wandering” while keeping it lively.

C. Make chunking more consistent
Accent drift is more likely when the engine synthesizes long spans and “resets” style differently across segments. If you have chunk controls (sentence/paragraph chunking), prefer:

    moderate chunk sizes (not huge multi-paragraph blobs)

    consistent punctuation cleanup (avoid weird OCR junk)

D. If the voice uses a reference sample
If Daisy Studio is conditioning from a voice sample, use a clean reference that is:

    15–30 seconds

    one speaker

    one consistent accent

    minimal background noise/music
    A mixed-accent or noisy ref can make the model jump.

2) “CSS” being read wrong (“she S S”)

That’s a text normalization / tokenizer thing: “CSS” sometimes gets treated like a word, or the “C” gets softened weirdly, especially in tech context.
Best fix: text replacements (global)

Find ebook2audiobook’s text replacement / preprocessing feature (it often exists for exactly this). Add replacements like:

    CSS → C. S. S.

    HTML → H. T. M. L.

    JS → J. S.

Periods are magic here: they strongly encourage “spell the letters.”

If you don’t want the dots, these also work:

    CSS → C S S

    CSS → the letters C S S

For a CSS-history book, I’d do C. S. S..
Extra: disambiguate the first time

If the book introduces the acronym early, you can force the model to learn the pattern by writing it once as:

    Cascading Style Sheets (C. S. S.)
    After that, your replacement rule handles the rest.

A solid updated config for your exact goals

Try this next:

    temp: 1.2

    rep penalty: 1.2

    top-k: 60

    top-p: 0.62

    speed: 1.1

    language: en-US (or explicitly en if that’s all you have)

    replacements: CSS → C. S. S. (plus any other acronyms)

If you tell me where Daisy Studio/ebook2audiobook stores “replacements” (some UIs have a panel, some use a config file), I can give you the exact format to paste in (regex vs literal, case sensitivity, etc.).
You said:
ok i tried changing it a bit:

- temp: 1.1
- rep penalty: 1.2
- top-k: 60
- top-p: 0.6
- speed: 1.1

With the same voice. but its HUGE pauses where it says nothing for many seconds. Thats horrible. I dont want that. i dont understand how these settings led to that. And then it will sound out slowly a word or two.

When its talking, it seems to be fine. its just the pauses that are devastingly bad
ChatGPT said:

Yeah, that “many-seconds of dead air, then a couple slow words” is not what those sampling knobs are supposed to do in a sane UI. When it happens in XTTS-style pipelines, it’s almost always one of these:

    The text has formatting tokens that the frontend turns into “long break” cues
    (multiple newlines, ellipses, em-dashes, bullet lists, code blocks, weird Unicode spaces, etc.).

    The frontend is adding silence between chunks (sentence/paragraph splitting), and your chunk boundaries are landing constantly (especially in a technical book with headings + code).

    The model is emitting “pause/breath/silence” tokens more often — and tighter sampling (lower top-p / lower temp) can sometimes make that worse because the “safe” token becomes “pause.”

The fact that “when it’s talking, it’s fine” strongly points to #1/#2 more than “bad voice quality.”
Do this first: figure out where the pauses are coming from (fast)

Listen to 2–3 of the worst pauses and check the text around that timestamp. If the pause lines up with any of these, you’ve found the culprit:

    blank lines / multiple newlines

    headings

    lists

    em-dashes (—), ellipses (…), or lots of punctuation

    code blocks (CSS snippets especially)

    inline parentheses with symbols, e.g. { } ; : sequences

In CSS-history content, code blocks are the #1 reason narration engines do something awful (either they pause forever or read symbols weirdly).
Fix path A (most likely): chunking / paragraph-break silence

If ebook2audiobook/Daisy has any of these settings, they’re the ones that actually control multi-second silence:

    “pause between sentences/paragraphs”

    “split by paragraph”

    “max pause”

    “silence after chunk”

    “pause scale”

    “keep line breaks”

What you want:

    Split by sentence (not paragraph) or keep paragraph but reduce pause

    Disable “keep line breaks” if present

    Set pauses to something like:

        sentence pause: 150–250 ms

        paragraph pause: 350–600 ms

        anything above ~1s will feel disastrous

If you can’t find those controls, the UI might be implicitly adding big pauses at paragraph breaks — then the workaround is text cleanup (below).
Fix path B: text normalization that stops “dead-air triggers”

If there’s a “text replacements” feature, add replacements specifically to kill long-break patterns:
Kill long pauses from newlines

    Replace double newline with “. ”

    Replace single newline with “ ” (space)

Normalize punctuation that causes dramatic pauses

    … → ... (or just .)

    — / – → , (comma) or -

For code blocks

If you can, remove CSS code blocks from narration or replace them with something like:

    “Code example omitted.”
    or

    “CSS snippet: selector brace, property, colon, value, semicolon, closing brace.”

(You’ll get a much better audiobook if the narrator doesn’t attempt raw CSS.)
Why your new sampling settings could worsen pauses

Lowering:

    temp (1.4 → 1.1)

    top-p (0.7 → 0.6)

can make the model more likely to pick the “high-probability” prosody tokens. In some TTS models, that includes strong break tokens. So it’s plausible your change increased the chance of long breaks.
A config that usually avoids runaway pauses while staying “clean + slightly expressive”

Try this next after you address chunking/text issues:

    temp: 1.2–1.3

    top-p: 0.70–0.78 ← this is the knob most likely to reduce “stuck on pause”

    top-k: 50–70

    rep penalty: 1.05–1.15 (1.2 is OK, but I’d lower it slightly while debugging pauses)

    speed: 1.05–1.10

So a concrete test:

    temp 1.25

    rep 1.10

    top-k 60

    top-p 0.75

    speed 1.10

If that still produces multi-second silence, it’s almost certainly chunking/text normalization, not sampling.
“Nuclear option” if the tool won’t behave: auto-trim long silences

If ebook2audiobook has an audio filter like “remove silence” / “trim silence”, enable it and set it to remove long gaps but keep natural breaths. A good starting point:

    threshold: around -35 dB

    remove silences longer than 0.4–0.6s

    leave 0.15–0.25s behind

If there’s no GUI toggle and you want a CLI solution later, tell me what file format you’re outputting (wav/flac/mp3) and I’ll give you a fish-friendly one-liner that trims only long dead air (not normal pauses).
One quick question that will let me nail it

Do the huge silences happen right after headings / blank lines / code blocks, or mid-sentence?

    If it’s around formatting: we fix chunking/text preprocessing.

    If it’s mid-sentence: we tighten sampling + check for model stalls.

You said:
matrix-100.txt
Document
even better, here is the text file im generating tts from. 
ChatGPT said:

Those multi-second “dead air” gaps make total sense once you look at your source text: it’s hard-wrapped (newline every ~70–80 chars) and it contains a lot of dash / ellipsis / bracketed citation patterns. Many TTS frontends treat any newline as a “strong break,” and hard-wrapped newlines can turn into huge pauses. You can see the hard wraps clearly (every sentence is split across multiple lines) in your file.

Also: the text includes bracketed footnote markers like [1] and sequences like … / em-dashes, which can further confuse prosody or force long pauses.
Why your sampling changes “caused” pauses (even though they shouldn’t)

They probably didn’t. What happened is more like: the model/front-end is already “on a knife edge” because of the input formatting, and with slightly different sampling it starts emitting stronger break tokens at more of those newline boundaries. But the root cause is the text formatting, not your top-p/top-k settings.
Fix the pauses properly: preprocess the text (unwrap paragraphs)

Goal: collapse single newlines inside paragraphs into spaces, while keeping paragraph breaks (blank lines) as paragraph breaks.

Also strongly recommended for TTS:

    remove footnote markers like [1]

    normalize Unicode dashes to ASCII -

    convert the multiplication sign × to “by” (e.g., 5×5 → 5 by 5)

    optionally convert ellipsis … to ... or .

A clean Python preprocessor (creates matrix-tts.txt)

import re
from pathlib import Path

src = Path("matrix-100.txt")
dst = Path("matrix-tts.txt")

text = src.read_text(encoding="utf-8")

# Normalize common Unicode punctuation that can trigger odd prosody.
text = text.replace("\u2014", " - ")   # em dash —
text = text.replace("\u2013", " - ")   # en dash –
text = text.replace("\u2026", "...")   # ellipsis …
text = text.replace("\u00d7", " by ")  # multiplication sign ×

# Remove bracketed numeric footnotes like [1], [3][4], etc.
text = re.sub(r"\[(\d+)\](\[(\d+)\])+", "", text)  # [3][4] -> ""
text = re.sub(r"\[(\d+)\]", "", text)              # [1] -> ""

# Trim trailing spaces on lines
lines = [ln.rstrip() for ln in text.splitlines()]

# Unwrap paragraphs: join consecutive non-empty lines with spaces.
out_lines = []
buf = []
for ln in lines:
    if ln.strip() == "":
        if buf:
            out_lines.append(" ".join(buf).strip())
            buf = []
        out_lines.append("")  # keep blank line as paragraph break
    else:
        buf.append(ln.strip())

if buf:
    out_lines.append(" ".join(buf).strip())

# Cleanup: collapse repeated spaces
out = re.sub(r"[ \t]{2,}", " ", "\n".join(out_lines)).strip() + "\n"

dst.write_text(out, encoding="utf-8")
print(f"Wrote {dst} ({len(out):,} chars)")

Use matrix-tts.txt as the TTS input. This one change usually eliminates 90%+ of “mystery pauses” in ebook pipelines.
Then set a stable “clean + slight expressiveness” XTTS profile

After preprocessing, your settings should behave much more predictably. I’d run:

    temp: 1.2–1.4

    top-p: 0.65–0.75

    top-k: 50–70

    rep penalty: 1.05–1.15

    speed: 1.05–1.10

If you still get long silences after preprocessing, then the issue is almost certainly an added silence between chunks (sentence/paragraph pause) in Daisy Studio / ebook2audiobook, not XTTS sampling.
Two extra quick wins for your specific text

    The title/heading lines can be treated as “section breaks” by some frontends. Your text has lots of headings and scene labels.
    If the tool has “pause after headings,” turn it down.

    “The Matrix” and proper nouns can nudge accent drift; forcing language to en-US (if available) helps, but preprocessing the punctuation/newlines usually also reduces weird stylistic jumps.

If you want, paste 10–20 lines around one of the worst pauses (as it appears in the TTS input you used), and I’ll tell you exactly which pattern is triggering the silence and what replacement rule fixes it.
You said:
Ok i will make a clean/normlization pipeline to clean text files for better audio generation. But i will write it in rust. Rules:
- use the latest version of each library used
ChatGPT said:

Those “many-seconds of nothing” pauses are very often caused by the text, not your XTTS sampling knobs.

In the snippet you uploaded (matrix-100.txt), I can already see several pause-magnets:

    Hard line wraps inside paragraphs (every ~60–80 chars) → models often treat these like breath / prosody breaks.

    Citation markers like [1] → can trigger weird timing/pause behavior.

    Ellipses ... and long dashes — / – → can become “dramatic pause” tokens.

    Curly punctuation (Gauss’s) and mixed Unicode punctuation → sometimes pushes the model into odd pronunciation modes.

So yes: a Rust “normalization pipeline” is a good move.

Below is a minimal but strong Rust CLI that:

    NFKC-normalizes Unicode

    Removes code fences (...)

    Removes numeric citations like [12]

    Replaces ellipses with a period

    “Unwraps” wrapped lines into one paragraph per line (this is the big one for killing long silences)

    Collapses whitespace

    Expands acronyms like CSS → C S S (fixes the “she S S” issue)

    Adds extensive logging

Dependency versions (latest as of now)

    clap = 4.5.56

    regex = 1.12.2

    anyhow = 1.0.100

    unicode-normalization = 0.1.25

    log = 0.4.29

    env_logger = 0.11.8

    once_cell = 1.21.3

Cargo.toml

[package]
name = "tts-clean"
version = "0.1.0"
edition = "2024"
license = "CC0-1.0"

[dependencies]
anyhow = "1.0.100"
clap = { version = "4.5.56", features = ["derive"] }
env_logger = "0.11.8"
log = "0.4.29"
once_cell = "1.21.3"
regex = "1.12.2"
unicode-normalization = "0.1.25"

src/main.rs

use anyhow::{Context, Result};
use clap::Parser;
use log::{debug, info, warn};
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use unicode_normalization::UnicodeNormalization;

static RE_CODE_FENCE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?s)```.*?```").unwrap());
static RE_INLINE_CODE: Lazy<Regex> = Lazy::new(|| Regex::new(r"`([^`]+)`").unwrap());
static RE_NUM_CITE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[\s*\d+\s*\]").unwrap());
static RE_MULTI_SPACE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[ \t\u{00A0}]+").unwrap());
static RE_SPACE_BEFORE_PUNCT: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s+([,.;:!?])").unwrap());
static RE_MULTI_BLANK: Lazy<Regex> = Lazy::new(|| Regex::new(r"\n{3,}").unwrap());

#[derive(Parser, Debug)]
#[command(
    name = "tts-clean",
    about = "Normalize/clean text for TTS to reduce weird pauses and pronunciations."
)]
struct Args {
    /// Input text file
    #[arg(short, long)]
    input: PathBuf,

    /// Output text file
    #[arg(short, long)]
    output: PathBuf,

    /// Expand common acronyms (e.g., CSS -> C S S)
    #[arg(long, default_value_t = true)]
    expand_acronyms: bool,

    /// If true, remove markdown code fences ```...``` entirely
    #[arg(long, default_value_t = true)]
    drop_code_fences: bool,

    /// If true, strip inline code backticks: `like this` -> like this
    #[arg(long, default_value_t = true)]
    strip_inline_code: bool,

    /// If true, remove numeric citations like [12]
    #[arg(long, default_value_t = true)]
    drop_numeric_citations: bool,

    /// Replace ellipses (...) and (…) with a single period.
    #[arg(long, default_value_t = true)]
    de_ellipsis: bool,

    /// Convert em/en dashes (—/–) to a comma (reduces dramatic pauses)
    #[arg(long, default_value_t = true)]
    dash_to_comma: bool,

    /// Output format: one paragraph per line (recommended for TTS)
    #[arg(long, default_value = "one-paragraph-per-line")]
    format: String,
}

fn main() -> Result<()> {
    // Logging: in fish, you can do: set -x RUST_LOG info
    env_logger::init();

    let args = Args::parse();
    info!("Starting tts-clean");
    info!("Input:  {}", args.input.display());
    info!("Output: {}", args.output.display());
    debug!("Args: {args:#?}");

    let raw = fs::read_to_string(&args.input)
        .with_context(|| format!("Failed to read input: {}", args.input.display()))?;
    info!("Read {} bytes", raw.len());

    let cleaned = clean_text(&raw, &args)?;
    info!("Cleaned text is {} bytes", cleaned.len());

    if cleaned.trim().is_empty() {
        warn!("Output is empty after cleaning. Check input and flags.");
    }

    fs::write(&args.output, cleaned.as_bytes())
        .with_context(|| format!("Failed to write output: {}", args.output.display()))?;
    info!("Done.");

    Ok(())
}

fn clean_text(s: &str, args: &Args) -> Result<String> {
    let mut t = s.to_string();

    // 1) Unicode normalization (NFKC) to reduce “weird unicode” surprises
    t = t.nfkc().collect::<String>();
    debug!("After NFKC: {} chars", t.chars().count());

    // 2) Normalize curly punctuation
    t = t
        .replace('’', "'")
        .replace('‘', "'")
        .replace('“', "\"")
        .replace('”', "\"");

    if args.dash_to_comma {
        // Make dashes less “dramatic pause”
        t = t.replace('—', ", ").replace('–', ", ");
    }

    // 3) Kill code fences (big pause source)
    if args.drop_code_fences {
        let before = t.len();
        t = RE_CODE_FENCE.replace_all(&t, "").to_string();
        debug!("Dropped code fences: {} -> {}", before, t.len());
    }

    // 4) Strip inline code backticks (keeps content)
    if args.strip_inline_code {
        let before = t.len();
        t = RE_INLINE_CODE.replace_all(&t, "$1").to_string();
        debug!("Stripped inline code: {} -> {}", before, t.len());
    }

    // 5) Drop numeric citations like [1]
    if args.drop_numeric_citations {
        let before = t.len();
        t = RE_NUM_CITE.replace_all(&t, "").to_string();
        debug!("Dropped numeric citations: {} -> {}", before, t.len());
    }

    // 6) Replace ellipses with a period
    if args.de_ellipsis {
        t = t.replace("...", ".").replace('…', ".");
    }

    // 7) Standardize newlines
    t = t.replace("\r\n", "\n").replace('\r', "\n");
    t = RE_MULTI_BLANK.replace_all(&t, "\n\n").to_string();

    // 8) Reflow / unwrap wrapped lines into paragraphs
    t = match args.format.as_str() {
        "one-paragraph-per-line" => paragraphs_one_line(&t),
        "preserve-paragraphs" => preserve_paragraphs_unwrap_lines(&t),
        other => {
            warn!("Unknown --format={other}, defaulting to one-paragraph-per-line");
            paragraphs_one_line(&t)
        }
    };

    // 9) Whitespace cleanup
    t = RE_MULTI_SPACE.replace_all(&t, " ").to_string();
    t = RE_SPACE_BEFORE_PUNCT.replace_all(&t, "$1").to_string();
    t = t
        .lines()
        .map(|ln| ln.trim())
        .filter(|ln| !ln.is_empty())
        .collect::<Vec<_>>()
        .join("\n");

    // 10) Acronym expansion (CSS -> C S S)
    if args.expand_acronyms {
        t = expand_acronyms(&t);
    }

    Ok(t.trim().to_string())
}

fn paragraphs_one_line(s: &str) -> String {
    // Split on blank lines. Inside each paragraph, join lines with spaces.
    // Output: exactly one paragraph per line => reduces “silent gaps”.
    let mut out: Vec<String> = Vec::new();
    let mut cur: Vec<String> = Vec::new();

    for line in s.lines() {
        let line = line.trim();
        if line.is_empty() {
            if !cur.is_empty() {
                out.push(cur.join(" "));
                cur.clear();
            }
            continue;
        }
        cur.push(line.to_string());
    }
    if !cur.is_empty() {
        out.push(cur.join(" "));
    }

    out.join("\n")
}

fn preserve_paragraphs_unwrap_lines(s: &str) -> String {
    // Similar to above, but keeps blank line separators.
    let mut out: Vec<String> = Vec::new();
    let mut cur: Vec<String> = Vec::new();

    for line in s.lines() {
        let line = line.trim();
        if line.is_empty() {
            if !cur.is_empty() {
                out.push(cur.join(" "));
                cur.clear();
            }
            out.push(String::new());
            continue;
        }
        cur.push(line.to_string());
    }
    if !cur.is_empty() {
        out.push(cur.join(" "));
    }

    // Collapse repeated blank lines
    let mut cleaned: Vec<String> = Vec::new();
    let mut last_blank = false;
    for ln in out {
        if ln.is_empty() {
            if !last_blank {
                cleaned.push(String::new());
            }
            last_blank = true;
        } else {
            cleaned.push(ln);
            last_blank = false;
        }
    }
    cleaned.join("\n")
}

fn expand_acronyms(s: &str) -> String {
    // Add whatever you want here. These are common TTS troublemakers.
    // The key idea is: force letter-by-letter reading.
    let map: BTreeMap<&'static str, &'static str> = BTreeMap::from([
        ("CSS", "C S S"),
        ("HTML", "H T M L"),
        ("HTTP", "H T T P"),
        ("HTTPS", "H T T P S"),
        ("URL", "U R L"),
        ("API", "A P I"),
        ("CPU", "C P U"),
        ("GPU", "G P U"),
        ("JSON", "J S O N"),
        ("SQL", "S Q L"),
        ("XML", "X M L"),
        ("TTS", "T T S"),
        ("XTTS", "X T T S"),
        ("LLM", "L L M"),
    ]);

    // Word-boundary replacement, case-sensitive on purpose (avoid clobbering normal words).
    // If you want lowercase support later, do a second pass or build a smarter tokenizer.
    let mut out = s.to_string();
    for (k, v) in map {
        let re = Regex::new(&format!(r"\b{}\b", regex::escape(k))).unwrap();
        out = re.replace_all(&out, v).to_string();
    }
    out
}

How this targets your exact pain points

    Huge silent pauses: usually from paragraph/line structure + punctuation. The one-paragraph-per-line output removes internal hard wraps and removes blank-line “dead air”.

    Random accent drifting: often triggered by odd Unicode + punctuation sequences. NFKC + quote normalization + de-ellipsis helps.

    “CSS” → “she S S”: the acronym expansion forces letter names: C S S.

Quick next step (so you can iterate fast)

Run your cleaner on the same input you feed ebook2audiobook, then feed the cleaned output to XTTS.

If you paste 20–40 lines of the CSS history text that’s causing the worst pauses (especially around where it stalls), I can add 2–3 more targeted transforms (e.g., handling headings, bullet lists, inline punctuation like ::, {}, ;, version strings, etc.) without making the narration sound robotic.
You said:
thank you. now create a config.toml to contain policies and parameter tuning for normalization
ChatGPT said:

# tts-clean config.toml
#
# Goal: normalize text so XTTS-style TTS produces clean narration with minimal dead-air pauses.
# Philosophy:
# - kill hard-wrap newlines inside paragraphs (BIGGEST pause fix)
# - normalize punctuation that creates "dramatic" pauses
# - remove/replace constructs that TTS reads badly (citations, code fences, footnotes)
# - optionally expand acronyms / initialisms (CSS -> C S S)

[meta]
version = 1
profile = "clean-narration-slightly-expressive"
notes = "Tune this for XTTS/Daisy Studio / ebook2audiobook."

[io]
# How to format output paragraphs after cleanup:
# - "one-paragraph-per-line": each paragraph becomes a single line (recommended)
# - "preserve-paragraphs": keep blank-line paragraph separators
output_format = "one-paragraph-per-line"

# If true, normalize line endings to LF and trim trailing whitespace.
normalize_line_endings = true
trim_trailing_whitespace = true

[logging]
# env_logger-style levels: trace|debug|info|warn|error
level = "info"
# If true, print a before/after stats summary (counts, removed items, etc.)
print_summary = true
# If true, optionally write a sidecar report (you can implement later)
write_report = false
report_path = "tts-clean.report.txt"

[unicode]
# Apply Unicode normalization to reduce weird glyph variants and compatibility forms.
# Common choices: "NFKC" (recommended), "NFC"
normalization = "NFKC"

# Replace common curly punctuation with ASCII.
ascii_quotes = true

# Replace common dash variants.
# "comma": replace em/en dashes with ", "
# "hyphen": replace with " - "
# "keep": leave as-is
dash_mode = "comma"

# Ellipsis handling:
# "period": replace "…" and "..." with "."
# "triple": replace "…" with "..."
# "keep": leave as-is
ellipsis_mode = "period"

[whitespace]
# Collapse repeated spaces/tabs (including NBSP) to a single normal space.
collapse_horizontal = true
# Remove spaces before punctuation (turn "word ." into "word.")
remove_space_before_punct = true

# Blank line policy:
# Maximum consecutive blank lines to keep (common cause of long silences)
max_consecutive_blank_lines = 1

[structure]
# The big pause fix: unwrap hard-wrapped lines inside paragraphs.
unwrap_hard_wrapped_lines = true

# Treat these as paragraph boundaries (you can tune later if needed).
# "blank_lines" is usually enough.
paragraph_boundary = "blank_lines"

# Optional: convert all single newlines within paragraphs to spaces, then reflow.
# If unwrap_hard_wrapped_lines is true, this is effectively what happens.
join_lines_with = " "

[markdown]
# Drop fenced code blocks entirely. Strongly recommended for technical books.
drop_code_fences = true
# Replace dropped code blocks with a spoken placeholder (empty string = remove entirely)
code_fence_replacement = "Code example omitted."

# Strip inline backticks but keep content.
strip_inline_code = true

# Optional: remove markdown links [text](url) -> text
strip_markdown_links = true

[citations]
# Remove numeric citations like [1] [23]
drop_numeric_brackets = true
# Remove multi-citation sequences like [3][4][5]
drop_stacked_numeric_brackets = true
# Remove parenthetical numeric cites like (1) or (23) if they look like citations
drop_parenthetical_numeric = false

[lists]
# Lists can create weird timing. This flattens list markers into commas.
flatten_bullets = true
bullet_replacement = ", "

# Common bullet markers to detect.
bullet_markers = ["- ", "* ", "• ", "– ", "— "]

[abbreviations]
# Expand initialisms/acronyms to force letter-by-letter reading.
expand_acronyms = true

# How to expand:
# "spaced": CSS -> "C S S"
# "dotted": CSS -> "C. S. S."
acronym_style = "spaced"

# Case policy:
# "exact": only match exact casing (CSS)
# "upper": match if token is all-caps
case_policy = "upper"

# Acronyms map. Add more as you discover TTS pain points.
# Keys are the token to match; values are what to replace with.
[abbreviations.map]
CSS = "C S S"
HTML = "H T M L"
HTTP = "H T T P"
HTTPS = "H T T P S"
URL = "U R L"
API = "A P I"
CPU = "C P U"
GPU = "G P U"
JSON = "J S O N"
SQL = "S Q L"
XML = "X M L"
TTS = "T T S"
XTTS = "X T T S"

[pronunciation]
# Optional lightweight string replacements to improve speech.
# Useful for things like "5×5" or special glyphs.
enable_replacements = true

# NOTE: Keep replacements conservative; overly aggressive rules can make narration weird.
# You can implement these as literal replacements or regex later.
[pronunciation.replacements]
"×" = " by "
"::" = " colon colon "
";" = ","
"{" = " brace "
"}" = " brace "
"(" = ", "
")" = ", "

[guardrails]
# If cleaning results in too-short output, warn (can happen if code fences dominate).
min_output_chars_warn = 200

# If a single paragraph becomes extremely long after unwrapping, you may optionally
# re-split by sentence later. Leave disabled until you implement sentence splitting.
max_paragraph_chars = 0   # 0 = disabled

[experimental]
# If true, try to detect and remove very long runs of punctuation (e.g., "-----", "======")
strip_punct_runs = false
punct_run_min_len = 5

If you want, I can also give you a matching Config struct layout (serde-compatible) for Rust (toml + serde) so your CLI can load this file cleanly and you can add “profiles” later.


ChatGPT can make mistakes. Check important info. See Cookie Preferences.

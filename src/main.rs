use anyhow::{Context, Result};
use clap::Parser;
use env_logger::Builder;
use log::{info, warn};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use std::cmp::Reverse;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use unicode_normalization::UnicodeNormalization;

static RE_CODE_FENCE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?s)```.*?```").unwrap());
static RE_INLINE_CODE: Lazy<Regex> = Lazy::new(|| Regex::new(r"`([^`]+)`").unwrap());
static RE_STACKED_NUM_CITE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?:\[\s*\d+\s*\]){2,}").unwrap());
static RE_NUMERIC_CITE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[\s*\d+\s*\]").unwrap());
static RE_PAREN_CITE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\(\s*\d+(?:,\s*\d+)*\s*\)").unwrap());
static RE_MARKDOWN_LINK: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[([^]]+)\]\([^)]*\)").unwrap());
static RE_MULTI_SPACE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[ \t\u{00A0}]+").unwrap());
static RE_GENERIC_BRACKETS: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\[[^\]]*?\d[^\]]*?\]").unwrap());
static RE_GENERIC_PARENS: Lazy<Regex> = Lazy::new(|| Regex::new(r"\([^)]*?\d[^)]*?\)").unwrap());
static RE_SPACE_BEFORE_PUNCT: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s+([,.;:!?])").unwrap());
static RE_PUNCT_RUN: Lazy<Regex> = Lazy::new(|| Regex::new(r"([=\\-~]{5,})").unwrap());
static RE_HTML_OPEN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"<\s*([a-zA-Z][a-zA-Z0-9]*)[^>]*>").unwrap());
static RE_HTML_CLOSE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"</\s*[a-zA-Z][a-zA-Z0-9]*\s*>").unwrap());
static RE_COMMA_BEFORE_PERIOD: Lazy<Regex> = Lazy::new(|| Regex::new(r",\s*\.").unwrap());
static RE_ACRONYM_SEQUENCE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\b[A-Z0-9](?:[.\s]*[A-Z0-9])+\b").unwrap());

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Clean and normalize text before feeding it into XTTS-style TTS engines."
)]
struct Args {
    /// File that should be cleaned.
    #[arg(short, long, value_name = "FILE")]
    input: PathBuf,

    /// Where the normalized text should be written.
    #[arg(short, long, value_name = "FILE")]
    output: PathBuf,

    /// Optional override for the config toml. Defaults to ./config.toml.
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct Config {
    meta: MetaConfig,
    io: IoConfig,
    unicode: UnicodeConfig,
    whitespace: WhitespaceConfig,
    structure: StructureConfig,
    markdown: MarkdownConfig,
    citations: CitationConfig,
    lists: ListConfig,
    abbreviations: AbbreviationConfig,
    pronunciation: PronunciationConfig,
    guardrails: GuardrailConfig,
    logging: LoggingConfig,
    experimental: ExperimentalConfig,
    punctuation: PunctuationConfig,
    selector: SelectorConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            meta: MetaConfig::default(),
            io: IoConfig::default(),
            unicode: UnicodeConfig::default(),
            whitespace: WhitespaceConfig::default(),
            structure: StructureConfig::default(),
            markdown: MarkdownConfig::default(),
            citations: CitationConfig::default(),
            lists: ListConfig::default(),
            abbreviations: AbbreviationConfig::default(),
            pronunciation: PronunciationConfig::default(),
            guardrails: GuardrailConfig::default(),
            logging: LoggingConfig::default(),
            experimental: ExperimentalConfig::default(),
            punctuation: PunctuationConfig::default(),
            selector: SelectorConfig::default(),
        }
    }
}

impl Config {
    /// Load the config from disk (or fall back to defaults).
    fn load(path: Option<&Path>) -> Result<Self> {
        let path = path.unwrap_or_else(|| Path::new("config.toml"));
        match fs::read_to_string(path) {
            Ok(contents) => {
                let cfg: Config = toml::from_str(&contents)
                    .with_context(|| format!("failed to parse {}", path.display()))?;
                Ok(cfg)
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                warn!(
                    "config {} not found, falling back to defaults",
                    path.display()
                );
                Ok(Config::default())
            }
            Err(err) => Err(err).context("reading config")?,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct MetaConfig {
    version: u32,
    profile: String,
    notes: String,
}

impl Default for MetaConfig {
    fn default() -> Self {
        Self {
            version: 1,
            profile: "clean-narration-slightly-expressive".to_string(),
            notes: "Targeted for XTTS / Daisy Studio / ebook2audiobook pipelines.".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct IoConfig {
    output_format: OutputFormat,
    normalize_line_endings: bool,
    trim_trailing_whitespace: bool,
}

impl Default for IoConfig {
    fn default() -> Self {
        Self {
            output_format: OutputFormat::OneParagraphPerLine,
            normalize_line_endings: true,
            trim_trailing_whitespace: true,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum OutputFormat {
    OneParagraphPerLine,
    PreserveParagraphs,
}

impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::OneParagraphPerLine
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct UnicodeConfig {
    normalization: UnicodeNormalizationMode,
    ascii_quotes: bool,
    dash_mode: DashMode,
    ellipsis_mode: EllipsisMode,
}

impl Default for UnicodeConfig {
    fn default() -> Self {
        Self {
            normalization: UnicodeNormalizationMode::Nfkc,
            ascii_quotes: true,
            dash_mode: DashMode::Comma,
            ellipsis_mode: EllipsisMode::Period,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum UnicodeNormalizationMode {
    Nfkc,
    Nfc,
    None,
}

impl Default for UnicodeNormalizationMode {
    fn default() -> Self {
        UnicodeNormalizationMode::Nfkc
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum DashMode {
    Comma,
    Hyphen,
    Keep,
}

impl Default for DashMode {
    fn default() -> Self {
        DashMode::Comma
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum EllipsisMode {
    Period,
    Triple,
    Keep,
}

impl Default for EllipsisMode {
    fn default() -> Self {
        EllipsisMode::Period
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct WhitespaceConfig {
    collapse_horizontal: bool,
    remove_space_before_punct: bool,
    max_consecutive_blank_lines: usize,
}

impl Default for WhitespaceConfig {
    fn default() -> Self {
        Self {
            collapse_horizontal: true,
            remove_space_before_punct: true,
            max_consecutive_blank_lines: 1,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct StructureConfig {
    unwrap_hard_wrapped_lines: bool,
    paragraph_boundary: ParagraphBoundary,
    join_lines_with: String,
}

impl Default for StructureConfig {
    fn default() -> Self {
        Self {
            unwrap_hard_wrapped_lines: true,
            paragraph_boundary: ParagraphBoundary::BlankLines,
            join_lines_with: " ".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum ParagraphBoundary {
    BlankLines,
    Never,
}

impl Default for ParagraphBoundary {
    fn default() -> Self {
        ParagraphBoundary::BlankLines
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct MarkdownConfig {
    drop_code_fences: bool,
    code_fence_replacement: String,
    strip_inline_code: bool,
    strip_markdown_links: bool,
}

impl Default for MarkdownConfig {
    fn default() -> Self {
        Self {
            drop_code_fences: true,
            code_fence_replacement: String::new(),
            strip_inline_code: true,
            strip_markdown_links: true,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct CitationConfig {
    drop_numeric_brackets: bool,
    drop_stacked_numeric_brackets: bool,
    drop_parenthetical_numeric: bool,
    drop_generic_parentheses: bool,
    drop_generic_brackets: bool,
}

impl Default for CitationConfig {
    fn default() -> Self {
        Self {
            drop_numeric_brackets: true,
            drop_stacked_numeric_brackets: true,
            drop_parenthetical_numeric: false,
            drop_generic_parentheses: true,
            drop_generic_brackets: true,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct ListConfig {
    flatten_bullets: bool,
    bullet_replacement: String,
    #[serde(default = "ListConfig::default_markers")]
    bullet_markers: Vec<String>,
}

impl ListConfig {
    fn default_markers() -> Vec<String> {
        ["- ", "* ", "• ", "– ", "— "].map(str::to_string).to_vec()
    }
}

impl Default for ListConfig {
    fn default() -> Self {
        Self {
            flatten_bullets: true,
            bullet_replacement: ", ".to_string(),
            bullet_markers: Self::default_markers(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct AbbreviationConfig {
    expand_acronyms: bool,
    acronym_style: AcronymStyle,
    case_policy: CasePolicy,
    #[serde(default)]
    map: BTreeMap<String, String>,
    letter_separator: String,
    digit_separator: String,
}

impl Default for AbbreviationConfig {
    fn default() -> Self {
        let mut map = BTreeMap::new();
        map.insert("CSS".to_string(), "C S S".to_string());
        map.insert("HTML".to_string(), "H T M L".to_string());
        map.insert("HTTP".to_string(), "H T T P".to_string());
        map.insert("HTTPS".to_string(), "H T T P S".to_string());
        map.insert("URL".to_string(), "U R L".to_string());
        map.insert("API".to_string(), "A P I".to_string());
        map.insert("CPU".to_string(), "C P U".to_string());
        map.insert("GPU".to_string(), "G P U".to_string());
        map.insert("JSON".to_string(), "J S O N".to_string());
        map.insert("SQL".to_string(), "S Q L".to_string());
        map.insert("XML".to_string(), "X M L".to_string());
        map.insert("TTS".to_string(), "T T S".to_string());
        map.insert("XTTS".to_string(), "X T T S".to_string());
        map.insert("LLM".to_string(), "L L M".to_string());
        Self {
            expand_acronyms: true,
            acronym_style: AcronymStyle::Spaced,
            case_policy: CasePolicy::Upper,
            map,
            letter_separator: "".to_string(),
            digit_separator: " dot ".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum AcronymStyle {
    Spaced,
    Dotted,
}

impl Default for AcronymStyle {
    fn default() -> Self {
        AcronymStyle::Spaced
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum CasePolicy {
    Exact,
    Upper,
}

impl Default for CasePolicy {
    fn default() -> Self {
        CasePolicy::Upper
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct PronunciationConfig {
    enable_replacements: bool,
    #[serde(default)]
    replacements: BTreeMap<String, String>,
    #[serde(default)]
    brand_map: BTreeMap<String, String>,
    year_mode: YearMode,
    html_tag_pronunciation: bool,
    html_tag_separator: String,
    version_mode: VersionMode,
    number_config: NumberConfig,
}

impl Default for PronunciationConfig {
    fn default() -> Self {
        let mut replacements = BTreeMap::new();
        replacements.insert("×".to_string(), " by ".to_string());
        replacements.insert("::".to_string(), " colon colon ".to_string());
        replacements.insert(";".to_string(), ",".to_string());
        replacements.insert("{".to_string(), " brace ".to_string());
        replacements.insert("}".to_string(), " brace ".to_string());
        replacements.insert("(".to_string(), ", ".to_string());
        replacements.insert(")".to_string(), ", ".to_string());

        let mut brand_map = BTreeMap::new();
        brand_map.insert("MySQL".to_string(), "My S. Q. L.".to_string());
        brand_map.insert("Mysql".to_string(), "My S. Q. L.".to_string());
        brand_map.insert("SQLITE".to_string(), "S. Q. Lite".to_string());
        brand_map.insert("SQLite".to_string(), "S. Q. Lite".to_string());
        brand_map.insert("PostCSS".to_string(), "Post C. S. S.".to_string());
        brand_map.insert("W3C".to_string(), "Double U Three C".to_string());
        brand_map.insert("JSSS".to_string(), "J. S. S. S.".to_string());
        brand_map.insert("IE4".to_string(), "I. E. Four".to_string());

        Self {
            enable_replacements: true,
            replacements,
            brand_map,
            year_mode: YearMode::American,
            html_tag_pronunciation: true,
            html_tag_separator: " ".to_string(),
            version_mode: VersionMode::SayDecimal,
            number_config: NumberConfig::default(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct NumberConfig {
    separator: String,
    insert_and: bool,
}

impl Default for NumberConfig {
    fn default() -> Self {
        Self {
            separator: " ".to_string(),
            insert_and: true,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
enum VersionMode {
    None,
    SayDecimal,
}

impl Default for VersionMode {
    fn default() -> Self {
        VersionMode::SayDecimal
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum YearMode {
    None,
    American,
}

impl Default for YearMode {
    fn default() -> Self {
        YearMode::American
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct GuardrailConfig {
    min_output_chars_warn: usize,
    max_paragraph_chars: usize,
}

impl Default for GuardrailConfig {
    fn default() -> Self {
        Self {
            min_output_chars_warn: 200,
            max_paragraph_chars: 0,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct LoggingConfig {
    level: String,
    print_summary: bool,
    write_report: bool,
    report_path: String,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            print_summary: true,
            write_report: false,
            report_path: "tts-clean.report.txt".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct ExperimentalConfig {
    strip_punct_runs: bool,
    punct_run_min_len: usize,
}

impl Default for ExperimentalConfig {
    fn default() -> Self {
        Self {
            strip_punct_runs: false,
            punct_run_min_len: 5,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct PunctuationConfig {
    collapse_commas: bool,
    max_consecutive_commas: usize,
    slash_replacement: String,
    stop_precedence: String,
}

impl Default for PunctuationConfig {
    fn default() -> Self {
        Self {
            collapse_commas: true,
            max_consecutive_commas: 1,
            slash_replacement: " or ".to_string(),
            stop_precedence: ".:;,?".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct SelectorConfig {
    prefix: String,
}

impl Default for SelectorConfig {
    fn default() -> Self {
        Self {
            prefix: "dot ".to_string(),
        }
    }
}

#[derive(Default)]
struct CleanStats {
    input_length: usize,
    output_length: usize,
    paragraph_count: usize,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let config = Config::load(args.config.as_deref())?;
    init_logger(&config.logging);
    info!("Loaded config profile: {}", config.meta.profile);

    let raw = fs::read_to_string(&args.input)
        .with_context(|| format!("Failed to read {}", args.input.display()))?;
    info!("Read {} bytes from {}", raw.len(), args.input.display());

    let (cleaned, stats) = clean_text(&raw, &config);
    info!(
        "Cleaned text is {} bytes ({} paragraphs)",
        stats.output_length, stats.paragraph_count
    );

    if config.guardrails.min_output_chars_warn > 0
        && stats.output_length < config.guardrails.min_output_chars_warn
    {
        warn!(
            "output is shorter than {} chars ({}), double-check that cleaning did not drop the whole document",
            config.guardrails.min_output_chars_warn, stats.output_length
        );
    }

    if config.logging.print_summary {
        info!(
            "Summary: read {} bytes, wrote {} bytes, {} paragraphs",
            stats.input_length, stats.output_length, stats.paragraph_count
        );
    }

    if config.logging.write_report {
        let report = format!(
            "Clean report\n============\nInput: {}\nOutput: {}\nParagraphs: {}\nProfile: {}\n",
            args.input.display(),
            args.output.display(),
            stats.paragraph_count,
            config.meta.profile
        );
        fs::write(&config.logging.report_path, report)
            .with_context(|| format!("writing report to {}", config.logging.report_path))?;
        info!("Wrote report to {}", config.logging.report_path);
    }

    fs::write(&args.output, cleaned)
        .with_context(|| format!("Failed to write {}", args.output.display()))?;
    info!("Wrote cleaned text to {}", args.output.display());

    Ok(())
}

fn init_logger(logging: &LoggingConfig) {
    let env = env_logger::Env::default().default_filter_or(logging.level.as_str());
    Builder::from_env(env).init();
}

fn clean_text(s: &str, config: &Config) -> (String, CleanStats) {
    let mut text = s.to_string();
    let mut stats = CleanStats::default();
    stats.input_length = text.len();

    if config.io.normalize_line_endings {
        text = text.replace("\r\n", "\n").replace('\r', "\n");
    }

    if config.io.trim_trailing_whitespace {
        text = text
            .lines()
            .map(|line| line.trim_end())
            .collect::<Vec<_>>()
            .join("\n");
    }

    text = match config.unicode.normalization {
        UnicodeNormalizationMode::Nfkc => text.nfkc().collect::<String>(),
        UnicodeNormalizationMode::Nfc => text.nfc().collect::<String>(),
        UnicodeNormalizationMode::None => text,
    };

    if config.unicode.ascii_quotes {
        text = text
            .replace("’", "'")
            .replace("‘", "'")
            .replace("“", "\"")
            .replace("”", "\"");
    }

    text = match config.unicode.dash_mode {
        DashMode::Comma => text.replace('—', ", ").replace('–', ", "),
        DashMode::Hyphen => text.replace('—', " - ").replace('–', " - "),
        DashMode::Keep => text,
    };

    text = match config.unicode.ellipsis_mode {
        EllipsisMode::Period => text.replace('…', ".").replace("...", "."),
        EllipsisMode::Triple => text.replace('…', "..."),
        EllipsisMode::Keep => text,
    };

    if config.markdown.drop_code_fences {
        if RE_CODE_FENCE.is_match(&text) {
            text = RE_CODE_FENCE
                .replace_all(&text, config.markdown.code_fence_replacement.as_str())
                .to_string();
        }
    }

    if config.markdown.strip_inline_code {
        text = RE_INLINE_CODE.replace_all(&text, "$1").to_string();
    }

    if config.markdown.strip_markdown_links {
        text = RE_MARKDOWN_LINK.replace_all(&text, "$1").to_string();
    }

    if config.citations.drop_stacked_numeric_brackets {
        text = RE_STACKED_NUM_CITE.replace_all(&text, "").to_string();
    }
    if config.citations.drop_numeric_brackets {
        text = RE_NUMERIC_CITE.replace_all(&text, "").to_string();
    }
    if config.citations.drop_parenthetical_numeric {
        text = RE_PAREN_CITE.replace_all(&text, "").to_string();
    }
    if config.citations.drop_generic_brackets {
        text = RE_GENERIC_BRACKETS.replace_all(&text, "").to_string();
    }
    if config.citations.drop_generic_parentheses {
        text = RE_GENERIC_PARENS.replace_all(&text, "").to_string();
    }

    if config.structure.unwrap_hard_wrapped_lines {
        text = unwrap_paragraphs(
            &text,
            &config.structure.join_lines_with,
            &config.structure.paragraph_boundary,
        );
    }

    if config.lists.flatten_bullets {
        text = flatten_bullets(&text, &config.lists);
    }

    if config.whitespace.collapse_horizontal {
        text = RE_MULTI_SPACE.replace_all(&text, " ").to_string();
    }

    if config.whitespace.remove_space_before_punct {
        text = RE_SPACE_BEFORE_PUNCT.replace_all(&text, "$1").to_string();
    }

    if config.whitespace.max_consecutive_blank_lines > 0 {
        text = collapse_blank_lines(&text, config.whitespace.max_consecutive_blank_lines);
    }

    if config.pronunciation.enable_replacements && !config.pronunciation.replacements.is_empty() {
        text = apply_replacements(&text, &config.pronunciation.replacements);
    }
    if !config.pronunciation.brand_map.is_empty() {
        text = apply_brand_pronunciation(&text, &config.pronunciation.brand_map);
    }

    match config.pronunciation.year_mode {
        YearMode::American => {
            text = apply_year_pronunciation(
                &text,
                &config.pronunciation.year_mode,
                &config.pronunciation.number_config,
            );
        }
        YearMode::None => {}
    }

    if config.abbreviations.expand_acronyms && !config.abbreviations.map.is_empty() {
        text = expand_acronyms(&text, &config.abbreviations);
    }

    if config.pronunciation.version_mode != VersionMode::None {
        text = apply_version_pronunciation(&text, &config.pronunciation.version_mode);
    }

    if config.pronunciation.html_tag_pronunciation {
        text = apply_html_pronunciation(&text, &config.pronunciation.html_tag_separator);
    }
    if !config.selector.prefix.is_empty() {
        text = apply_selector_pronunciation(&text, &config.selector.prefix);
    }

    if config.punctuation.collapse_commas && config.punctuation.max_consecutive_commas > 0 {
        text = collapse_commas(&text, config.punctuation.max_consecutive_commas);
    }

    if !config.punctuation.slash_replacement.is_empty() {
        text = replace_slashes(&text, &config.punctuation.slash_replacement);
    }

    if !config.punctuation.stop_precedence.is_empty() {
        text = collapse_stop_sequences(&text, &config.punctuation.stop_precedence);
    }

    text = RE_COMMA_BEFORE_PERIOD.replace_all(&text, ".").to_string();

    text = RE_MULTI_SPACE.replace_all(&text, " ").to_string();

    if config.experimental.strip_punct_runs && config.experimental.punct_run_min_len > 0 {
        let pattern = RE_PUNCT_RUN.clone();
        text = pattern
            .replace_all(&text, |caps: &regex::Captures| {
                caps[1].chars().next().unwrap_or('-').to_string()
            })
            .to_string();
    }

    if config.io.trim_trailing_whitespace {
        text = text
            .lines()
            .map(|line| line.trim_end())
            .collect::<Vec<_>>()
            .join("\n");
    }

    text = text.trim().to_string();
    text.push('\n');

    stats.output_length = text.len();
    stats.paragraph_count = text.lines().filter(|line| !line.trim().is_empty()).count();

    if config.guardrails.max_paragraph_chars > 0 {
        let limit = config.guardrails.max_paragraph_chars;
        for line in text.lines() {
            if !line.trim().is_empty() && line.len() > limit {
                warn!(
                    "paragraph exceeds guardrail of {} chars ({} chars)",
                    limit,
                    line.len()
                );
            }
        }
    }

    (text, stats)
}

fn unwrap_paragraphs(text: &str, joiner: &str, boundary: &ParagraphBoundary) -> String {
    let mut paragraphs = Vec::new();
    let mut buffer = Vec::new();

    for line in text.lines() {
        if line.trim().is_empty() {
            if !buffer.is_empty() {
                paragraphs.push(buffer.join(joiner));
                buffer.clear();
            }
            if let ParagraphBoundary::BlankLines = boundary {
                paragraphs.push(String::new());
            }
        } else {
            buffer.push(line.trim().to_string());
        }
    }

    if !buffer.is_empty() {
        paragraphs.push(buffer.join(joiner));
    }

    paragraphs.join("\n")
}

fn flatten_bullets(text: &str, cfg: &ListConfig) -> String {
    text.lines()
        .map(|line| {
            let trimmed = line.trim_start();
            if let Some(marker) = cfg
                .bullet_markers
                .iter()
                .find(|marker| trimmed.starts_with(marker.as_str()))
            {
                let remainder = trimmed[marker.len()..].trim_start();
                format!("{}{}", cfg.bullet_replacement, remainder)
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn collapse_blank_lines(text: &str, max_blank: usize) -> String {
    if max_blank == 0 {
        return text.to_string();
    }
    let mut count = 0;
    let mut out = Vec::new();
    for line in text.lines() {
        if line.trim().is_empty() {
            count += 1;
            if count <= max_blank {
                out.push(String::new());
            }
        } else {
            count = 0;
            out.push(line.to_string());
        }
    }
    out.join("\n")
}

fn collapse_commas(text: &str, max_consecutive: usize) -> String {
    if max_consecutive == 0 {
        return text.to_string();
    }

    let mut result = String::with_capacity(text.len());
    let mut comma_run = 0;
    let mut buffered_space = String::new();

    for ch in text.chars() {
        if ch == ',' {
            if comma_run < max_consecutive {
                if !buffered_space.is_empty() {
                    result.push_str(&buffered_space);
                    buffered_space.clear();
                }
                result.push(',');
            }
            comma_run += 1;
        } else if ch.is_whitespace() {
            buffered_space.push(ch);
        } else {
            if !buffered_space.is_empty() {
                result.push_str(&buffered_space);
                buffered_space.clear();
            }
            comma_run = 0;
            result.push(ch);
        }
    }

    if !buffered_space.is_empty() {
        result.push_str(&buffered_space);
    }

    result
}

fn replace_slashes(text: &str, replacement: &str) -> String {
    if replacement.is_empty() {
        text.replace('/', "")
    } else {
        text.replace('/', replacement)
    }
}

fn collapse_stop_sequences(text: &str, precedence: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut run: Vec<char> = Vec::new();
    for ch in text.chars() {
        if precedence.contains(ch) {
            run.push(ch);
            continue;
        }
        if !run.is_empty() {
            if let Some(chosen) = choose_stop(&run, precedence) {
                result.push(chosen);
            }
            run.clear();
        }
        result.push(ch);
    }
    if !run.is_empty() {
        if let Some(chosen) = choose_stop(&run, precedence) {
            result.push(chosen);
        }
    }
    result
}

fn choose_stop(run: &[char], precedence: &str) -> Option<char> {
    for pref in precedence.chars() {
        if run.contains(&pref) {
            return Some(pref);
        }
    }
    run.first().copied()
}

fn apply_replacements(text: &str, replacements: &BTreeMap<String, String>) -> String {
    let mut result = text.to_string();
    let mut entries: Vec<_> = replacements.iter().collect();
    entries.sort_by_key(|(key, _)| Reverse(key.len()));

    for (from, to) in entries {
        result = result.replace(from, to);
    }

    result
}

fn apply_brand_pronunciation(text: &str, brands: &BTreeMap<String, String>) -> String {
    let mut result = text.to_string();
    let mut entries: Vec<_> = brands.iter().collect();
    entries.sort_by_key(|(key, _)| Reverse(key.len()));

    for (from, to) in entries {
        let pattern = Regex::new(&format!(r"\b{}\b", regex::escape(from))).unwrap();
        result = pattern.replace_all(&result, to.as_str()).to_string();
    }

    result
}

fn apply_year_pronunciation(text: &str, mode: &YearMode, number_config: &NumberConfig) -> String {
    let mut result = text.to_string();
    match mode {
        YearMode::American => {
            let re = Regex::new(r"\b(1\d{3}|20\d{2})\b").unwrap();
            result = re
                .replace_all(&result, |caps: &regex::Captures| {
                    let year: usize = caps[1].parse().unwrap_or(0);
                    year_to_words(year, number_config)
                })
                .to_string();
        }
        YearMode::None => {}
    }
    result
}

fn year_to_words(year: usize, number_config: &NumberConfig) -> String {
    if year < 1000 || year > 2099 {
        return year.to_string();
    }
    let ones = [
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let teens = [
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];
    let tens = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    let thousands = year / 1000;
    let hundreds = (year / 100) % 10;
    let remainder = year % 100;

    let mut parts: Vec<String> = Vec::new();
    if thousands > 0 {
        parts.push(format!("{} thousand", ones[thousands]));
    }
    if hundreds > 0 {
        parts.push(format!("{} hundred", ones[hundreds]));
    }

    if remainder > 0 {
        let mut remainder_str = String::new();
        if remainder < 10 {
            remainder_str.push_str(ones[remainder]);
        } else if remainder < 20 {
            remainder_str.push_str(teens[remainder - 10]);
        } else {
            remainder_str.push_str(tens[remainder / 10]);
            if remainder % 10 > 0 {
                remainder_str.push_str(" ");
                remainder_str.push_str(ones[remainder % 10]);
            }
        }

        if hundreds > 0 && number_config.insert_and {
            parts.push(format!("and {}", remainder_str));
        } else {
            parts.push(remainder_str);
        }
    }

    parts.join(&number_config.separator)
}

fn simple_number_to_words(n: usize) -> String {
    if n == 0 {
        return "zero".to_string();
    }
    if n >= 10_000 {
        return n.to_string();
    }

    let ones = [
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let teens = [
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];
    let tens = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    let thousands = n / 1000;
    let hundreds = (n / 100) % 10;
    let remainder = n % 100;

    let mut parts: Vec<String> = Vec::new();
    if thousands > 0 {
        parts.push(format!("{}", ones[thousands]));
        parts.push("thousand".to_string());
    }
    if hundreds > 0 {
        parts.push(format!("{}", ones[hundreds]));
        parts.push("hundred".to_string());
    }

    if remainder > 0 {
        if remainder < 10 {
            parts.push(ones[remainder].to_string());
        } else if remainder < 20 {
            parts.push(teens[remainder - 10].to_string());
        } else {
            let ten = remainder / 10;
            parts.push(tens[ten].to_string());
            if remainder % 10 > 0 {
                parts.push(ones[remainder % 10].to_string());
            }
        }
    }

    parts.join(" ")
}

fn apply_version_pronunciation(text: &str, mode: &VersionMode) -> String {
    if let VersionMode::SayDecimal = mode {
        let re = Regex::new(r"\b\d+(?:\.\d+)+\b").unwrap();
        re.replace_all(text, |caps: &regex::Captures| {
            caps[0]
                .split('.')
                .map(|segment| {
                    segment
                        .parse::<usize>()
                        .map(simple_number_to_words)
                        .unwrap_or_else(|_| segment.to_string())
                })
                .collect::<Vec<_>>()
                .join(" point ")
        })
        .to_string()
    } else {
        text.to_string()
    }
}

fn apply_html_pronunciation(text: &str, separator: &str) -> String {
    let result = RE_HTML_OPEN
        .replace_all(text, |caps: &regex::Captures| {
            if separator.is_empty() {
                caps[1].to_string()
            } else {
                format!("{}{}", &caps[1], separator)
            }
        })
        .to_string();
    RE_HTML_CLOSE.replace_all(&result, "").to_string()
}

fn apply_selector_pronunciation(text: &str, prefix: &str) -> String {
    if prefix.is_empty() {
        return text.to_string();
    }
    let re = Regex::new(r"(?P<dot>\.)(?P<name>[a-zA-Z0-9_-]+)").unwrap();
    re.replace_all(text, |caps: &regex::Captures| {
        format!("{}{}", prefix, &caps["name"])
    })
    .to_string()
}

fn expand_acronyms(text: &str, cfg: &AbbreviationConfig) -> String {
    let mut result = text.to_string();
    for (token, replacement) in cfg.map.iter() {
        let token_to_match = match cfg.case_policy {
            CasePolicy::Exact => token.clone(),
            CasePolicy::Upper => token.to_uppercase(),
        };
        let final_replacement = match cfg.acronym_style {
            AcronymStyle::Spaced => replacement.clone(),
            AcronymStyle::Dotted => replacement.replace(' ', ". "),
        };
        let pattern = format!(
            r"\b{}(?P<digits>\d+(?:\.\d+)*)?\b",
            regex::escape(&token_to_match)
        );
        let re = Regex::new(&pattern).unwrap();
        result = re
            .replace_all(&result, |caps: &regex::Captures| {
                let mut spelled = final_replacement.clone();
                if let Some(digits) = caps.name("digits") {
                    if !digits.as_str().is_empty() {
                        spelled.push(' ');
                        let formatted = digits
                            .as_str()
                            .split('.')
                            .collect::<Vec<_>>()
                            .join(&cfg.digit_separator);
                        spelled.push_str(&formatted);
                    }
                }
                spelled
            })
            .to_string();
    }
    result = normalize_acronym_spacing(&result, &cfg.letter_separator);
    result
}

fn normalize_acronym_spacing(text: &str, separator: &str) -> String {
    if separator != "" {
        return RE_ACRONYM_SEQUENCE
            .replace_all(text, |caps: &regex::Captures| {
                let letters: Vec<String> = caps[0]
                    .chars()
                    .filter(|c| c.is_alphanumeric())
                    .map(|c| c.to_string())
                    .collect();
                letters.join(separator)
            })
            .to_string();
    }
    RE_ACRONYM_SEQUENCE
        .replace_all(text, |caps: &regex::Captures| {
            caps[0]
                .chars()
                .filter(|c| c.is_alphanumeric())
                .collect::<String>()
        })
        .to_string()
}

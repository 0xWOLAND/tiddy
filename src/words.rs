use rand::seq::SliceRandom;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Debug, Clone)]
struct WordList {
    words: Vec<String>,
}

impl Default for WordList {
    fn default() -> Self {
        Self {
            words: ["the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"]
                .iter().map(|&s| s.to_string()).collect(),
        }
    }
}

const AVAILABLE_LANGUAGES: &[&str] = &[
    "afrikaans", "afrikaans_10k", "afrikaans_1k", "albanian", "albanian_1k", "amharic", "amharic_1k", "amharic_5k", 
    "arabic", "arabic_10k", "arabic_egypt", "arabic_egypt_1k", "armenian", "armenian_1k", "armenian_western", 
    "armenian_western_1k", "azerbaijani", "azerbaijani_1k", "bangla", "bangla_10k", "bangla_letters", "bashkir", 
    "belarusian", "belarusian_100k", "belarusian_10k", "belarusian_1k", "belarusian_25k", "belarusian_50k", 
    "belarusian_5k", "belarusian_lacinka", "belarusian_lacinka_1k", "bosnian", "bosnian_4k", "bulgarian", 
    "bulgarian_latin", "catalan", "catalan_1k", "chinese_simplified", "chinese_simplified_10k", "chinese_simplified_1k", 
    "chinese_simplified_5k", "chinese_simplified_50k", "chinese_traditional", "code_arduino", "code_assembly", 
    "code_bash", "code_brainfck", "code_c++", "code_c", "code_cobol", "code_common_lisp", "code_csharp", "code_css", 
    "code_dart", "code_elixir", "code_fortran", "code_fsharp", "code_gdscript", "code_gdscript_2", "code_go", 
    "code_haskell", "code_html", "code_java", "code_javascript", "code_javascript_1k", "code_javascript_react", 
    "code_jule", "code_julia", "code_kotlin", "code_latex", "code_lua", "code_luau", "code_matlab", "code_nim", 
    "code_nix", "code_odin", "code_ook", "code_opencl", "code_pascal", "code_perl", "code_php", "code_powershell", 
    "code_python", "code_python_1k", "code_python_2k", "code_python_5k", "code_r", "code_r_2k", "code_rockstar", 
    "code_ruby", "code_rust", "code_scala", "code_sql", "code_swift", "code_systemverilog", "code_typescript", 
    "code_typst", "code_v", "code_vim", "code_vimscript", "code_visual_basic", "code_zig", "croatian", "croatian_1k", 
    "czech", "czech_10k", "czech_1k", "danish", "danish_10k", "danish_1k", "docker_file", "dutch", "dutch_10k", 
    "dutch_1k", "english", "english_10k", "english_1k", "english_25k", "english_450k", "english_5k", 
    "english_commonly_misspelled", "english_contractions", "english_doubleletter", "english_medical", "english_old", 
    "english_shakespearean", "esperanto", "esperanto_10k", "esperanto_1k", "esperanto_25k", "esperanto_36k", 
    "esperanto_h_sistemo", "esperanto_h_sistemo_10k", "esperanto_h_sistemo_1k", "esperanto_h_sistemo_25k", 
    "esperanto_h_sistemo_36k", "esperanto_x_sistemo", "esperanto_x_sistemo_10k", "esperanto_x_sistemo_1k", 
    "esperanto_x_sistemo_25k", "esperanto_x_sistemo_36k", "estonian", "estonian_10k", "estonian_1k", "estonian_5k", 
    "euskera", "filipino", "filipino_1k", "finnish", "finnish_10k", "finnish_1k", "french", "french_10k", "french_1k", 
    "french_2k", "french_600k", "french_bitoduc", "frisian", "frisian_1k", "friulian", "galician", "georgian", 
    "german", "german_10k", "german_1k", "german_250k", "git", "greek", "greek_10k", "greek_1k", "greek_25k", 
    "greek_5k", "greeklish", "greeklish_10k", "greeklish_1k", "greeklish_25k", "greeklish_5k", "gujarati", 
    "gujarati_1k", "hausa", "hausa_1k", "hebrew", "hebrew_10k", "hebrew_1k", "hebrew_5k", "hindi", "hindi_1k", 
    "hinglish", "hungarian", "hungarian_2k", "icelandic_1k", "indonesian", "indonesian_10k", "indonesian_1k", 
    "irish", "italian", "italian_1k", "italian_280k", "italian_60k", "italian_7k", "japanese_hiragana", 
    "japanese_katakana", "japanese_romaji", "japanese_romaji_1k", "jyutping", "kabyle", "kabyle_10k", "kabyle_1k", 
    "kabyle_2k", "kabyle_5k", "kannada", "kazakh", "kazakh_1k", "khmer", "klingon", "klingon_1k", "korean", 
    "korean_1k", "korean_5k", "kurdish_central", "kurdish_central_2k", "kurdish_central_4k", "kyrgyz", "kyrgyz_1k", 
    "latin", "latvian", "latvian_1k", "league_of_legends", "lithuanian", "lithuanian_1k", "lithuanian_3k", 
    "lojban_cmavo", "lojban_gismu", "lorem_ipsum", "macedonian", "macedonian_10k", "macedonian_1k", "macedonian_75k", 
    "malagasy", "malagasy_1k", "malay", "malay_1k", "malayalam", "maltese", "maltese_1k", "maori_1k", "marathi", 
    "mongolian", "mongolian_10k", "myanmar_burmese", "nepali", "nepali_1k", "nepali_romanized", "norwegian_bokmal", 
    "norwegian_bokmal_10k", "norwegian_bokmal_150k", "norwegian_bokmal_1k", "norwegian_bokmal_5k", "norwegian_bokmal_600k", 
    "norwegian_nynorsk", "norwegian_nynorsk_100k", "norwegian_nynorsk_10k", "norwegian_nynorsk_1k", "norwegian_nynorsk_400k", 
    "norwegian_nynorsk_5k", "occitan", "occitan_10k", "occitan_1k", "occitan_2k", "occitan_5k", "oromo", "oromo_1k", 
    "oromo_5k", "pashto", "persian", "persian_1k", "persian_20k", "persian_5k", "persian_romanized", "pig_latin", 
    "pinyin", "pinyin_10k", "pinyin_1k", "polish", "polish_10k", "polish_200k", "polish_20k", "polish_2k", 
    "polish_40k", "polish_5k", "portuguese", "portuguese_1k", "portuguese_320k", "portuguese_3k", "portuguese_550k", 
    "portuguese_5k", "portuguese_acentos_e_cedilha", "quenya", "romanian", "romanian_100k", "romanian_10k", 
    "romanian_1k", "romanian_200k", "romanian_25k", "romanian_50k", "romanian_5k", "russian", "russian_10k", 
    "russian_1k", "russian_25k", "russian_375k", "russian_50k", "russian_5k", "russian_abbreviations", 
    "russian_contractions", "russian_contractions_1k", "sanskrit", "sanskrit_roman", "santali", "serbian", 
    "serbian_10k", "serbian_latin", "serbian_latin_10k", "shona", "shona_1k", "sinhala", "slovak", "slovak_10k", 
    "slovak_1k", "slovenian", "slovenian_1k", "slovenian_5k", "spanish", "spanish_10k", "spanish_1k", 
    "spanish_650k", "swahili_1k", "swedish", "swedish_1k", "swedish_diacritics", "swiss_german", "swiss_german_1k", 
    "swiss_german_2k", "tamil", "tamil_1k", "tamil_old", "tanglish", "tatar", "tatar_1k", "tatar_5k", "tatar_9k", 
    "tatar_crimean", "tatar_crimean_10k", "tatar_crimean_15k", "tatar_crimean_1k", "tatar_crimean_5k", 
    "tatar_crimean_cyrillic", "tatar_crimean_cyrillic_10k", "tatar_crimean_cyrillic_15k", "tatar_crimean_cyrillic_1k", 
    "tatar_crimean_cyrillic_5k", "telugu", "telugu_1k", "thai", "thai_10k", "thai_1k", "thai_20k", "thai_50k", 
    "thai_5k", "thai_60k", "tibetan", "tibetan_1k", "toki_pona", "toki_pona_ku_lili", "toki_pona_ku_suli", 
    "turkish", "turkish_1k", "turkish_5k", "twitch_emotes", "typing_of_the_dead", "udmurt", "ukrainian", 
    "ukrainian_10k", "ukrainian_1k", "ukrainian_50k", "ukrainian_endings", "ukrainian_latynka", "ukrainian_latynka_10k", 
    "ukrainian_latynka_1k", "ukrainian_latynka_50k", "ukrainian_latynka_endings", "urdish", "urdu", "urdu_1k", 
    "urdu_5k", "uzbek", "uzbek_1k", "uzbek_70k", "vietnamese", "vietnamese_1k", "vietnamese_5k", "viossa", 
    "viossa_njutro", "welsh", "welsh_1k", "wordle", "wordle_1k", "xhosa", "xhosa_3k", "yiddish", "yoruba_1k", "zulu"
];

pub fn languages() -> Vec<String> {
    AVAILABLE_LANGUAGES.iter().map(|&lang| format!("{lang}.json")).collect()
}

fn cache_dir() -> PathBuf {
    dirs::cache_dir().unwrap_or_else(|| PathBuf::from(".")).join("tiddy")
}

pub fn downloaded() -> Vec<String> {
    let dir = cache_dir();
    if !dir.exists() { return vec![]; }
    
    let mut files = fs::read_dir(&dir).ok()
        .map(|entries| entries.flatten()
            .filter_map(|entry| entry.file_name().to_str().map(|s| s.to_string()))
            .filter(|name| name.ends_with(".json"))
            .collect::<Vec<_>>())
        .unwrap_or_default();
    files.sort();
    files
}

pub async fn download(filename: &str) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!("https://raw.githubusercontent.com/monkeytypegame/monkeytype/refs/heads/master/frontend/static/languages/{filename}");
    
    let dir = cache_dir();
    fs::create_dir_all(&dir)?;
    let path = dir.join(filename);
    
    // Return cached file if exists
    if path.exists() {
        let content = fs::read_to_string(&path)?;
        let word_list: WordList = serde_json::from_str(&content)?;
        return Ok(word_list.words);
    }
    
    // Download, cache, and return
    let content = reqwest::get(&url).await?.text().await?;
    let word_list: WordList = serde_json::from_str(&content)?;
    fs::write(&path, &content)?;
    Ok(word_list.words)
}

fn load_words(filename: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let paths = [format!("words/{filename}"), cache_dir().join(filename).to_string_lossy().to_string()];
    
    for path in &paths {
        if let Ok(content) = fs::read_to_string(path) {
            let word_list: WordList = serde_json::from_str(&content)?;
            return Ok(word_list.words);
        }
    }
    
    Err("File not found".into())
}

pub fn generate_words(count: usize, word_list: Option<&str>) -> Vec<String> {
    let filename = word_list.unwrap_or("english.json");
    let words = load_words(filename).unwrap_or_else(|_| WordList::default().words);
    words.choose_multiple(&mut rand::thread_rng(), count).cloned().collect()
}

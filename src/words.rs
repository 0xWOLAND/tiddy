use rand::seq::SliceRandom;

const COMMON_WORDS: &[&str] = &[
    "the", "be", "to", "of", "and", "a", "in", "that", "have", "it", "for", "not", "on", "with",
    "he", "as", "you", "do", "at", "this", "but", "his", "by", "from", "they", "we", "say", "her",
    "she", "or", "an", "will", "my", "one", "all", "would", "there", "what", "so", "up", "out",
    "if", "about", "who", "get", "which", "go", "me", "when", "make", "can", "like", "time", "no",
    "just", "know", "take", "people", "into", "year", "good", "some", "could", "them", "see",
    "other", "than", "then", "now", "look", "only", "come", "over", "think", "also", "back",
    "after", "use", "two", "how", "work", "first", "well", "way", "even", "new", "want", "because",
    "any", "these", "give", "day", "most", "us", "was", "are", "been", "has", "had", "were", "said"
];

pub fn generate_words(count: usize) -> Vec<String> {
    let mut rng = rand::thread_rng();
    COMMON_WORDS
        .choose_multiple(&mut rng, count)
        .map(|&word| word.to_owned())
        .collect()
}

pub fn words_to_text(words: Vec<String>) -> String {
    words.join(" ")
}
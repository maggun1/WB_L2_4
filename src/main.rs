use std::collections::HashMap;

fn find_anagrams<'a>(words: &'a Vec<&'a str>) -> HashMap<&'a str, Vec<&'a str>> {
    let mut anagram_map: HashMap<String, Vec<&'a str>> = HashMap::new();

    for &word in words {
        let word_lower = word.to_lowercase();
        let mut sorted_word: Vec<char> = word_lower.chars().collect();
        sorted_word.sort_unstable();
        let sorted_word_key: String = sorted_word.into_iter().collect();

        anagram_map.entry(sorted_word_key)
            .or_insert(Vec::new())
            .push(word);
    }

    let mut result: HashMap<&'a str, Vec<&'a str>> = HashMap::new();
    for (_, group) in anagram_map {
        if group.len() > 1 {
            let mut sorted_group = group.clone();
            sorted_group.sort();
            result.insert(sorted_group[0], sorted_group);
        }
    }

    result
}

fn main() {
    let words = vec!["пятак", "пятка", "тяпка", "листок", "слиток", "столик", "компьютер", "телефон"];
    let anagram_map = find_anagrams(&words);

    for (key, group) in &anagram_map {
        println!("{}: {:?}", key, group);
    }
}

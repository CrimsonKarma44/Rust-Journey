pub mod my_solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut result = String::new();
        let length = word1.len().max(word2.len());
        for (index, _) in (0..length).enumerate() {
            if index < word1.len() {
                result.push_str(&word1[index..index + 1]);
            }
            if index < word2.len() {
                result.push_str(&word2[index..index + 1]);
            }
        }
        result
    }
}

pub mod others_solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut result = String::with_capacity(word1.len() + word2.len());
        for (c1, c2) in word1.chars().zip(word2.chars()) {
            result.push(c1);
            result.push(c2);
        }
        result.extend(word1.chars().skip(word2.len()));
        result.extend(word2.chars().skip(word1.len()));
        result
    }
}

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let words = s.split_whitespace();
        match words.last() {
            Some(word) => return word.chars().count() as i32,
            None => return 0,
        };
    }
}

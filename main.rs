use std::io;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
  let secret_word = "apple apple pan";
  let word_size = get_word_size(secret_word);
  println!("{}", hidden_secret_word(secret_word));
}

fn hidden_secret_word(secret_word: &str) -> String {
  let mut hidden = String::new();
  let word_characters: Vec<char> = secret_word.chars().collect();

  for letter in word_characters {
    if letter.is_whitespace() {
      hidden.push_str("   ");
    }
    hidden.push_str("_");
    hidden.push_str(" ");
  }
  hidden
}

fn get_word_size(secret_word: &str) -> usize {
  UnicodeSegmentation::graphemes(secret_word, true).count()
}

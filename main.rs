use std::io;
use unicode_segmentation::UnicodeSegmentation;

const CHANCES: i32 = 6;

fn hidden_secret_word(secret_word: &str) -> String {
  let mut hidden = String::new();
  let word_characters: Vec<char> = secret_word.chars().collect();
  let word_size = UnicodeSegmentation::graphemes(secret_word, true).count();

  for letter in 0..word_size {
    hidden.push_str("_");
    hidden.push_str(" ");
  }

  hidden
}

fn main() {
  let secret_word = "apple";
  println!("{}", hidden_secret_word(secret_word));
}

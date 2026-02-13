use std::io;
use std::fmt;
use std::process::Command;
use unicode_segmentation::UnicodeSegmentation;

struct WrongLetters {
  letters: Vec<char>,
}
impl WrongLetters {
  fn new(wrong_letters: &str) -> Self {
    WrongLetters { letters: wrong_letters.chars().collect(), }
  }
}
impl fmt::Display for WrongLetters {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.letters.iter().collect::<String>())
  }
}

fn main() {
  let secret_word = "apple";
  let word_size = get_word_size(secret_word);
  let mut attempts = 0;
  let chances: usize = word_size + 2;
  let mut wrong_letters_list = WrongLetters::new("");
  let mut hidden = hidden_secret_word(secret_word);
  let mut final_word = String::new();

  while attempts < chances {
    println!("The secret word is: {}\n", hidden);
    if !wrong_letters_list.letters.is_empty() {
      println!("wrong letters: {}", wrong_letters_list);
      println!("-------------------------------\n");
    }

    let mut letter = String::new();
    println!("=> Type a letter: ");
    io::stdin().read_line(&mut letter).expect("an error ocurred to read this letter.");
    let letter: char = letter.trim().chars().next().unwrap();

    if !secret_word.contains(letter) {
      wrong_letters_list.letters.push(letter);
      println!("wrong letter!");
      console_clear();
    } else {
      let mut chars: Vec<char> = hidden.chars().collect();
      for (idx, chr) in secret_word.char_indices() {
        if letter == chr && idx < chars.len() {
          chars[idx] = letter;
          final_word.push(letter);
        }
      }
      hidden = chars.into_iter().collect();
    }
    console_clear();
    attempts += 1;

    if is_winner(&final_word, secret_word) {
      println!("You won!");
      break; 
    } else {
      println!("Game over!");
    };
  }
}

fn is_winner(final_word: &str, original_word: &str) -> bool {
  if final_word == original_word {
    return true;
  }
  false
}

fn hidden_secret_word(secret_word: &str) -> String {
  let mut hidden = String::new();
  let word_characters: Vec<char> = secret_word.chars().collect();

  for letter in word_characters {
    if letter.is_whitespace() {
      hidden.push_str("   ");
    }
    hidden.push_str("_ ");
  }
  hidden
}

fn get_word_size(secret_word: &str) -> usize {
  UnicodeSegmentation::graphemes(secret_word, true).count()
}

fn console_clear() {
  if cfg!(target_os = "windows") {
    Command::new("cmd")
      .args(["/C", "cls"])
      .status()
      .expect("an error ocurred to clear the terminal!");
  } else {
    Command::new("sh")
      .arg("-c")
      .arg("clear")
      .status()
      .expect("an error ocurred to clear the terminal!");
  }
}

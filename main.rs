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
  let secret_word = "apple pan";
  let word_size = get_word_size(secret_word);
  let mut attempts = 0;
  let chances: usize = word_size + 2;
  let mut wrong_letters_list = WrongLetters::new("");
  let mut hidden = hidden_secret_word(secret_word);

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
      for (idx, chr) in secret_word.chars().enumerate() {
        if letter == chr && idx < chars.len() {
          chars[idx] = letter;
        }
      }
      hidden = chars.into_iter().collect();
    }
    console_clear();
    attempts += 1;

    if is_winner(&hidden, secret_word) {
      println!("You won!");
      break;
    }
  }

  if attempts == chances {
    println!("Game over!");
  }
}

fn is_winner(final_word: &str, original_word: &str) -> bool {
  final_word == original_word
}

fn hidden_secret_word(secret_word: &str) -> String {
  secret_word.chars()
  .map(|c|
    if c.is_whitespace() { ' ' } else { '_' }
  )
  .collect()
}

fn get_word_size(secret_word: &str) -> usize {
  let no_space: String = secret_word.chars().filter(|c| !c.is_whitespace()).collect();
  UnicodeSegmentation::graphemes(no_space.as_str(), true).count()
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

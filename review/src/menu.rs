pub mod auth;
pub mod main;
pub mod review;

use dialoguer::{
  Select,
  theme::ColorfulTheme
};
use console::Term;
use std::io::Result;

pub fn clear_line() {
  let term = Term::stdout();
  term.move_cursor_up(1).expect("Cursor moving error");
  term.clear_line().expect("Failed to clear line");
}

pub fn make_choice<'a>(options: Vec<&'a str>, title: &str) -> Result<&'a str> {
  println!("{}", title);
  
  let option_index = Select::with_theme(&ColorfulTheme::default())
    .items(&options)
    .default(0)
    .interact()?;
  clear_line();

  Ok(options[option_index])
}
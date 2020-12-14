extern crate dialoguer;
extern crate console;

use dialoguer::{
  Select,
  theme::ColorfulTheme,
  Input,
  Password
};
use console::Term;

fn main() {
  let option = make_choice(vec!["Login", "Register"], "New here?");
  let choice_result = option.unwrap();
  println!("Let's {}", choice_result);
  auth(&choice_result);
}

fn clear_line() {
  let term = Term::stdout();
  term.move_cursor_up(1).expect("Cursor moving error");
  term.clear_line().expect("Failed to clear line");
}

fn make_choice(options: Vec<&str>, title: &str) -> std::io::Result<String> {
  println!("{}", title);
  let option_index = Select::with_theme(&ColorfulTheme::default())
    .items(&options)
    .default(0)
    .interact()?;
  clear_line();
  let option = options[option_index];
  Ok(String::from(option))
}

fn ask_login() -> std::io::Result<String> {
  let input: String = Input::new()
    .with_prompt("Entrer Login")
    .interact_text()?;
  Ok(input)
}

fn ask_password(s: &str) -> std::io::Result<String> {
  match s {
    "Register" => {
      Ok(
        Password::new()
          .with_prompt("New Password")
          .with_confirmation("Confirm password", "Passwords mismatching")
          .interact()?
      )
    },
    "Login" => {
      Ok(
        Password::new()
          .with_prompt("Enter Password")
          .interact()?
      )
    },
    _ => unreachable!()
  }

}
 
fn validation(s: &str) -> bool {
  match s.len() {
    0..=6 => {
      println!("Too short");
      false
    },
    7..=16 => {
      println!("It's OK");
      true
    },
    _ => {
      println!("Too long");
      false
    }
  }
}

fn login_query() -> String {
  let answer = ask_login().unwrap();
  match validation(&answer) {
    true => answer,
    false => login_query()
  }
}

fn password_query(s: &str) -> String {
  let answer = ask_password(s).unwrap();
  match validation(&answer) {
    true => answer,
    false => password_query(s)
  }
}

fn auth(s: &str) {
  let login = login_query();
  println!("{}", login);
  //go to db and check existance of login
  let password = password_query(s);
  //go to db and check password if login
  // let pass_result = match handle {
  // }
}
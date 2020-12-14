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
  let auth = make_choice(vec!["Login", "Register"], "New here?");
  let auth_result = match auth.unwrap().as_str() {
    "Login" => {
      println!("Let's Login");
      let password = password_query(false);
    },
    "Register" => {
      println!("Let's Register");
      let password = password_query(true);
    },
    _ => unreachable!()
  };
  let login = login_query();
  println!("{}", login);
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

fn ask_password(confirm: bool) -> std::io::Result<String> {
  match confirm {
    true => {
      Ok(
        Password::new()
          .with_prompt("New Password")
          .with_confirmation("Confirm password", "Passwords mismatching")
          .interact()?
      )
    },
    false => {
      Ok(
        Password::new()
          .with_prompt("Enter Password")
          .interact()?
      )
    }
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

fn password_query(confirm: bool) -> String {
  let answer = ask_password(confirm).unwrap();
  match validation(&answer) {
    true => answer,
    false => password_query(confirm)
  }
}
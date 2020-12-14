use dialoguer::{
  Select,
  theme::ColorfulTheme,
  Input,
  Password
};
use console::Term;

pub fn clear_line() {
  let term = Term::stdout();
  term.move_cursor_up(1).expect("Cursor moving error");
  term.clear_line().expect("Failed to clear line");
}

pub fn make_choice<'a>(options: Vec<&'a str>, title: &str) -> std::io::Result<&'a str> {
  println!("{}", title);
  
  let option_index = Select::with_theme(&ColorfulTheme::default())
    .items(&options)
    .default(0)
    .interact()?;
  clear_line();

  Ok(options[option_index])
}

pub fn ask_login() -> std::io::Result<String> {
  let input: String = Input::new()
    .with_prompt("Entrer Login")
    .interact_text()?;
  Ok(input)
}

pub fn ask_password(s: &str) -> std::io::Result<String> {
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
 
pub fn validation(s: &str) -> bool {
  match s.len() {
    0..=6 => {
      println!("Too short");
      false
    },
    7..=16 => true,
    _ => {
      println!("Too long");
      false
    }
  }
}

pub fn login_query() -> String {
  let answer = ask_login().unwrap();
  clear_line();

  match validation(&answer) {
    true => answer,
    false => { 
      let s = login_query();
      clear_line();
      s
    }
  }
}

pub fn password_query(s: &str) -> String {
  let answer = ask_password(s).unwrap();
  clear_line();

  match validation(&answer) {
    true => answer,
    false => password_query(s)
  }
}

pub fn auth(s: &str) -> String {
  let login = login_query();
  //go to db and check existance of login

  let password = password_query(s);
  //go to db and check password if login
  // let pass_result = match handle {
  // }

  String::from("")
}
use std::io::Result as IOResult;
use postgres::Error;
use dialoguer::{
  Input,
  Password
};
use super::*;
use crate::db::*;

pub fn ask_login() -> IOResult<String> {
  let input: String = Input::new()
    .with_prompt("Entrer Login")
    .interact_text()?;
  Ok(input)
}

pub fn ask_password(s: &str) -> IOResult<String> {
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
    0..=4 => {
      println!("Too short");
      false
    },
    5..=16 => true,
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

fn check_login(login: &str, db: Db) -> bool {
  let query = db.execute("SELECT name FROM Users WHERE name = $1", &[&login]);
  match query {
    Ok(_) => true,
    Err(_) => false
  }
}

fn check_password(login: &str, input_password: &str, db: Db) -> bool {
  let query = db
    .execute(
      "SELECT password FROM Users WHERE name = $1", 
      &[&login]
    ).unwrap();
  let db_password: &str = query[0].get(0);
  input_password == db_password
}

fn create_user(login: &str, password: &str, db: Db) -> std::result::Result<(), Error> {
  db.execute(
    "INSERT INTO Users VALUES ($1, $2)", 
    &[&login, &password]
  )?;
  Ok(())
}

pub fn auth(option: &str, db: Db) -> String {
  let login = login_query();
  let is_login_exist = check_login(&login, db);

  match option {
    "Regsiter" => {
      if is_login_exist {
        println!("User with name <{}> already exists!", login);
        return auth(option, db);
      }
      let password = password_query(option);
      match create_user(&login, &password, db) {
        Ok(_) => String::from("registration_successful"),
        Err(_) => return auth(option, db)
      }
    },
    "Login" => {
      if !is_login_exist {
        println!("User with name <{}> doesn't exist!", login);
        return auth(option, db);
      }
      let password = password_query(option);
      let password_ok = check_password(&login, &password, db);
      if password_ok {
        String::from("login_successful")
      } else {
        auth(option, db)
      }
    },
    _ => unreachable!()
  }
}
use std::io::Result as IOResult;
use dialoguer::{
  Input,
  Password
};
use super::*;
use crate::db::*;

fn ask_login() -> IOResult<String> {
  let input: String = Input::new()
    .with_prompt("Enter Login")
    .interact()?;
  Ok(input)
}

fn ask_password(s: &str) -> IOResult<String> {
  match s {
    "Register" => {
      Ok(
        Password::new()
          .with_prompt("Enter password for your user")
          .with_confirmation("Confirm password", "Passwords mismatch")
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

fn check_login(login: &str, option: &str, db: &mut Db) -> bool {
  match login.len() {
    0..=4 => {
      println!("Too short");
      return false
    },
    5..=16 => (),
    _ => {
      println!("Too long");
      return false
    }
  }

  let query = db.execute("SELECT name FROM Users WHERE name = $1", &[&login]);
  match query.len() {
    0 => {
      match option {
        "Login" => {
          println!("User with name <{}> doesn't exist!", login);
          false
        },
        "Register" => true,
        _ => unreachable!()
      }
    }
    _ => {
      match option {
        "Login" => true,
        "Register" => {
          println!("User with name <{}> already exists!", login);
          false
        },
        _ => unreachable!()
      }
    }
  }
}

fn check_password(login: &str, input_password: &str, db: &mut Db) -> bool {
  match input_password.len() {
    0..=4 => {
      println!("Too short");
      return false
    },
    5..=16 => (),
    _ => {
      println!("Too long");
      return false
    }
  }

  let query = db
    .execute(
      "SELECT password FROM Users WHERE name = $1", 
      &[&login]
    );

  let db_password: &str = query[0].get(0);
  
  input_password == db_password
}

fn create_user(login: &str, password: &str, db: &mut Db) {
  db.execute(
    "INSERT INTO Users(name, password) VALUES ($1, $2)", 
    &[&login, &password]
  );
}

pub fn auth<'a>(db: &mut Db, user: &mut Option<SystemUser>) -> Option<MenuInput> {
  let option = make_choice(vec![
      "Login", 
      "Register"
    ], 
    "New here?"
  ).unwrap();

  let mut login = ask_login().unwrap();
  while !check_login(&login, option, db) {
    login = ask_login().unwrap();
  }

  *user = Some(SystemUser { login: login.clone() });

  let mut password = ask_password(option).unwrap();

  match option {
    "Login" => {
      while !check_password(&login, &password, db) {  
        password = ask_password(option).unwrap(); 
      }
    },
    "Register" => create_user(&login, &password, db),
    _ => unreachable!()
  };

  Some(Success)
}
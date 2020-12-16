use std::io::Result as IOResult;
use postgres::Error;
use dialoguer::{
  Input,
  Password
};
use super::*;
use crate::db::*;

fn ask_login() -> IOResult<String> {
  let input: String = Input::new()
    .with_prompt("Enter Login")
    .interact_text()?;
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
 
fn validate(s: &str) -> bool {
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

// fn login_query() -> String {
//   let mut answer = ask_login().unwrap();

//   while !validate(&answer) {
//     answer = ask_login().unwrap();
//   }

//   answer
// }

// fn password_query(s: &str) -> String {
//   let mut answer = ask_password(s).unwrap();

//   while !validate(&answer) {
//     answer = ask_password(s).unwrap();
//   }

//   answer
// }

fn check_login(login: &str, option: &str, db: &mut Db) -> bool {
  if !validate(login) {
    return false;
  }

  let query = db.execute("SELECT name FROM Users WHERE name = $1", &[&login]).expect("SQL issue");
  // match option {
  //   "Login" => {
  //     println!("User with name <{}> doesn't exist!", login);
  //     false
  //   },
  //   "Register" => true,
  //   _ => unreachable!()
  // }
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
  if !validate(input_password) {
    return false;
  }

  let query = db
    .execute(
      "SELECT password FROM Users WHERE name = $1", 
      &[&login]
    ).expect("SQL issue");

  let db_password: &str = query[0].get(0);
  
  input_password == db_password
}

fn create_user(login: &str, password: &str, db: &mut Db) -> std::result::Result<(), Error> {
  db.execute(
    "INSERT INTO Users VALUES ($1, $2)", 
    &[&login, &password]
  )?;
  Ok(())
}

pub fn auth(db: &mut Db) -> Option<MenuInput> {
  let option = make_choice(vec![
      "Login", 
      "Register"
    ], 
    "New here?"
  ).unwrap();

  let mut login = ask_login().unwrap();
  while !check_login(&login, option, db) {
    clear_screen();

    login = ask_login().unwrap();
  }

  let mut password = ask_password(option).unwrap();

  match option {
    "Login" => {
      while !check_password(&login, &password, db) {  
        clear_screen();
        password = ask_password(option).unwrap(); 
      }
    },
    "Register" => {
      match create_user(&login, &password, db) {
        Ok(_) => (),
        Err(_) =>  {
          clear_screen();
          println!("Failed to register user, try again");

          return Some(Failed);
        }
      }
    },
    _ => unreachable!()
  }

  Some(Success)
}
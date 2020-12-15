extern crate dialoguer;
extern crate console;

mod menu;
mod automaton;
mod db;

use crate::menu::*;
use crate::automaton::*;
use crate::db::*;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum MenuState {
  Auth,
  Main,
  Review,
  Teachers,
  Subjects
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum MenuInput {
  Logout,
  Subjects,
  Teachers,
  Review,
  Success,
  Back,
  None
}
use MenuState::*;
use MenuInput::*;

struct SystemUser {
  login: String
}

fn main() {
  let transition_table = Box::new(|state: MenuState, x: MenuInput| -> MenuState {
    match (state, x) {
      (Auth, Success)              => Main,
      (Main, Logout)               => Auth,
      (Main, MenuInput::Subjects)  => MenuState::Subjects,
      (Main, MenuInput::Teachers)  => MenuState::Teachers,
      (Main, MenuInput::Review)    => MenuState::Review,
      (MenuState::Teachers, Back)  => Main,
      (MenuState::Subjects, Back)  => Main,
      (MenuState::Review, Success) => Main,
      (MenuState::Review, Back)    => Main,
      _ => Auth
    }
  });
  
  let output_table = Box::new(|state: MenuState, x: MenuInput| -> Option<MenuInput> {
    match (state, x) {
      (Auth, _) => {
        use auth::*;

        // let mut review_db = Db::new("postgresql://postgres:postgres@127.0.0.1/review").unwrap();
        let option = make_choice(vec![
            "Login", 
            "Register"
          ], 
          "New here?"
        ).unwrap();
        // auth(&option, &mut review_db);
    
        Some(Success)
      },
      (Main, _) => {
        let option = make_choice(vec![
          "Show Subjects", 
          "Show Teachers", 
          "Make Review", 
          "Logout", 
          "Exit"
          ], "")
          .unwrap();
        
        match option {
          "Exit" => Option::None,
          "Show Subjects" => Some(MenuInput::Subjects), 
          "Show Teachers" => Some(MenuInput::Teachers), 
          "Make Review" => Some(MenuInput::Review), 
          "Logout" => Some(Logout), 
          _ => unreachable!()
        }
      },
      (MenuState::Teachers, _) => {
        //display info

        make_choice(vec!["Back"], "").unwrap();

        Some(Back)
      },
      (MenuState::Subjects, _) => {
        // display info

        make_choice(vec!["Back"], "").unwrap();

        Some(Back)
      },
      (MenuState::Review, _) => {
        use review::*;

        review_menu().unwrap();

        Some(Back)
      },
      _ => Option::None
    }
  });
  
  let mut menu = Automaton::new(output_table, transition_table, Auth);
  let mut input = Some(None);

  while let Some(output) = menu.transition(input) {
    input = output;
  }
}

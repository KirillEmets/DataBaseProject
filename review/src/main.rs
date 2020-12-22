extern crate dialoguer;
extern crate console;

mod menu;
mod automaton;
mod db;

use crate::menu::*;
use crate::automaton::*;
use crate::db::*;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum MenuState {
  Auth,
  Main,
  Review,
  Show
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum MenuInput {
  Logout,
  Subjects,
  Teachers,
  Review,
  Success,
  Failed,
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
      (Auth, Failed)               => Auth,
      (Auth, Success)              => Main,
      (Main, Logout)               => Auth,
      (Main, Subjects)             => Show,
      (Main, Teachers)             => Show,
      (Main, MenuInput::Review)    => MenuState::Review,
      (Show, Back)                 => Main,
      (MenuState::Review, Success) => Main,
      (MenuState::Review, Back)    => Main,
      _ => Auth
    }
  });
  
  let output_table = Box::new(|state: MenuState, x: MenuInput| -> Option<MenuInput> {
    let mut review_db = Db::new("postgresql://postgres:postgres@127.0.0.1/review").unwrap();
    clear_screen();

    match (state, x) {
      (Auth, _) => auth::auth(&mut review_db),
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
          "Show Subjects" => Some(Subjects), 
          "Show Teachers" => Some(Teachers), 
          "Make Review" => Some(MenuInput::Review), 
          "Logout" => Some(Logout), 
          _ => unreachable!()
        }
      },
      (Show, Teachers) => {
        //display info

        make_choice(vec!["Back"], "").unwrap();

        Some(Back)
      },
      (Show, Subjects) => {
        // display info

        make_choice(vec!["Back"], "").unwrap();

        Some(Back)
      },
      (MenuState::Review, _) => review::review(&mut review_db),
      _ => Option::None
    }
  });
  
  let mut menu = Automaton::new(output_table, transition_table, Auth);
  let mut input = Some(None);

  while let Some(output) = menu.transition(input) {
    input = output;
  }
}

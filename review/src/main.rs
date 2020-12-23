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

#[derive(Clone)]
pub struct SystemUser {
  login: String
}

fn main() {
  let transition_table = Box::new(|state: &MenuState, x: &MenuInput, user: &mut Option<SystemUser>| -> MenuState {
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
  
  let output_table = Box::new(|state: &MenuState, x: &MenuInput, user: &mut Option<SystemUser>| -> Option<MenuInput> {
    clear_screen();
    let mut review_db = Db::new("postgresql://postgres:12hr56tf@127.0.0.1/review");

    match (state, x) {
      (Auth, _) => auth::auth(&mut review_db, user),
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
      (Show, x) => {
        //display info
        match x {
          Teachers => {
            let teachers = review::get_teachers(&mut review_db);
            println!("Teachers\n");
            for teacher in teachers {
              println!("{}", teacher.name);
            } 
          },
          Subjects => {
            let subjects = review::get_subjects(&mut review_db);
            println!("Subjects\n");
            for subject in subjects {
              println!("{}", subject.name);
            } 
          },
          _ => unreachable!()
        }

        make_choice(vec!["Back"], "").unwrap();

        Some(Back)
      },
      (MenuState::Review, _) => review::review(&mut review_db, user.clone().unwrap().login.as_str()),
      _ => Option::None
    }
  });
  
  let mut menu = Automaton::new(output_table, transition_table, Auth, Option::None);
  let mut input = Some(None);

  while let Some(output) = menu.transition(input) {
    input = output;
  }
}

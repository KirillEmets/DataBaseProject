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
  Review,
  Reviews,
  Subjects,
  Teachers,
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

pub struct Menu {
  storage:          Option<SystemUser>,
  state:            MenuState,
  output_table:     Box<dyn FnMut(&MenuState, &MenuInput, &mut Option<SystemUser>) -> Option<MenuInput>>,
  transition_table: Box<dyn FnMut(&MenuState, &MenuInput, &mut Option<SystemUser>) -> MenuState>,
}

impl Menu{
  pub fn new(
    output_table:     Box<dyn FnMut(&MenuState, &MenuInput, &mut Option<SystemUser>) -> Option<MenuInput>>, 
    transition_table: Box<dyn FnMut(&MenuState, &MenuInput, &mut Option<SystemUser>) -> MenuState>,
    starting_state:   MenuState,
    storage:          Option<SystemUser>
  ) -> Menu
  {
   Menu {
      state: starting_state,
      output_table,
      transition_table,
      storage
    }
  }
}


impl Automaton<MenuState, MenuInput, MenuInput, Option<SystemUser>> for Menu {

  fn storage(&mut self) -> &mut Option<SystemUser> {
    &mut self.storage
  }

  fn state(&mut self) -> &mut MenuState {
    &mut self.state
  }

  fn output_table(&mut self) 
    -> &mut Box<dyn FnMut(&MenuState, &MenuInput, &mut Option<SystemUser>) -> Option<MenuInput>> {
    &mut self.output_table
  }

  fn transition_table(&mut self) -> &mut Box<dyn FnMut(&MenuState, &MenuInput, &mut Option<SystemUser>) -> MenuState> {
    &mut self.transition_table
  }
}


fn main() {

  let transition_table = Box::new(|state: &MenuState, x: &MenuInput, _: &mut Option<SystemUser>| -> MenuState {
    match (state, x) {
      (Auth, Failed)               => Auth,
      (Auth, Success)              => Main,
      (Main, Logout)               => Auth,
      (Main, Subjects)             => Show,
      (Main, Teachers)             => Show,
      (Main, Reviews)             => Show,
      (Show, Back)                 => Main,
      (MenuState::Review, Success) => Main,
      (MenuState::Review, Back)    => Main,
      _ => Auth
    }
  });
  
  let output_table = Box::new(|state: &MenuState, x: &MenuInput, user: &mut Option<SystemUser>| -> Option<MenuInput> {
    let mut review_db = Db::new("postgresql://postgres:postgres@127.0.0.1/review");   
   
    let res = match (state, x) {
      (Auth, _) => auth::auth(&mut review_db, user),
      (Main, _) => {
        let option = make_choice(vec![
          "Show Reviews",
          "Show Subjects", 
          "Show Teachers", 
          "Make Review", 
          "Logout", 
          "Exit"
          ], "")
          .unwrap();
        
        match option {
          "Show Reviews" => Some(Reviews),
          "Show Subjects" => Some(Subjects), 
          "Show Teachers" => Some(Teachers), 
          "Make Review" => Some(MenuInput::Review), 
          "Logout" => Some(Logout), 
          "Exit" => Option::None,
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
          Reviews => {
            let reviews = review::get_reviews(&mut review_db);
            println!("Reviews from our community\n");
            for review in reviews {
              println!("By: {}\nTeacher: {}\nSubject: {}\nWith mark: {}\nReview:\n{}\n", 
                review.owner, 
                review.teacher, 
                review.subject, 
                review.mark,
                review.text 
              );
            } 
          }
          _ => unreachable!()
        }

        make_choice(vec!["Back"], "").unwrap();

        Some(Back)
      },
      (MenuState::Review, _) => review::review(&mut review_db, user.clone().unwrap().login.as_str())
    };

    clear_screen();

    res
  });
  
  let mut menu = Menu::new(output_table, transition_table, Auth, Option::None);
  let mut input = Some(None);

  while let Some(output) = menu.transition(input) {
    input = Some(output);
  }
}

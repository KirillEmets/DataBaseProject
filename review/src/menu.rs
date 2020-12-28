pub mod auth;
pub mod review;

use dialoguer::{
  Select,
  theme::ColorfulTheme
};
use console::Term;
pub use crate::*;

pub enum State {
  Auth,
  Main,
  Review,
  Show
}

pub enum Input {
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

pub struct Menu {
  pub storage:      Option<SystemUser>,
  state:            State,
}

impl Automaton<'_> for Menu {
  type State = State;
  type Input = Input;
  type Output = Input;
  type Storage = Option<SystemUser>;

  fn new(state: Self::State, storage: Self::Storage) -> Self {
    Menu {
      state,
      storage
    }
  }

  fn state_storage(&mut self) -> (&mut State, &mut Self::Storage) {
    (&mut self.state, &mut self.storage)
  }

  fn output_table(state: &State, input: &Input, user: &mut Self::Storage) -> Option<Input> {
    use Input::*;
    use State::*;

    let mut review_db = Db::new("postgresql://postgres:12hr56tf@127.0.0.1/review");   
   
    let res = match (state, input) {
      (Auth, _) => auth::auth(&mut review_db, user),
      (Main, _) => {
        let option = make_choice(vec![
          "Show Reviews",
          "Show Subjects", 
          "Show Teachers", 
          "Make Review", 
          "Logout", 
          "Exit"
          ], "");
        
        match option {
          "Show Reviews" => Some(Reviews),
          "Show Subjects" => Some(Subjects), 
          "Show Teachers" => Some(Teachers), 
          "Make Review" => Some(Input::Review), 
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

        make_choice(vec!["Back"], "");

        Some(Back)
      },
      (State::Review, _) => review::review(&mut review_db, user.clone().unwrap().login.as_str())
    };

    clear_screen();

    res
  }

  fn transition_table(state: &State, input: &Input, _user: &mut Self::Storage) -> State {
    use Input::*;
    use State::*;

    match (state, input) {
      (Auth, Failed)           => Auth,
      (Auth, Success)          => Main,
      (Main, Logout)           => Auth,
      (Main, Subjects)         => Show,
      (Main, Teachers)         => Show,
      (Main, Reviews)          => Show,
      (Main, Input::Review)    => State::Review,
      (Show, Back)             => Main,
      (State::Review, Success) => Main,
      (State::Review, Back)    => Main,
      _ => Auth
    }
  }
}

pub fn clear_screen() {
  let term = Term::stdout();
  term.clear_screen().expect("Failed to clear line");
}

pub fn clear_line() {
  let term = Term::stdout();
  term.move_cursor_up(1).expect("Cursor moving error");
  term.clear_line().expect("Failed to clear line");
}

pub fn make_choice<'a>(options: Vec<&'a str>, title: &str) -> &'a str {
  println!("{}", title);
  
  let option_index = Select::with_theme(&ColorfulTheme::default())
    .items(&options)
    .default(0)
    .interact().expect("Problems when handling choice");
  clear_line();

  options[option_index]
}
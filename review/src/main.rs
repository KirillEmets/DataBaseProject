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
}
use MenuState::*;
use MenuInput::*;

struct SystemUser {
  login: String
}

fn main() {
  let mut transition_table: HashMap<(MenuState, MenuInput), MenuState> = HashMap::new();
  let transitions = vec![
    ((Auth, Success)              , Main),
    ((Main, Logout)               , Auth),
    ((Main, MenuInput::Subjects)  , MenuState::Subjects),
    ((Main, MenuInput::Teachers)  , MenuState::Teachers),
    ((Main, MenuInput::Review)    , MenuState::Review),
    ((MenuState::Teachers, Back)  , Main),
    ((MenuState::Subjects, Back)  , Main),
    ((MenuState::Review, Success) , Main),
    ((MenuState::Review, Back)    , Main),
  ];
  for (i, j) in transitions {
    transition_table.insert(i, j);
  }

  let mut callbacks: HashMap<MenuState, Box<dyn Fn(&mut Automaton<MenuState, MenuInput>)>> = HashMap::new();

  callbacks.insert(Auth, Box::new(|automaton| {
    use auth::*;
    let mut review_db = Db::new("postgresql://postgres:postgres@127.0.0.1/review").unwrap();
    let option = make_choice(vec![
        "Login", 
        "Register"
      ], 
      "New here?"
    ).unwrap();
    auth(&option, &mut review_db);

    automaton.transition(Success);
    //automaton.transition("login_successful");
  }));

  callbacks.insert(Main, Box::new(|automaton| {
    let option = make_choice(vec![
      "Show Subjects", 
      "Show Teachers", 
      "Make Review", 
      "Logout", 
      "Exit"
      ], "")
      .unwrap();
    
    let option = match option {
      "Exit" => return,
      "Show Subjects" => MenuInput::Subjects, 
      "Show Teachers" => MenuInput::Teachers, 
      "Make Review" => MenuInput::Review, 
      "Logout" => Logout, 
      _ => unreachable!()
    };

    automaton.transition(option);
  }));

  callbacks.insert(MenuState::Teachers, Box::new(|automaton| {
    //display info

    make_choice(vec!["Back"], "").unwrap();

    automaton.transition(Back);
  }));
  
  callbacks.insert(MenuState::Subjects, Box::new(|automaton| {
    //display info
    
    make_choice(vec!["Back"], "").unwrap();

    automaton.transition(Back);
  }));

  callbacks.insert(MenuState::Review, Box::new(|automaton| {
    use review::*;
    //allow for input

    let res = review_menu();

    automaton.transition(Back);
    
    // match res {
    //   Ok("Post") => {
    //     // let res = post();
        
    //     // automaton.transition(res.unwrap());
    //     automaton.transition("Post_successful");
    //   },
    //   Ok("Back") => {
    //     automaton.transition("Back");
    //   },
    //   Ok(_) => unreachable!(),
    //   Err(_) => unreachable!()
    // }

  }));

  let menu = Automaton::new(callbacks, transition_table, Auth);
}

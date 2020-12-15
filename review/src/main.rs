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
  ShowTeachers,
  ShowSubjects
}
use MenuState::*;

struct SystemUser {
  login: String
}

fn main() {

  let mut transition_table: HashMap<(MenuState, &str), MenuState> = HashMap::new();
  let transitions = vec![
    ((Auth, "login_successful")  , Main),
    ((Auth, "registration_successful")  , Main),
    ((Auth, "login_failed")     , Auth),
    ((Main, "Logout")           , Auth),
    ((Main, "Show Subjects")    , ShowSubjects),
    ((Main, "Show Teachers")    , ShowTeachers),
    ((Main, "Make Review")      , Review),
    ((ShowTeachers, "Back")     , Main),
    ((ShowSubjects, "Back")     , Main),
    ((Review, "Post_successful") , Main),
    ((Review, "Post_failed")    , Review),
    ((Review, "Back")           , Main),
  ];
  for (i, j) in transitions {
    transition_table.insert(i, j);
  }

  let mut callbacks: HashMap<MenuState, Box<dyn Fn(&mut Automaton<MenuState, &str>)>>= HashMap::new();

  callbacks.insert(Auth, Box::new(|automaton| {
    use auth::*;
    let mut review_db = Db::new("postgresql://postgres:postgres@127.0.0.1/review").unwrap();
    let option = make_choice(vec![
        "Login", 
        "Register"
      ], 
      "New here?"
    ).unwrap();
    let auth_res = auth(&option, review_db);

    automaton.transition(&auth_res);
    //automaton.transition("login_successful");
  }));

  callbacks.insert(Main, Box::new(|automaton| {
    let option = make_choice(vec![
      "Show Subjects", 
      "Show Teachers", 
      "Make Review", 
      "Logout", 
      "Exit"
      ], "");
    
    if let Ok("Exit") = option { return; }

    automaton.transition(option.unwrap());
  }));
  
  callbacks.insert(ShowTeachers, Box::new(|automaton| {
    //display info

    let option = make_choice(vec!["Back"], "");

    automaton.transition(option.unwrap());
  }));
  
  callbacks.insert(ShowSubjects, Box::new(|automaton| {
    //display info

    let option = make_choice(vec!["Back"], "");

    automaton.transition(option.unwrap());
  }));

  callbacks.insert(Review, Box::new(|automaton| {
    use review::*;
    //allow for input

    let res = review_menu();
    
    match res {
      Ok("Post") => {
        // let res = post();
        
        // automaton.transition(res.unwrap());
        automaton.transition("Post_successful");
      },
      Ok("Back") => {
        automaton.transition("Back");
      },
      Ok(_) => unreachable!(),
      Err(_) => unreachable!()
    }

  }));

  let menu = Automaton::new(callbacks, transition_table, Auth);
}

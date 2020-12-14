extern crate dialoguer;
extern crate console;

mod menu;
mod automaton;

use crate::menu::auth::*;
use crate::automaton::*;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum MenuState {
  Auth,
  Main,
  Review,
  Show
}
use MenuState::*;

struct User {
  login: String
}

fn main() {
  let mut transition_table: HashMap<(MenuState, &str), MenuState> = HashMap::new();
  transition_table.insert((Auth, "login_succesful"), Main);
  transition_table.insert((Main, "Logout")         , Auth);

  let mut callbacks: HashMap<MenuState, fn(&mut Automaton<MenuState, &str>)> = HashMap::new();

  callbacks.insert(Auth, |automaton| {
    let option = make_choice(vec![
        "Login", 
        "Register"
      ], 
      "New here?"
    ).unwrap();
    let auth_res = auth(&option);

    // automaton.transition(auth_res);
    automaton.transition("login_succesful");
  });

  callbacks.insert(Main, |automaton| {
    let option = make_choice(vec![
        "Show Subjects", 
        "Show Teachers", 
        "Make Review", 
        "Logout", 
        "Exit"
      ], "");
    
    if let Ok("Exit") = option { return; }

    automaton.transition(option.unwrap());
  });

  let menu = Automaton::new(callbacks, transition_table, Auth);
}

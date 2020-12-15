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
  // let mut db = Db::new("postgresql://postgres:postgres@127.0.0.1/rust").unwrap();
  
  // let query = db.execute("SELECT * FROM SystemUser", &[]).unwrap();
  // for row in query {
  //   let user = User {
  //     id: row.get(0),
  //     name: row.get(1),
  //     password: row.get(2)
  //   };
  //   println!("{} {} {}", user.id, user.name, user.password);
  // }

  let mut transition_table: HashMap<(MenuState, &str), MenuState> = HashMap::new();
  let transitions = vec![
    ((Auth, "login_succesful")  , Main),
    ((Auth, "login_failed")     , Auth),
    ((Main, "Logout")           , Auth),
    ((Main, "Show Subjects")    , ShowSubjects),
    ((Main, "Show Teachers")    , ShowTeachers),
    ((Main, "Make Review")      , Review),
    ((ShowTeachers, "Back")     , Main),
    ((ShowSubjects, "Back")     , Main),
    ((Review, "Post_succesful") , Main),
    ((Review, "Post_failed")    , Review),
    ((Review, "Back")           , Main),
  ];
  for (i, j) in transitions {
    transition_table.insert(i, j);
  }

  let mut callbacks: HashMap<MenuState, fn(&mut Automaton<MenuState, &str>)> = HashMap::new();

  callbacks.insert(Auth, |automaton| {
    use auth::*;

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
  
  callbacks.insert(ShowTeachers, |automaton| {
    //display info

    let option = make_choice(vec!["Back"], "");

    automaton.transition(option.unwrap());
  });
  
  callbacks.insert(ShowSubjects, |automaton| {
    //display info

    let option = make_choice(vec!["Back"], "");

    automaton.transition(option.unwrap());
  });

  callbacks.insert(Review, |automaton| {
    use review::*;
    //allow for input

    let res = review_menu();
    
    match res {
      Ok("Post") => {
        let res = post();
        
        // automaton.transition(res.unwrap());
        automaton.transition("Post_succesful");
      },
      Ok("Back") => {
        automaton.transition("Back");
      },
      Ok(_) => unreachable!(),
      Err(_) => unreachable!()
    }

  });

  let menu = Automaton::new(callbacks, transition_table, Auth);
}

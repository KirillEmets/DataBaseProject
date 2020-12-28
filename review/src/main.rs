extern crate dialoguer;
extern crate console;

mod menu;
mod automaton;
mod db;

use crate::menu::*;
use crate::automaton::*;
use crate::db::*;

#[derive(Clone)]
pub struct SystemUser {
  login: String
}

fn main() {
  let mut menu = Menu::new(State::Auth, Option::None);
  let mut input = Some(Input::None);

  while let Some(output) = menu.transition(&input) {
    input = Some(output);
  }
}

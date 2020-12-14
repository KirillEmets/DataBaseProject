pub use std::collections::HashMap;
pub use std::cmp::Eq;
pub use std::hash::Hash;
pub use std::fmt::Debug;

pub struct Automaton<State, Input>
where 
  State: Eq + Hash + Copy + Debug, 
  Input: Eq + Hash + Copy + Debug 
{
  state: State,
  transition_table: HashMap<(State, Input), State>,
  callbacks: HashMap<State, fn(&mut Self)>
}

impl<T, U> Automaton<T, U> 
where 
  T: Eq + Hash + Copy + Debug, 
  U: Eq + Hash + Copy + Debug 
{
  pub fn new(callbacks: HashMap<T, fn(&mut Self)>, transition_table: HashMap<(T, U), T>, starting_state: T) -> Automaton<T, U> {
    let mut automaton = Automaton {
      state: starting_state,
      transition_table,
      callbacks
    };

    if let Some(callback) = automaton.callbacks.get(&automaton.state) {
      callback(&mut automaton);
    }

    automaton
  }

  pub fn transition(&mut self, x: U) { //takes some input
    //do what corresponds to that state
    //change state

    self.state = *self.transition_table
      .get(&(self.state, x))
      .expect(&format!("No transition from state {:?} with input {:?}", self.state, x));
    if let Some(callback) = self.callbacks.get(&self.state) {
      callback(self);
    }
  }
}
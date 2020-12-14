pub use std::collections::HashMap;
pub use std::cmp::Eq;
pub use std::hash::Hash;
pub use std::fmt::Debug;

pub struct Automaton<State, Input>
where 
  State: Eq + Hash + Copy + Debug, 
  Input: Eq + Hash + Copy + Debug 
{
  state:            State,
  transition_table: HashMap<(State, Input), State>,
  callbacks:        HashMap<State, fn(&mut Self)>
}

impl<State, Input> Automaton<State, Input>
where 
  State: Eq + Hash + Copy + Debug, 
  Input: Eq + Hash + Copy + Debug 
{
  pub fn new(
    callbacks:        HashMap<State, fn(&mut Self)>, 
    transition_table: HashMap<(State, Input), State>, 
    starting_state:   State
  ) -> Automaton<State, Input> 
  {
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

  //takes some input
  //change state
  //do what corresponds to that state
  pub fn transition(&mut self, x: Input) { 
    self.state = *self.transition_table
      .get(&(self.state, x))
      .expect(&format!("No transition from state {:?} with input {:?}", self.state, x));
      
    if let Some(callback) = self.callbacks.get(&self.state) {
      callback(self);
    }
  }
}
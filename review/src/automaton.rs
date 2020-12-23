pub use std::collections::HashMap;
pub use std::cmp::Eq;
pub use std::hash::Hash;
pub use std::fmt::Debug;

pub struct Automaton<State, Input, Output, Storage>
{
  storage:          Storage,
  state:            State,
  output_table:     Box<dyn FnMut(&State, &Input, &mut Storage) -> Output>,
  transition_table: Box<dyn FnMut(&State, &Input, &mut Storage) -> State>,
}

impl<State, Input, Output, Storage> Automaton<State, Input, Output, Storage>
{
  pub fn new(
    output_table:     Box<dyn FnMut(&State, &Input, &mut Storage) -> Output>, 
    transition_table: Box<dyn FnMut(&State, &Input, &mut Storage) -> State>,
    starting_state:   State,
    storage: Storage
  ) -> Automaton<State, Input, Output, Storage> 
  {
    Automaton {
      state: starting_state,
      output_table,
      transition_table,
      storage
    }
  }

  //take some input
  //change state
  //emit what corresponds to that state and input
  pub fn transition(&mut self, x: Option<Input>) -> Option<Output> { 
    match x {
      Some(x) => {
        self.state = (self.transition_table)(&self.state, &x, &mut self.storage);
        
        Some((self.output_table)(&self.state, &x, &mut self.storage))
      }
      None => None
    }
  }
}
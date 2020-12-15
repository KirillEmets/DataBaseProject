pub use std::collections::HashMap;
pub use std::cmp::Eq;
pub use std::hash::Hash;
pub use std::fmt::Debug;

pub struct Automaton<State, Input, Output>
where 
  State: Eq + Hash + Copy + Debug, 
  Input: Eq + Hash + Copy + Debug 
{
  state:            State,
  output_table:     Box<dyn FnMut(State, Input) -> Output>,
  transition_table: Box<dyn FnMut(State, Input) -> State>,
}

impl<State, Input, Output> Automaton<State, Input, Output>
where 
  State: Eq + Hash + Copy + Debug, 
  Input: Eq + Hash + Copy + Debug 
{
  pub fn new(
    output_table:     Box<dyn FnMut(State, Input) -> Output>, 
    transition_table: Box<dyn FnMut(State, Input) -> State>,
    starting_state:   State
  ) -> Automaton<State, Input, Output> 
  {
    Automaton {
      state: starting_state,
      output_table,
      transition_table,
    }
  }

  //take some input
  //change state
  //emit what corresponds to that state and input
  pub fn transition(&mut self, x: Option<Input>) -> Option<Output> { 
    match x {
      Some(x) => {
        self.state = (self.transition_table)(self.state, x);
        
        Some((self.output_table)(self.state, x))
      }
      None => None
    }
  }
}
pub trait Automaton<State: Clone, Input, Output, Storage: Clone>
{
  fn storage(&mut self) -> &mut Storage;

  fn state(&mut self) -> &mut State;

  fn output_table(&mut self) -> &mut Box<dyn FnMut(&State, &Input, &mut Storage) -> Option<Output>>;

  fn transition_table(&mut self) -> &mut Box<dyn FnMut(&State, &Input, &mut Storage) -> State>;

  fn transition(&mut self, x: Option<Input>) -> Option<Output> { 
    match x {
      Some(x) => {
        let state = self.state().clone();
        let mut storage = self.storage().clone();

        *self.state() = (self.transition_table())(&state, &x, &mut storage);
        
        (self.output_table())(&state, &x, &mut storage)
    
      }
      None => None
    }
  }

}

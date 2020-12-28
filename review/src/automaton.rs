pub trait Automaton<'a>
{
  type State: 'a;
  type Input;
  type Output;
  type Storage: 'a;

  fn new(starting_state: Self::State, storage: Self::Storage) -> Self;

  fn state_storage(&'a mut self) -> (&'a mut Self::State, &'a mut Self::Storage);

  fn output_table(state: &Self::State, input: &Self::Input, storage: &mut Self::Storage) -> Option<Self::Output>;

  fn transition_table(state: &Self::State, input: &Self::Input, storage: &mut Self::Storage) -> Self::State;

  fn transition(&'a mut self, x: &Option<Self::Input>) -> Option<Self::Output> { 
    match x {
      Some(x) => {
        let (state, storage) = self.state_storage();

        *state = Self::transition_table(state, x, storage);
        
        Self::output_table(state, x, storage)
      }
      None => None
    }
  }

}

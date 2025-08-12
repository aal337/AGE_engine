use crate::state::State;

pub struct StateConfig {
    
}

impl Default for StateConfig {
    fn default() -> Self {
        Self {
            
        }        
    }
}

impl StateConfig {
    #[inline]
    pub fn apply_to_state(self, state: State) -> State {
        State {
            //config params
            ..state
        }
    }
}

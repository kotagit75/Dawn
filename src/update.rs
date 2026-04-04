use crate::state::State;

pub enum Event {
    None,
}
pub enum Effect {
    None,
}

pub fn update(event: Event, state: State) -> (State, Vec<Effect>) {
    (state, vec![Effect::None])
}

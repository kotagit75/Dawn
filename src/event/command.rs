use tokio::sync::oneshot;

use crate::event::{Event, UpdateResult};

#[derive(Debug)]
pub enum Command {
    Event(Event),
    ApiRequest(Event, oneshot::Sender<UpdateResult>),
}

impl Command {
    pub fn into_event(&self) -> Event {
        match self {
            Command::Event(event) => event.clone(),
            Command::ApiRequest(event, _) => event.clone(),
        }
    }

    pub fn into_response_tx(self) -> Option<oneshot::Sender<UpdateResult>> {
        match self {
            Command::Event(_) => None,
            Command::ApiRequest(_, tx) => Some(tx),
        }
    }
}

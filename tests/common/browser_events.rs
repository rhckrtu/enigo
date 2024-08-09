use log::debug;
use serde::{Deserialize, Serialize};
use tungstenite::Message;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BrowserEvent {
    KeyDown(String),
    KeyUp(String),
    MouseDown(String),
    MouseUp(String),
    MouseMove(((i32, i32), (i32, i32))),
    MouseScroll((i32, i32)),
    Open,
    Close,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BrowserEventError {
    UnknownMessageType,
    InvalidMessageFormat,
    ParseError,
}

impl TryFrom<Message> for BrowserEvent {
    type Error = BrowserEventError;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message {
            Message::Close(_) => {
                debug!("Message::Close received");
                Ok(BrowserEvent::Close)
            }
            Message::Text(msg) => {
                debug!("Message::Text received");
                debug!("msg: {:?}", msg);
                ron::from_str(&msg).unwrap()
            }
            _ => {
                debug!("Other Message received");
                Err(BrowserEventError::UnknownMessageType)
            }
        }
    }
}

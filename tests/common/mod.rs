use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{Receiver, Sender};

use log::debug;
use serde::{Deserialize, Serialize};
use tungstenite::{accept, Message};

pub mod key;
pub mod mouse;

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

#[derive(Debug)]
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

                ron::from_str(&msg).ok_or(BrowserEventError::UnknownMessageType)?
            }
            _ => {
                debug!("Other Message received");
                Err(BrowserEventError::UnknownMessageType)
            }
        }
    }
}

#[allow(clippy::similar_names)]
fn handle_connection(stream: TcpStream, tx: &Sender<BrowserEvent>) {
    let mut websocket = accept(stream).unwrap();

    debug!("Waiting for messages");
    loop {
        let message = websocket.read().unwrap();
        debug!("Processing message");

        match message {
            Message::Close(_) => {
                debug!("Message::Close received");
                tx.send(BrowserEvent::Close).unwrap();
                debug!("Client disconnected");
                return;
            }
            Message::Text(msg) => {
                debug!("Message::Text received");
                debug!("msg: {msg:?}");
                let (key, data) = msg.split_once(':').unwrap();
                let be = match key {
                    "open" => BrowserEvent::Open,
                    "close" => BrowserEvent::Close, // Is this needed?
                    "keydown" => BrowserEvent::KeyDown(data.to_string()),
                    "keyup" => BrowserEvent::KeyUp(data.to_string()),
                    "mousedown" => BrowserEvent::MouseDown(data.to_string()),
                    "mouseup" => BrowserEvent::MouseUp(data.to_string()),
                    "mousemove" => {
                        // format is relx,rely|absx,absy
                        let (rel, abs) = data.split_once('|').unwrap();
                        let (relx, rely) = rel.split_once(',').unwrap();
                        let (absx, absy) = abs.split_once(',').unwrap();
                        BrowserEvent::MouseMove((
                            (relx.parse().unwrap(), rely.parse().unwrap()),
                            (absx.parse().unwrap(), absy.parse().unwrap()),
                        ))
                    }
                    "mousewheel" => {
                        // format is x,y
                        let (x, y) = data.split_once(',').unwrap();
                        BrowserEvent::MouseScroll((x.parse().unwrap(), y.parse().unwrap()))
                    }
                    _ => {
                        debug!("Other text received");
                        continue;
                    }
                };
                tx.send(be).unwrap();
            }
            _ => {
                debug!("Other Message received");
            }
        }
    }
}

pub fn launch_ws_server(tx: Sender<BrowserEvent>) {
    let listener = TcpListener::bind("127.0.0.1:26541").unwrap();
    debug!("TcpListener was created");

    match listener.accept() {
        Ok((stream, addr)) => {
            debug!("New connection was made from {addr:?}");
            handle_connection(stream, &tx);
        }
        Err(e) => {
            debug!("Connection failed: {e:?}");
        }
    }
}

/*pub fn launch_browser(rs: &Receiver<BrowserEvent>) {
    let url = &format!(
        "file://{}/tests/index.html",
        std::env::current_dir().unwrap().to_str().unwrap()
    );
    if !webbrowser::Browser::Firefox.exists() {
        debug!("Firefox is not installed");
    }
    if webbrowser::open_browser_with_options(
        webbrowser::Browser::Default,
        url,
        webbrowser::BrowserOptions::new().with_suppress_output(false),
    )
    .is_err()
    {
        panic!("Unable to open the browser");
    }
    debug!("Try opening test page");
    if rs.recv_timeout(std::time::Duration::from_millis(5000)) == Ok(BrowserEvent::Open) {
        debug!("Test page was opened");
    } else {
        panic!("Expected Open event");
    }
    /*loop {
        if rs
            .recv_timeout(std::time::Duration::from_millis(500))
            .is_err()
        {
            break;
        }
    }*/
    debug!("Done with launch function");
}
*/

use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::Sender;

use tungstenite::accept;

use super::browser_events::BrowserEvent;

pub fn launch_ws_server(tx: Sender<BrowserEvent>) {
    let listener = TcpListener::bind("127.0.0.1:26541").unwrap();
    println!("TcpListener was created");

    match listener.accept() {
        Ok((stream, addr)) => {
            println!("New connection was made from {addr:?}");
            handle_connection(stream, &tx);
        }
        Err(e) => {
            println!("Connection failed: {e:?}");
        }
    }
}

#[allow(clippy::similar_names)]
fn handle_connection(stream: TcpStream, tx: &Sender<BrowserEvent>) {
    let mut websocket = accept(stream).unwrap();

    println!("Waiting for messages");
    loop {
        let message = websocket.read().unwrap();
        println!("Processing message");

        match BrowserEvent::try_from(message) {
            Ok(browser_event) if browser_event == BrowserEvent::Close => {
                tx.send(browser_event).unwrap();
                return;
            }
            Ok(browser_event) => {
                tx.send(browser_event).unwrap();
            }
            Err(_) => {
                println!("Other text received");
            }
        }
    }
}

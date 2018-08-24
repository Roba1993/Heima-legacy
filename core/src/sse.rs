//! Server Side Event implementation for Rocket

use std::sync::mpsc::Receiver;
use std::io::{self, ErrorKind, Read};
use model::Event;
use rocket::response::{self, Response, Responder};
use rocket::Request;
use serde_json;


pub struct ServerEvent<T> {
    receiver: Receiver<T>,
    was_message: bool,
    open_message: Vec<u8>,
}

impl<T> ServerEvent<T> {
    pub fn new(receiver: Receiver<T>) -> ServerEvent<T> {
        ServerEvent {
            receiver: receiver,
            was_message: false,
            open_message: vec!()
        }
    }
}

impl<T: Event> Read for ServerEvent<T> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        // check if there is a message back in the buffer
        if self.open_message.len() < 1 {
            // check if the last message was a real message
            if self.was_message {
                // actual message is an empty message
                self.was_message = false;
                // empty the message
                for i in 0..buf.len() {
                    buf[i] = 0;
                }
                // return the empty message
                return Ok(buf.len());
            } else {
                // actual message is a real message
                self.was_message = true;
                // receive the message
                let msg = &self.receiver.recv()
                    .map_err(|_| io::Error::new(ErrorKind::BrokenPipe, "Can't receive the message"))?;


                let parameter = serde_json::to_string(&msg
                    .get_parameter())
                    .map_err(|_| io::Error::new(ErrorKind::InvalidData, "Can't convert the message"))?;

                // format into SSE format
                self.open_message = format!{"event: {}\ndata: {}\n\n", msg.get_command(), parameter}.into_bytes();
            }
        }

        // fill the message buffer correctly
        for i in 0..buf.len() {
            buf[i] = if self.open_message.len() > 0 {
                self.open_message.remove(0)
            } else {
                0
            };
        }

        // return the buffer
        Ok(buf.len())
    }
}

impl<'r, T: 'r + Event> Responder<'r> for ServerEvent<T> {

    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .raw_header("Access-Control-Allow-Origin", "*")
            .raw_header("Content-Type", "text/event-stream")
            .raw_header("Cache-Control", "no-cache")
            .streamed_body(self)
            .ok()
    }
}
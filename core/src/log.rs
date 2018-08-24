use error::Error;
use core::Reactor;
use model::{Event, SimpleEvent};
use std::thread;


pub fn start(reactor: &Reactor<SimpleEvent>) -> Result<(), Error> {
    let reactor = reactor.clone();

    // get the sender and receiver for the device handler
    let (_, receiver) = reactor.channel("*")?;

    // start a thread for the device handler
    thread::spawn(move || {
        loop {
            // receive an event and print it
            let event = match receiver.recv() {
                Ok(e) => e,
                Err(e) => {println!("Reactor error: {:?}", e); continue;}
            };

            println!("event: {}, {}, {:?}, {}", event.get_id(), event.get_req_id(), event.get_typ(), event.get_command());
        }
    });

    Ok(())
}
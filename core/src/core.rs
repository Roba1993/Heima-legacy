use std::sync::mpsc::{Sender, Receiver, channel};
use std::sync::{Arc, Mutex};
use std::clone::Clone;
use std::thread;
use error::Error;
use std::time::Duration;
use sse::ServerEvent;
use model::{Event, EventTyp};

// Internal data for the reactor
struct ReactorData<T> {
    sender_list: Vec<(Sender<T>, Vec<String>)>,
    sender: Sender<T>
}

#[derive(Clone)]
pub struct Reactor<T> {
    data: Arc<Mutex<ReactorData<T>>>,
}

impl<T: 'static + Event> Reactor<T> {

    pub fn new() -> Reactor<T> {
        // create the default channel
        let (sender, receiver): (Sender<T>, Receiver<T>) = channel();

        // create a new reactor-data type
        let data = Arc::new(Mutex::new(ReactorData {
            sender_list: Vec::new(),
            sender: sender
        }));

        // copy a reference for the execution thread
        let thread_data = data.clone();

        // start the endless thread to send a received massage to all receivers
        thread::spawn(move || {
            // endless loop the handle the data
            loop {
                let res = Reactor::reactor_handler_intern(&thread_data, &receiver);
                if res.is_err() {
                    println!("Reactor Loop produced the following error: {:?}", res);
                }
            }
        });

        // return the reactor
        Reactor {
            data: data
        }
    }

    fn reactor_handler_intern(data: &Arc<Mutex<ReactorData<T>>>, receiver: &Receiver<T>) -> Result<(), Error> {
        // receive message from the bus
        let message = receiver.recv()?;

        // get the data
        let mut data = data.lock().map_err(|_| Error::Lock)?;

        // loop over the sender list with while, because we need a check each round
        let mut i = 0;
        while i < data.sender_list.len() {
            // send the message
            if Reactor::send_intern(&data.sender_list, i, &message).is_err() {
                // check if the data is still available - the remove
                // command panics, if we are out of index....bad rust....
                match data.sender_list.get(i) {
                    Some(_) => {
                        // remove the entry
                        data.sender_list.remove(i);
                        // continue and to upcount, because the entrys in the list moved
                        continue;
                    },
                    None => {}
                }
            }
            // count one up
            i += 1;
        }

        Ok(())
    }

    fn send_intern(data: &Vec<(Sender<T>, Vec<String>)>, id: usize, message: &T) -> Result<bool, Error> {
        // try to get the right sender
        let data = data.get(id).ok_or(Error::Lock)?;

        // check if the command fits to the subscription
        if data.1.contains(&"*".to_string()) || data.1.contains(&message.get_command()) {
            // send the message
            data.0.send(message.clone()).map_err(|_| Error::Send)?;
            // return and data was written
            return Ok(true);
        }

        // return, but no data was written
        Ok(false)
    }

    pub fn channel<M: Into<PathBuilder>>(&self, messages: M) -> Result<(Sender<T>, Receiver<T>), Error> {
        // lock the data to use it
        let mut data = self.data.lock().map_err(|_| Error::Lock)?;

        // create a new channel to communicate with the bus
        let (sender, receiver): (Sender<T>, Receiver<T>) = channel();

        // push the new sender to the sender list
        data.sender_list.push((sender, messages.into().as_vec()));

        // return the bus
        Ok((data.sender.clone(), receiver))

        // free the lock
    }

    pub fn send_request(&self, event: T) -> Result<T, Error> {
        // rust bug?
        let mut event = event;

        event.set_typ(EventTyp::Request);
        let msg_id = event.get_id();

        // get a channel
        let (sender, receiver) = self.channel("*")?;

        // send the data
        sender.send(event).map_err(|_| Error::Send)?;

        let mut msg;
        loop {
            // receive the message
            msg = receiver.recv_timeout(Duration::from_secs(5))?;

            // check if the answer has the right id
            if (msg.get_typ() == EventTyp::Answer || msg.get_typ() == EventTyp::Publish) && msg.get_req_id() == msg_id {
                break;
            }
        }

        // return the message id
        Ok(msg)
    }

    pub fn get_server_event<M: Into<PathBuilder>>(&self, messages: M) -> Result<ServerEvent<T>, Error> {
        // lock the data to use it
        let mut data = self.data.lock().map_err(|_| Error::Lock)?;

        // create a new channel to communicate with the bus
        let (sender, receiver): (Sender<T>, Receiver<T>) = channel();

        // push the new sender to the sender list
        data.sender_list.push((sender, messages.into().as_vec()));

        Ok(ServerEvent::new(receiver))
    }
}

/******************** Path Builder ***********************/

pub struct PathBuilder {
    path: Vec<String>
}

impl PathBuilder {
    pub fn new<S: Into<String>>(path: S) -> PathBuilder {
        PathBuilder {
            path: vec!(path.into())
        }
    }

    pub fn add<S: Into<String>>(mut self, path: S) -> PathBuilder {
        self.path.push(path.into());
        self
    }

    pub fn as_vec(self) -> Vec<String> {
        self.path
    }
}

impl From<Vec<String>> for PathBuilder {
    fn from(vec: Vec<String>) -> Self {
        PathBuilder {
            path: vec
        }
    }
}

impl From<String> for PathBuilder {
    fn from(string: String) -> Self {
        PathBuilder {
            path: vec!(string)
        }
    }
}

impl<'a> From<&'a str> for PathBuilder {
    fn from(string: &'a str) -> Self {
        PathBuilder {
            path: vec!(string.to_string())
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mapping_one() {
        let reactor = Reactor::new();
        let e1 = SimpleEvent::new("Event1");
        let e2 = SimpleEvent::new("Event2");
        let e3 = SimpleEvent::new("Event1");

        let (tx, rx) = reactor.channel(vec!("Event1".to_string()));
        tx.send(e1);
        tx.send(e2);
        tx.send(e3);

        println!("recv: {:?}", rx.recv());
        println!("recv: {:?}", rx.recv());
        println!("recv: {:?}", rx.recv());
    }

    #[test]
    fn mapping_all() {
        let reactor = Reactor::new();
        let e1 = SimpleEvent::new("Event1");
        let e2 = SimpleEvent::new("Event2");
        let e3 = SimpleEvent::new("Event1");

        let (tx, rx) = reactor.channel(vec!("*".to_string()));
        tx.send(e1);
        tx.send(e2);
        tx.send(e3);

        println!("recv: {:?}", rx.recv());
        println!("recv: {:?}", rx.recv());
        println!("recv: {:?}", rx.recv());
    }
}

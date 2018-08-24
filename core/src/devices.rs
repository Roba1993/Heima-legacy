use serde_json::Value as JValue;
use serde_json;
use json::SimpleJson;
use error::Error;
use core::{PathBuilder, Reactor};
use model::{Event, SimpleEvent, EventTyp};
use std::thread;
use std::sync::mpsc::{Sender, Receiver};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;


pub fn start(reactor: &Reactor<SimpleEvent>) -> Result<(), Error> {
    let path = PathBuilder::new("get_devices")
        .add("add_device")
        .add("update_device");

    // get the sender and receiver for the device handler
    let (sender, receiver) = reactor.channel(path)?;

    // start a thread for the device handler
    thread::spawn(move || {
        // create a new device handler
        let mut dh = DeviceHandler::new(sender, receiver);

        loop {
            // handle the events
             match dh.handle() {
                 Ok(_) => {},
                 Err(e) => {println!("Device error: {:?}", e)}
             };
        }
    });

    Ok(())
}

struct DeviceHandler {
    devices: Vec<JValue>,
    sender: Sender<SimpleEvent>,
    receiver: Receiver<SimpleEvent>
}

impl DeviceHandler {
    /// Generate a new device handler with 
    fn new(sender: Sender<SimpleEvent>, receiver: Receiver<SimpleEvent>) -> DeviceHandler {
        // load the devices
        let devices = match DeviceHandler::load_devices() {
            Ok(d) => d,
            Err(_) => vec!()
        };

        // create the device handler
        DeviceHandler {
            devices: devices,
            sender: sender,
            receiver: receiver
        }
    }

    /// This function tries to read an event from the bus
    /// and maps this to the device funtions.
    fn handle(&mut self) -> Result<(), Error> {
        // receive the event
        let event = self.receiver.recv()?;

        // execute the right function for the event
        match event.get_command().as_str() {
            "get_devices" => { self.get_devices(event)? },
            "add_device" => { self.add_device(event)? },
            "update_device" => { self.update_device(event) ?},
            _ => { return Ok(()) }
        };
        Ok(())
    }

    // Returns the list of all messages over the bus as Answer.
    //
    // This function can return an error if the bus didn't accept the
    // answer message to send out.
    fn get_devices(&self, event: SimpleEvent) -> Result<(), Error> {
        // only answer to request
        if event.get_typ() != EventTyp::Request {
            return Ok(());
        }

        // send the devices
        let mut msg = SimpleEvent::new_with_parameter("get_devices", JValue::Array(self.devices.clone()));
        msg.set_req_id(event.get_id());
        msg.set_typ(EventTyp::Answer);
        self.sender.send(msg).map_err(|_| Error::Send)?;
        Ok(())
    }

    /// Adds a new deivce to the device list. When the device already
    /// existed, delete the old device and create the new one.
    ///
    /// This function publishes a new device event to notify everyone
    /// about this new device.
    fn add_device(&mut self, event: SimpleEvent) -> Result<(), Error> {
        // only answer to request
        if event.get_typ() != EventTyp::Request {
            return Ok(());
        }

        // check and fix prereq
        let event = self.check_prereq(event)?;

        // extract the device
        let device = event.get_parameter();

        // generate the id
        let id = format!("{}-{}", device.get_str("/provider"), device.get_str("/provider_id"));

        // remove all devices with the same id from the list
        self.devices.retain(|d| d.pointer("/id").and_then(|v| v.as_str()).unwrap_or("") != id);

        // clone the device to allow changes on it
        let mut device = device.clone();

        // add the id to the device
        device.set_str("id", id);

        // add the device to the list
        self.devices.push(device.clone());

        // sent new device event
        let mut msg = SimpleEvent::new_with_parameter("new_device", device);
        msg.set_req_id(event.get_id());
        msg.set_typ(EventTyp::Publish);
        self.sender.send(msg).map_err(|_| Error::Send)?;

        // save to file and end
        self.save_devices();
        Ok(())
    }

    /// Updates the value of a device based on the id or the 
    /// following prereq's:
    /// * provider
    /// * provider_id
    /// * typ
    ///
    /// When no existing device was found, create a new one.
    ///
    /// When the prereq's are not fullfilled, send error message over the bus.
    fn update_device(&mut self, event: SimpleEvent) -> Result<(), Error> {
        // only answer to request
        if event.get_typ() != EventTyp::Request {
            return Ok(());
        }

        // check and fix prereq
        let event = self.check_prereq(event)?;

        // get the needed values
        let device = event.get_parameter();

        // we need to know if the updated was successfull
        let mut changed = false;

        // update the device
        for ref mut d in &mut self.devices {
            // find the device in the device list
            if device.get_str("/provider") == d.get_str("/provider")
                    && device.get_str("/provider_id") == d.get_str("/provider_id")
                    && device.get_str("/typ") == d.get_str("/typ") {
                // update all values
                for (key, value) in device.as_object().unwrap().iter() {
                    d.as_object_mut().unwrap().insert(key.clone(), value.clone());
                }

                // sent update event
                let mut msg = SimpleEvent::new_with_parameter("update_device", d.clone());
                msg.set_req_id(event.get_id());
                msg.set_typ(EventTyp::Publish);
                self.sender.send(msg).map_err(|_| Error::Send)?;

                // update was successfull
                changed = true;
            }
        }

        // when the update was successfull
        if changed {
            // save to file and end
            self.save_devices();
            return Ok(());
        }


        // when no device was found, create new device
        self.add_device(event)
    }

    /// Checks if the follwing prereq's are available:
    /// * provider
    /// * provider_id
    /// * typ
    ///
    /// When there are not availabe, the function tries to get the 
    /// prereq's from an existing device over the id.
    /// When the reqreq's are not available, the function fails.
    fn check_prereq(&self, event: SimpleEvent) -> Result<SimpleEvent, Error> {
        // get the needed values
        let mut device = event.get_parameter();

        // check if all needed device data is Available
        if device.get_str("/provider") == "" 
                || device.get_str("/provider_id") == "" 
                || device.get_str("/typ") == "" {
            // when an id is avilable
            if device.get_str("/id").trim() != "" {
                // try to find an existing device and take his prereq
                for d in &self.devices {
                    if device.get_str("/id") == d.get_str("/id") {
                        device.set_str("/provider", d.get_str("/provider"));
                        device.set_str("/provider_id", d.get_str("/provider_id"));
                        device.set_str("/typ", d.get_str("/typ"));
                    }

                    // return updated event
                    return Ok(event);
                }
            }

            // Prepare error message
            let mut msg = SimpleEvent::new_with_parameter("error_device", json!({
                "status": "error",
                "code": "E0001",
                "description": "Missing parameter (provider, provider_id, typ) or (id)"
            }));

            // send the error message
            msg.set_req_id(event.get_id());
            msg.set_typ(EventTyp::Answer);
            self.sender.send(msg).map_err(|_| Error::Send)?;
            return Err(Error::Other("Prereq not available"))
        }

        Ok(event)
    }

    fn save_devices(&self) {
        // get the devices
        let devices = self.devices.clone();

        // save in a new thread, to continue with the device worker
        thread::spawn(move || {
            let mut file = File::create("devices.json").unwrap();
            file.write_all(serde_json::to_string_pretty(&devices).unwrap().as_bytes()).unwrap();
        });
    }

    fn load_devices() -> Result<Vec<JValue>, Error> {
        // load from file
        let file = File::open("devices.json")?;
        let buf_reader = BufReader::new(file);

        // convert to json
        let json : JValue = serde_json::from_reader(buf_reader)?;
        Ok(json.as_array().ok_or(Error::Other("Devices not readable"))?.clone())
    }
}
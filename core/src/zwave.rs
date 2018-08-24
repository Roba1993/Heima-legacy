use serde_json::Value as JValue;
use serde_json::map::Map;
use error::Error;
use std::time::Duration;
use std::thread;
//use std::time::SystemTime;
use rzw::driver::serial::SerialDriver;
use rzw::basic::Controller;
use rzw::cmds::CommandClass;
use rzw;
use std::sync::mpsc::Sender;
use core::Reactor;
use model::{Event, SimpleEvent, EventTyp};
use json::SimpleJson;

pub fn start(reactor: &Reactor<SimpleEvent>, path: String) -> Result<(), Error> {
    let reactor = reactor.clone();

    // get the sender and receiver for the device handler
    let (sender, receiver) = reactor.channel("set_switch")?;

    // start a thread for the device handler
    thread::spawn(move || {
        //let mut time = SystemTime::now();
        let mut err_num = 0u8;
        let mut zwave = get_zwave_device(&reactor, path.clone());

        loop {
            // receive an event
            let event = match receiver.recv() {
                Ok(e) => e,
                Err(e) => {println!("Reactor error: {:?}", e); continue;}
            };

            // handle the event
            match handle(&sender, event, &mut zwave) {
                // on success reset error number
                Ok(_) => {
                    err_num = 0u8;
                },
                // on failure count error up and recreate zwave device
                Err(e) => {
                    println!("Zwave device error: {:?}", e);
                    err_num += 1u8;
                    if err_num > 2 {
                        err_num = 0u8;
                        zwave = get_zwave_device(&reactor, path.clone());
                    }
                }
            };

            // THIS NEED TO BE DONE SEPERATLY / IN A OWN THREAD
            // get the devices status every 30 secs
            /*if SystemTime::now().duration_since(time).unwrap() > Duration::from_secs(30) {
                match get_devices(&reactor, &mut zwave) {
                    // on success reset error number
                    Ok(_) => {
                        err_num = 0u8;
                    },
                    // on failure count error up and recreate zwave device
                    Err(e) => {
                        println!("Zwave device error: {:?}", e);
                        err_num += 1u8;
                        if err_num > 2 {
                            err_num = 0u8;
                            zwave = get_zwave_device(&reactor, path.clone());
                        }
                    }
                };
                // reset the time
                time = SystemTime::now();
            }*/
        }
    });

    Ok(())
}


/// This function tries to connect every 10 seconds to the
/// zwave driver. When a driver was found, it returns the driver.
fn get_zwave_device(reactor: &Reactor<SimpleEvent>, path: String) -> Controller<SerialDriver> {
    loop {
        // try to open the zwave device
        let mut zwave = match rzw::open(path.clone()) {
            // on sucess return the device
            Ok(z) => z,
            // on error wait 10 seconds and try again
            Err(e) => {
                println!("{:?}", e);
                thread::sleep(Duration::from_secs(10));
                continue;
            }
        };

        // initial state request to get the devices
        match get_devices(&reactor, &mut zwave) {
            // on success return the zwave driver
            Ok(_) => {
                return zwave;
            },
            // on error wait 10 seconds and try again
            Err(e) => {
                println!("Zwave error: {:?}", e);
                thread::sleep(Duration::from_secs(10));
                continue;
            }
        };
    }
}

fn handle(sender: &Sender<SimpleEvent>, event: SimpleEvent, zwave: &mut Controller<SerialDriver>) -> Result<(), Error> {
    // execute the right function for the event
    match event.get_command().as_str() {
        "set_switch" => { set_switch(&sender, event, zwave)? },
        _ => { return Ok(()) }
    };
    Ok(())
}

// todo clean this function up
fn set_switch(sender: &Sender<SimpleEvent>, event: SimpleEvent, zwave: &mut Controller<SerialDriver>) -> Result<(), Error> {
    // only answer to request
    let msg_id = match event.get_typ() {
        EventTyp::Request => event.get_id(),
        _ => { return Ok(()) }
    };

    // get the parameter
    let mut parameter = event.get_parameter();

    // only process zwave devices
    let ptmp = parameter.clone();
    let vec: Vec<&str> = ptmp.get_str("/id").split('-').collect();
    if vec.get(0) != Some(&"zwave") || vec.get(1).is_none() {
        return Ok(());
    }

    let str_id = vec.get(1).unwrap();

    // update the parameter
    parameter.set_str("provider", "zwave");
    parameter.set_str("provider_id", str_id.clone());
    parameter.set_str("typ", "switch");

    // get the id as u8
    let id_num = str_id.clone().parse::<u8>()?;

    if parameter.get_str("/status") == "on" {
        zwave.node(id_num).ok_or(Error::Other("Couldn't get Zwave commands"))?.switch_binary_set(true)?;
    } else {
        zwave.node(id_num).ok_or(Error::Other("Couldn't get Zwave commands"))?.switch_binary_set(false)?;
    }

    // send update event
    let mut msg = SimpleEvent::new_with_parameter("update_device", parameter.clone());
    msg.set_typ(EventTyp::Request);
    sender.send(msg).map_err(|_| Error::Send)?;

    // prepare and send the answer message
    let mut msg = SimpleEvent::new_with_parameter("set_switch", json!({
        "status": "ok",
        "description": "Switch changed successfully"
    }));
    msg.set_req_id(msg_id);
    msg.set_typ(EventTyp::Answer);
    sender.send(msg).map_err(|_| Error::Send)?;

    Ok(())
}

// todo clean this function up
fn get_devices(reactor: &Reactor<SimpleEvent>, zwave: &mut Controller<SerialDriver>) -> Result<(), Error> {

    for i in zwave.nodes() {
        // get the commands for the node
        let commands = zwave.node(i).ok_or(Error::Other("Couldn't get Zwave commands"))?.get_commands();

        // only add the node, when it's a binary switch
        if commands.iter().any(|x| x == &CommandClass::SWITCH_BINARY) {
            // get the status
            let status = zwave.node(i).ok_or(Error::Other("Couldn't get Zwave commands"))?.switch_binary_get()?;

            // set device information
            let mut json = Map::new();
            json.insert("provider".to_string(), JValue::String("zwave".to_string()));
            json.insert("provider_id".to_string(), JValue::String(format!("{}", i)));
            json.insert("typ".to_string(), JValue::String("switch".to_string()));
            if status == true {
                json.insert("status".to_string(), JValue::String("on".to_string()));
            } else {
                json.insert("status".to_string(), JValue::String("off".to_string()));
            }

            // send to the bus
            let mut tmp = SimpleEvent::new_with_parameter("update_device", json);
            tmp.set_typ(EventTyp::Request);
            reactor.send_request(tmp)?;
        }
    }

    Ok(())
}

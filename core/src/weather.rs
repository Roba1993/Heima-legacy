use serde_json;
use json::SimpleJson;
use error::Error;
use core::Reactor;
use model::SimpleEvent;
use yahoo_weather as yw;
use std::time::Duration;
use std::thread;
use model::EventTyp;
use model::Event;


pub fn start<S>(reactor: &Reactor<SimpleEvent>, location: S) -> Result<(), Error>
where S: Into<String> {
    let location = location.into();
    let reactor = reactor.clone();
    let refresh_rate = Duration::from_secs(60 * 5);

    // start a thread for the device handler
    thread::spawn(move || {
        loop {
            // get the newest weather news
             match handle(&reactor, location.clone()) {
                 Ok(_) => {},
                 Err(e) => {println!("Weather error: {:?}", e)}
             }

            // wait for the next refresh
            thread::sleep(refresh_rate);
        }
    });

    Ok(())
}

fn handle<S: Into<String>>(reactor: &Reactor<SimpleEvent>, location: S) -> Result<(), Error> {
    let location = location.into();

    // get the weather data
    let mut parameter = serde_json::to_value(yw::get_weather(location.clone())?)?;

    // add device information
    parameter.set_str("provider", "yahoo_weather");
    parameter.set_str("provider_id", location);
    parameter.set_str("typ", "weather");

    // send to the bus
    let mut tmp = SimpleEvent::new_with_parameter("update_device", parameter);
    tmp.set_typ(EventTyp::Request);
    reactor.send_request(tmp)?;
    Ok(())
}

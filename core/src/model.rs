//! All models for the heima app

use std::clone::Clone;
use std::fmt::Debug;
//use serde::{Serialize, Deserialize};
use serde_json::Value as JValue;
use error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use toml;
use rocket::config::Environment;
use std::sync::atomic::{AtomicU64, Ordering, ATOMIC_U64_INIT};


/***************** Event & EventTyp ***********************/

pub trait Event : Clone + Send + Debug {
    fn get_id(&self) -> u64;
    fn get_req_id(&self) -> u64;
    fn set_req_id<I: Into<u64>>(&mut self, req_id: I);
    fn get_typ(&self) -> EventTyp;
    fn set_typ<T: Into<EventTyp>>(&mut self, typ: T);
    fn get_command(&self) -> String;
    fn set_command<C: Into<String>>(&mut self, command: C);
    fn get_parameter(&self) -> JValue;
    fn set_parameter<P: Into<JValue>>(&mut self, parameter: P);
}

#[derive(Clone, Debug, PartialEq)]
pub enum EventTyp {
    Request,
    Answer,
    Publish
}

/****************** Simple Event **************************/
static EVENT_COUNTER : AtomicU64 = ATOMIC_U64_INIT;

#[derive(Clone, Debug)]
pub struct SimpleEvent {
    id: u64,
    req_id: u64,
    typ: EventTyp,
    command: String,
    parameter: JValue
}

impl SimpleEvent {
    pub fn new<C: Into<String>>(command: C) -> SimpleEvent {
        SimpleEvent{
            id: EVENT_COUNTER.fetch_add(10, Ordering::SeqCst),
            req_id: 0,
            typ: EventTyp::Publish,
            command: command.into(),
            parameter: JValue::Null
        }
    }

    pub fn new_with_parameter<C: Into<String>, P: Into<JValue>>(command: C, parameter: P) -> SimpleEvent {
        SimpleEvent{
            id: EVENT_COUNTER.fetch_add(10, Ordering::SeqCst),
            req_id: 0,
            typ: EventTyp::Publish,
            command: command.into(),
            parameter: parameter.into()
        }
    }
}

impl Event for SimpleEvent {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn get_req_id(&self) -> u64 {
        self.req_id
    }

    fn set_req_id<I: Into<u64>>(&mut self, req_id: I) {
        self.req_id = req_id.into();
    }

    fn get_typ(&self) -> EventTyp {
        self.typ.clone()
    }

    fn set_typ<T: Into<EventTyp>>(&mut self, typ: T) {
        self.typ = typ.into();
    }

    fn get_command(&self) -> String {
        self.command.clone()
    }

    fn set_command<C: Into<String>>(&mut self, command: C) {
        self.command = command.into();
    }

    fn get_parameter(&self) -> JValue {
        self.parameter.clone()
    }

    fn set_parameter<P: Into<JValue>>(&mut self, parameter: P) {
        self.parameter = parameter.into();
    }
}

/***************************** Setting **************************/
#[derive(Debug, Serialize, Deserialize)]
pub struct Setting {
    env: Option<String>,
    port: Option<u16>,
    location: Option<String>,
    zwave: Option<String>
}

impl Setting {
    pub fn from_file() -> Setting {
        match Setting::load_from_file("config.toml".to_string()) {
            Ok(config) => config,
            Err(_) => Setting {
                env: None,
                port: None,
                location: None,
                zwave: None
            },
        }
    }

    fn load_from_file(file: String) -> Result<Setting, Error> {
        let file = File::open(file)?;
        let mut buf_reader = BufReader::new(file);
        let mut content = String::new();
        buf_reader.read_to_string(&mut content)?;
        Ok(toml::from_str(&content)?)
    }

    pub fn get_env(&self) -> Environment {
        if self.env == Some("prod".to_string()) {
            Environment::Production
        }
        else {
            Environment::Development
        }
    }

    pub fn get_port(&self) -> u16 {
        match self.port {
            Some(expr) => expr,
            None => 8888,
        }
    }

    pub fn get_location(&self) -> String {
        match self.location {
            Some(ref l) => l.clone(),
            None => "Berlin".to_string(),
        }
    }

    pub fn get_zwave(&self) -> String {
        match self.zwave {
            Some(ref l) => l.clone(),
            None => "/dev/tty.usbmodem1411".to_string(),
        }
    }
}

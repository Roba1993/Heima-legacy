use std::{io, num, sync};
use yahoo_weather::error::Error as YwError;
use serde_json;
use rzw;
use toml;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: io::Error) {
            from()
            description("io error")
            display("I/O error: {}", err)
            cause(err)
        }
        ParseInt(err: num::ParseIntError) {
            from()
            description("Parse integer error")
            display("Parse integer: {}", err)
            cause(err)
        }
        ParseFloat(err: num::ParseFloatError) {
            from()
            description("Parse float error")
            display("Parse float: {}", err)
            cause(err)
        }
        Recv(err: sync::mpsc::RecvError) {
            from()
            description("Parse receive error")
            display("Parse receive: {}", err)
            cause(err)
        }
        TryRecv(err: sync::mpsc::TryRecvError) {
            from()
            description("Parse try_receive error")
            display("Parse try_receive: {}", err)
            cause(err)
        }
        RecvTimeout(err: sync::mpsc::RecvTimeoutError) {
            from()
            description("Parse receive timeout error")
            display("Parse receive timeout: {}", err)
            cause(err)
        }
        Yw(err: YwError) {
            from()
            description("Parse receive yahoo_weather error")
            display("Parse yahoo_weather timeout: {}", err)
            cause(err)
        }
        SerdeJson(err: serde_json::Error) {
            from()
            description("Parse receive serde json error")
            display("Parse serde json timeout: {}", err)
            cause(err)
        }
        Zwave(err: rzw::error::Error) {
            from()
            description("Parse receive zwave error")
            display("Parse zwave error: {}", err)
            cause(err)
        }
        Toml(err: toml::de::Error) {
            from()
            description("Parse receive toml error")
            display("Parse toml error: {}", err)
            cause(err)
        }
        Send {
            description("Coudn't sent")
            display("Coudn't sent")
        }
        NoData {
            description("No data are returned")
            display("No data are returned")
        }
        Lock {
            description("The lock is poisened")
            display("The lock is poisened")
        }
        Other(descr: &'static str) {
            description(descr)
            display("Error {}", descr)
        }
    }
}

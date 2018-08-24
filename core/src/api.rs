use rocket::response::NamedFile;
use rocket::{State, Response};
use rocket::http::{Status, ContentType};
use rocket_contrib::Json;
use core::{Reactor};
use model:: SimpleEvent;
use sse::ServerEvent;
use error::Error;
use serde_json::Value as JValue;
use std::io;
use std::path::Path;
use model::Event;

use serde_json;
use std;


#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("www/index.html")
}

#[get("/<file>")]
fn files(file: String) -> Option<NamedFile> {
    NamedFile::open(Path::new("www/").join(file)).ok()
}

#[get("/api")]
fn api() -> &'static str {
    "Available commands:\n
    #get /cmd/<command> -> Executes the command and returns the result.
    #post /cmd/<command> -> Executes the command with parameter and returns the result.
    #get /events -> Streams all events as server send events."
}

#[options("/cmd")]
fn opt_cmd<'a>() -> Response<'a> {
    Response::build()
        .status(Status::Ok)
        .raw_header("Access-Control-Allow-Origin", "*")
        .raw_header("Access-Control-Allow-Headers", "Content-Type")
        .finalize()
}

#[get("/cmd/<command>")]
fn get_cmd(reactor: State<Reactor<SimpleEvent>>, command: String) -> Result<Response, Error> {
    // send the command to the bus and receive the answer as json
    //let json = Json(reactor.send_request(SimpleEvent::new(command))?.get_parameter());

    let t = reactor.send_request(SimpleEvent::new(command))?.get_parameter();
    let t = serde_json::to_string(&t).unwrap();
    
    let response = Response::build()
        .status(Status::Ok)
        .header(ContentType::JSON)
        .raw_header("Access-Control-Allow-Origin", "*")
        .sized_body(std::io::Cursor::new(t))
        .finalize();
    

    // get the respond from the json
    //let mut respond = json.respond().map_err(|_| Error::Other("Can't generate respond"))?;

    // define cors header
    //respond.set_raw_header("Access-Control-Allow-Origin", "*");

    // return the respond
    Ok(response)
}

#[post("/cmd/<command>", data = "<json>")]
fn post_cmd(reactor: State<Reactor<SimpleEvent>>, command: String, json: Json<JValue>) -> Result<Response, Error> {
    
    let t = reactor.send_request(SimpleEvent::new_with_parameter(command, json.into_inner()))?.get_parameter();
    let t = serde_json::to_string(&t).unwrap();
    
    let response = Response::build()
        .status(Status::Ok)
        .header(ContentType::JSON)
        .raw_header("Access-Control-Allow-Origin", "*")
        .sized_body(std::io::Cursor::new(t))
        .finalize();
    
    
    // send the command to the bus and receive the answer as json
    //let json = Json(reactor.send_request(SimpleEvent::new_with_parameter(command, json.into_inner()))?.get_parameter());

    // get the respond from the json
    //let mut respond = json.respond().map_err(|_| Error::Other("Can't generate respond"))?;

    // define cors header
    //respond.set_raw_header("Access-Control-Allow-Origin", "*");

    // return the respond
    Ok(response)
}

#[get("/events")]
fn events(reactor: State<Reactor<SimpleEvent>>) -> Result<ServerEvent<SimpleEvent>, Error> {
    reactor.get_server_event("*").map_err(|_| Error::Send)
}
//LOCAL IMPORT
use crate::colors::*;
use crate::messages::*;
use crate::agents::*;
use crate::cli::logo::print_logo;

// OTHER DEPENDENCIES
use rocket::config::{Config, Environment};
use rocket_contrib::json::{Json, JsonValue};
use std::fs::File;
use std::io::Read;
use lazy_static::lazy_static;
use std::sync::Mutex;
use rocket::Request;


lazy_static! {
    pub static ref INTERFACE: Mutex<String> = Mutex::new(String::new());
}


#[post("/agent_telemetry", format = "json", data = "<basemessage>")]
fn agent_telemetry(basemessage: Json<messages::BaseMessage>) -> JsonValue {
    let msg = basemessage.into_inner();
    let returnmsg = handle_msg(msg.clone());
    return json!(returnmsg);
}

#[post("/alert", format = "json", data = "<basemessage>")]
fn agent_alert(basemessage: Json<messages::BaseMessage>) -> JsonValue {
    let msg = basemessage.into_inner();
    let returnmsg = handle_msg(msg.clone());
    return json!(returnmsg);
}

#[post("/agent_update", format = "json", data = "<basemessage>")]
fn agent_update(basemessage: Json<messages::BaseMessage>) -> JsonValue {
    let msg = basemessage.into_inner();
    let returnmsg = handle_msg(msg.clone());
    return json!(returnmsg);
}

pub fn run(interface: String){
    let iface = format!("{}:9999", interface);

    let config = Config::build(Environment::Development)
    .address(interface.clone())
    .port(9999)
    .finalize();

    *INTERFACE.lock().unwrap() = iface.clone();

    rocket::custom(config.unwrap()).mount("/api", routes![agent_telemetry, agent_alert, agent_update]).launch();

}

fn handle_msg(msg: messages::BaseMessage) -> messages::BaseMessage{
    let mut returnmsg = messages::BaseMessage::default();

    let ptype = msg.payload_type.as_str();

    match ptype {
        "StatusCheckIn" => {
            let returnmsg = agents::stats_checkin(msg.clone());
            return returnmsg;
        }
        "InitialCheckin" => {
            agents::new_agent(msg.clone());
            returnmsg.payload_type = "ServerOk".to_string();
            return returnmsg;

        }
        "CmdResult" => {
            agents::job_results(msg.clone());
            returnmsg.payload_type = "ServerOk".to_string();
            return returnmsg;
        }
        "Alert" => {
            agents::handle_alerts(msg.clone());
            returnmsg.payload_type = "NoAction".to_string();
            return returnmsg;
        }

        _ => {
            returnmsg.payload_type = "NoAction".to_string();
            return returnmsg;
        }
    }
}



use crate::messages::messages::*;
use crate::user::agent::*;
use crate::user::core::*;
use std::collections::HashMap;
use std::sync::Mutex;


pub async fn send_alert (mut message: BaseMessage){

    let server_ip =  INTERFACE.lock().unwrap().clone();
    let server = format!("http://{}:9999/api/alert",server_ip);

    message.id = AGENT.lock().unwrap().id.clone();

    let client = reqwest::Client::new();
    let res: BaseMessage = client.post(&server)
        .json(&message)
        .send().await.unwrap().json().await.unwrap();


    let mut send_msg: BaseMessage = handle_msg(res).await;

    if send_msg.payload_type == "NoAction"{
        return;
    } 

    return;
}

pub async fn send_message (mut message: BaseMessage){

    let server_ip =  INTERFACE.lock().unwrap().clone();
    let server = format!("http://{}:9999/api/agent_update",server_ip);

    message.id = AGENT.lock().unwrap().id.clone();

    let client = reqwest::Client::new();
    let res: BaseMessage = client.post(&server)
        .json(&message)
        .send().await.unwrap().json().await.unwrap();


    let mut send_msg: BaseMessage = handle_msg(res).await;

    if send_msg.payload_type == "NoAction"{
        return;
    } 

    return;
}
//LOCAL IMPORTS
use crate::hunts;
use crate::messages::messages::*;
use crate::user::core;
use crate::user::comms;


//DEPENDENCIES
use log::*;
use async_std::task;
use std::time::Duration;
use lazy_static::lazy_static;
use std::sync::Mutex;
use tokio::*;
use futures::future::*;
lazy_static! {
    pub static ref INTERFACE: Mutex<String> = Mutex::new(String::new());
}

pub async fn monitor_run(server_ip: String){
    *INTERFACE.lock().unwrap() = server_ip.clone();

    info!("** Monitoring the System **");

    let agent = core::new_agent();
    let agent_msg = core::initial_checkin(agent);
    comms::send_message(agent_msg).await;


    let handles = vec![
        tokio::spawn(async { hunts::T1037::monitor_logon_scripts().await;}),
        tokio::spawn(async { hunts::T1136_001::monitor_create_accounts().await;}),
        tokio::spawn(async { hunts::T1531::monitor_delete_accounts().await;}),
        tokio::spawn(async { hunts::T1053_005::monitor_scheduled_tasks().await;}),
        tokio::spawn(async { hunts::T1543_003::monitor_create_system_process().await;}),

    ];


    join_all(handles).await;

    let dur = Duration::from_millis(1000);
    tokio::time::sleep(dur).await;
}




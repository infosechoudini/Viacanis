use crate::hunts;
use crate::user;

use std::time::Duration;
use log::*;
use smol;
use std::pin::Pin;
use async_std::task;
use async_std::prelude::*;

pub async fn monitor_run(){
    info!("** Monitoring the System **");

/*
    tokio::spawn( async {((hunts::createaccount::hunt_create_accounts().await))});
    tokio::spawn( async {((hunts::deleteaccount::hunt_delete_accounts().await))});
    tokio::spawn( async {((hunts::logonscripts::hunt_logon_scripts().await))});
*/
        task::spawn( async {((hunts::logonscripts::hunt_logon_scripts().await))});   
        task::spawn( async {((hunts::createaccount::hunt_create_accounts().await))});
        task::spawn( async {((hunts::deleteaccount::hunt_delete_accounts().await))});




    //let fut = join_all([f1, f2, f3]);

    //fut.await;



}
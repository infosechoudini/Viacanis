use crate::colors::*;
use crate::hunts;

use tokio;


pub fn monitor_run(){
    colors::note_logger("** Monitoring the System **".to_string());
    //tokio::spawn( async{hunts::create_account::hunt_create_accounts().await});

    hunts::createaccount::hunt_create_accounts();

}

pub mod colors;
pub mod hunts;
pub mod eventlog;

pub mod user;
pub use user::cli::*;
pub use user::dragosagent::*;



use clap::{Arg, App};
use std::thread;
use tokio;

#[tokio::main]
async fn main() {
    let matches = App::new("dragosagent")
                          .version("1.0")
                          .author("Harry Thomas @infosechoudini")
                          .about("Dragos Industrial Hunting Agent")
                          .arg(Arg::with_name("hunt")
                               .short("h")
                               .long("hunt")
                               .value_name("hunt")
                               .help("Hunt for malicious activity on the system")
                               .takes_value(true)
                               .default_value("None"))
                            .arg(Arg::with_name("monitor")
                               .short("m")
                               .long("monitor")
                               .value_name("monitor")
                               .help("Monitor the system for malicious activity, dispating hunts as changes are detected")
                               .takes_value(true)
                               .default_value("None"))
                          .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let hunt_option = matches.value_of("hunt").unwrap().to_owned();
    let monitor_option = matches.value_of("monitor").unwrap().to_owned();
    // Retrieve the IDs of all active CPU cores.

    if monitor_option != "None"{
        let _detached_thread = thread::spawn( move || {
            user::dragosagent::monitor_run();
        });
    }

    if hunt_option != "None"{
        let _detached_thread = thread::spawn( move || {
                user::dragosagent::monitor_run();
            });
    }

    user::cli::start_cli();

}

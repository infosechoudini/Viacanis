pub mod colors;
pub mod hunts;
pub mod eventlog;
pub mod messages;
pub mod user;
pub use user::cli::*;
pub use user::agent::*;

use clap::{Arg, App};
use log::*;
#[macro_use]
extern crate bitflags;

#[tokio::main]
async fn main() {
    let matches = App::new("viacanisagent")
                          .version("0.0.2")
                          .author("Harry Thomas @infosechoudini")
                          .about("Industrial Hunting Agent")
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
                            .arg(Arg::with_name("server")
                               .short("s")
                               .long("server")
                               .value_name("server")
                               .help("IP of Viacanis Server")
                               .takes_value(true)
                               .default_value("None"))
                          .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let hunt_option = matches.value_of("hunt").unwrap().to_owned();
    let monitor_option = matches.value_of("monitor").unwrap().to_owned();
    let server_ip = matches.value_of("server").unwrap().to_owned();

    env_logger::init();


    if monitor_option != "None"{
        user::agent::monitor_run(server_ip.clone()).await;
        user::cli::start_cli();
    }

    if hunt_option != "None"{
        info!("STARTING HUNT MODE");
        user::agent::hunt_run().await; 
        user::cli::start_cli(); 
    }
}

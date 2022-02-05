#![allow(dead_code, unused_imports, unused_variables, unused_mut, non_snake_case)]
//LOCAL IMPORTS
pub mod colors;
pub mod hunts;
pub mod eventlog;
pub mod messages;
pub mod user;
pub mod processes;
pub use user::cli::*;
pub use user::agent::*;

//DEPENDENCIES
use clap::{Arg, App};
use log::*;
#[macro_use]
extern crate bitflags;
use std::alloc;
use cap::Cap;

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");


#[global_allocator]
static ALLOCATOR: Cap<alloc::System> = Cap::new(alloc::System, usize::max_value());

#[tokio::main]
async fn main() {
    let matches = App::new("viacanisagent")
                          .version(VERSION.unwrap())
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

    // Limiting Alloc to the baseline of Antimalware Executable 
    // Windows Antimalware Exec hovers at 130MB
    ALLOCATOR.set_limit(50 * 1024 * 1024).unwrap();
    info!("Currently allocated: {}B", ALLOCATOR.allocated());

    if monitor_option != "None"{
        user::agent::monitor_run(server_ip.clone()).await;
        user::cli::start_cli();
    }

    if hunt_option != "None"{
        info!("STARTING HUNT MODE");
    }
}

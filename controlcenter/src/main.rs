#![allow(dead_code, unused_imports, unused_variables)]
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate prettytable;



pub mod server;
pub mod cli;
pub mod colors;
pub mod agents;
pub mod messages;
pub use server::*;
pub use cli::*;

use clap::{Arg, App}; //Might use this later
use std::thread;

pub const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

fn main() {

    let matches = App::new("Viacanis")
                          .version(VERSION.unwrap())
                          .author("Harry Thomas @infosechoudini")
                          .about("Blue Team Threat Hunting and Monitoring Tool")
                          .arg(Arg::with_name("standalone")
                               .short("ss")
                               .long("standalone")
                               .value_name("standalone")
                               .help("Sets server in standalone server mode")
                               .takes_value(false))
                            .arg(Arg::with_name("iface")
                               .short("i")
                               .long("interface")
                               .value_name("iface")
                               .help("Sets server ip")
                               .takes_value(true))
                          .get_matches();

        let interface = matches.value_of("iface").unwrap().to_owned();
        println!("Value for interface: {}", interface);

        if matches.is_present("standalone"){
            let _detached_thread = thread::spawn( move || {
                server::standalone_server::run(interface);
            });

            cli::cli::start_cli();

        } else {
            let _detached_thread = thread::spawn( move || {
                server::server::run();
            });

            cli::cli::start_cli();
        }
    

}
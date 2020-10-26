//T1037 - Boot or Logon Initialization Scripts
//Tactic::Persistence | Tactic::PrivilegeEscalation
 

use std::io;
use winreg::RegKey;
use winreg::enums::*;
use std::thread::sleep;
use std::time::Duration;
use std::str;


pub async fn hunt_logon_scripts() {
    println!("File extensions, registered in system:");

    loop{
        let system = RegKey::predef(HKEY_CURRENT_USER).open_subkey("Environment").unwrap();

        for (name, value) in system.enum_values().map(|x| x.unwrap()) {
            match name.as_str() {
                "UserInitMprLogonScript" => {
                                        let reg_sz_value = str::from_utf8(&value.bytes).unwrap();
                                            println!("{:?}", value.bytes);
                                            if reg_sz_value != "\0\0" {
                                            println!("{:?}", value)}
                                        }
                _ => continue,
            }
        }
        sleep(Duration::from_millis(200));
    }
}
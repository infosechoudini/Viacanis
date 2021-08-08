//T1037 - Boot or Logon Initialization Scripts
//Tactic::Persistence | Tactic::PrivilegeEscalation
 
use winreg::RegKey;
use winreg::enums::*;
use std::time::Duration;
use log::*;
use async_std::task;



pub async fn monitor_logon_scripts(){
    info!("LOGON SCRIPT");
    info!("File extensions, registered in system:");

    loop{
        let system = RegKey::predef(HKEY_CURRENT_USER).open_subkey("Environment").unwrap();

        for (name, value) in system.enum_values().map(|x| x.unwrap()) {
            match name.as_str() {
                "UserInitMprLogonScript" => {
                                        //let reg_sz_value = str::from_utf8(&value.bytes).unwrap();
                                        if value.bytes != [0x00,0x00] {
                                            info!("{:?}", hex::encode(value.bytes))}
                                        }
                _ => continue,
            }
        }
        let dur = Duration::from_millis(100000);
        task::sleep(dur).await;

    }
}


pub async fn hunt_logon_scripts(){
    info!("LOGON SCRIPT");
    info!("File extensions, registered in system:");

    let system = RegKey::predef(HKEY_CURRENT_USER).open_subkey("Environment").unwrap();

    for (name, value) in system.enum_values().map(|x| x.unwrap()) {
        match name.as_str() {
            "UserInitMprLogonScript" => {
                                    //let reg_sz_value = str::from_utf8(&value.bytes).unwrap();
                                    if value.bytes != [0x00,0x00] {
                                        info!("{:?}", hex::encode(value.bytes))}
                                    }
            _ => continue,
        }
    }
}
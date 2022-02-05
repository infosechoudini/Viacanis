
use std::mem::zeroed;

use winapi::um::debugapi::DebugActiveProcess;
use win32_error::*;
use log::*;


pub fn run_debugger(pid: u32){
    unsafe {

        let debug_bool = DebugActiveProcess(pid);

        if debug_bool != 0{
            let err = Win32Error::new();
            info!("EVENT ERROR: {}", err);
        }

        loop {
                let mut debug_event: winapi::um::minwinbase::LPDEBUG_EVENT = std::mem::zeroed();
                winapi::um::debugapi::WaitForDebugEvent(debug_event, winapi::um::winbase::INFINITE);

                match (*debug_event).dwDebugEventCode {
                    winapi::um::minwinbase::LOAD_DLL_DEBUG_EVENT => loaded_dll_event(*debug_event),
                    _ => continue,
                }

                winapi::um::debugapi::ContinueDebugEvent((*debug_event).dwProcessId, (*debug_event).dwThreadId, winapi::um::winnt::DBG_CONTINUE);
            }
    }
}


pub fn loaded_dll_event(debug_event: winapi::um::minwinbase::DEBUG_EVENT ){
    unsafe {
        let dll_debug_info = debug_event.u;
        let dll_info = dll_debug_info.LoadDll();
        let image_name = dll_info.lpImageName;
        info!("IMAGE NAME: {:#?}", &image_name);
    }


}
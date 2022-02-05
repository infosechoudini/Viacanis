
use std::process::Command;
use std::ffi::{CString, OsStr, c_void};
use std::os::windows::ffi::OsStrExt;
use std::fs::{self, File};
use std::io::Read;
use std::ptr;
use winapi::um;
use libc::{c_char, uintptr_t, uint32_t, size_t};
use ntapi::*;
use kernel32::*;
use std::mem::transmute; 
use std::panic;
use std::fmt;
use std::time::Duration;
use winapi::shared::minwindef::LPCVOID;

use winapi::um::minwinbase::SECURITY_ATTRIBUTES;
use winapi::um::winnt::PROCESS_CREATE_THREAD;
use winapi::um::winnt::PROCESS_VM_OPERATION;
use winapi::um::winnt::PROCESS_VM_WRITE;
use winapi::um::winnt::PROCESS_VM_READ;
use winapi::um::winnt::PROCESS_QUERY_INFORMATION;
use winapi::um::winnt::PROCESS_ALL_ACCESS;

use crate::message_box;


pub fn modify_peb(){
    let mut file = File::create("C:\\Users\\user01\\Desktop\\log.txt").unwrap();

    unsafe {
        let mut ppeb: ntapi::ntpebteb::PPEB = std::mem::zeroed();
        let process_basic_info_holder  = std::mem::zeroed();
        let process_info_class: ntapi::ntpsapi::PROCESSINFOCLASS = std::mem::zeroed();
        let process_handle = winapi::um::processthreadsapi::GetCurrentProcess();
        message_box("NICE" ,"GOT PROCESS");    

        let return_length = std::ptr::null_mut();
        ntapi::ntpsapi::NtQueryInformationProcess(process_handle, process_info_class, process_basic_info_holder,  std::mem::size_of_val(&process_basic_info_holder) as u32, return_length );
        message_box("NICE" ,"GOT NT QUERY");    

        let process_basic_info: *mut ntapi::ntpsapi::PROCESS_BASIC_INFORMATION = std::mem::transmute(&mut *process_basic_info_holder);        
        message_box("NICE" ,"TRANSMUTED BASIC INFO");    


        let process_struct_size = std::mem::size_of_val(&process_basic_info);

        message_box("STRUCT SIZE" ,&process_struct_size.to_string());    


        let ppeb: *mut ntapi::ntpebteb::PPEB = std::mem::transmute(&mut (*process_basic_info).PebBaseAddress);
        message_box("NICE" ,"GOT PEB ADDRESS");    

        let being_debugged: *mut u8 = std::mem::transmute(&mut (*(*ppeb)).BeingDebugged);
        message_box("BEING DEBUGGED" ,&being_debugged.as_ref().unwrap().to_string());    

        (*(*ppeb)).BeingDebugged = 0 as u8;
        message_box("NICE" ,"DEBUGGER SET TO ZERO");    

        message_box("NICE" ,"WRITING MEMORY");    

        let wpm_error = winapi::um::memoryapi::WriteProcessMemory(process_handle, (*(*ppeb)).ImageBaseAddress, ppeb.clone() as *mut winapi::ctypes::c_void, std::mem::size_of_val(&ppeb), 0 as _ );
        if wpm_error == winapi::shared::minwindef::FALSE{
            let error = format!("WRITE PROCESS MEMORY FAILED {:#?}", wpm_error);
            message_box("FAILES", &error);
        }
        message_box("NICE" ,"WRITE MEMORY COMPLETED");    
    }

}


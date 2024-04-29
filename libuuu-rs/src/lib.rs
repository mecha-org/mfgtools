#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

mod errors;

use std::{ffi::{CStr, CString}, os::raw, ptr::{null, null_mut}};
use anyhow::{bail, Result};
use crate::errors::{UUUError, UUUErrorCodes};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn get_version() -> Result<String> {
    let ver_ptr: *const raw::c_char = unsafe { uuu_get_version_string() }.into();
    let ver_str = unsafe { CStr::from_ptr(ver_ptr) };

    let version = match ver_str.to_str().map(|s| s.to_owned()) {
        Ok(v) => v,
        Err(_) => bail!(UUUError::new(UUUErrorCodes::VersionError, "failed to get version".to_owned())),
    };

    Ok(version)
}

pub fn get_last_err() -> Result<String> {
    let err_ptr: *const raw::c_char = unsafe { uuu_get_last_err_string() };
    let err_str = unsafe { CStr::from_ptr(err_ptr) };

    let error = match err_str.to_str().map(|s| s.to_owned()) {
        Ok(v) => v,
        Err(_) => bail!(UUUError::new(UUUErrorCodes::GetLastErrError, "failed to get last error".to_owned())),
    };

    Ok(error)
}

pub fn run_command(cmd: &str, dry_run: i32) -> Result<i32> {
    let cmd_str = CString::new(cmd).unwrap();
    let cmd_ptr: *const raw::c_char = cmd_str.as_ptr() as *const raw::c_char;

    let ret: raw::c_int = unsafe {
        println!("running command");
        uuu_run_cmd(cmd_ptr, dry_run.into())
    };

    Ok(ret.into())
}

pub fn run_script(script: &str, dry_run: i32) -> Result<i32> {
    let script_str = CString::new(script).unwrap();
    let script_ptr: *const raw::c_char = script_str.as_ptr() as *const raw::c_char;

    let ret: raw::c_int = unsafe {
        uuu_run_cmd_script(script_ptr, dry_run.into())
    };

    Ok(ret.into())
}

extern fn collect_device(path: *const raw::c_char,
    chip: *const raw::c_char,
    pro: *const raw::c_char,
    vid: u16,
    pid: u16,
    bcd: u16,
    p: *mut raw::c_void) -> raw::c_int {
        println!("device");
        0.into()
}

pub fn get_devices() {
    // let _ = unsafe {
    //     uuu_for_each_devices(Some(collect_device), null_mut())
    // };
}

extern fn notify(notify: uuu_notify, p: *mut raw::c_void) -> raw::c_int {
        unsafe {
            println!("notify type={:?}, status={:?}", notify.type_, notify.__bindgen_anon_1.status);
        };
    
        0.into()
}

pub fn register_notify_callback() {
    unsafe {
        uuu_register_notify_callback(Some(notify), null_mut());
    }
}

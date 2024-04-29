use std::fs;

use libuuu_rs::{get_devices, get_last_err, get_version, register_notify_callback, run_command, run_script};

fn run_command_test() {
    let ex = run_command("SDP: boot -f _flash.bin", 0);
    println!("error is {}", ex.unwrap());
}

fn run_script_test(script: String) {
    let ex = run_script(&script, 0);
    println!("error is {}", ex.unwrap());
}

fn get_last_err_test() {
    let e = get_last_err();
    println!("last err is {:?}", e.unwrap());
}

fn get_version_test() {
    let ver = get_version();
    if ver.is_err() {
        println!("version error {:?}", ver.as_ref().err());
        return;
    }

    println!("version {}", ver.unwrap());
}

fn get_devices_test() {
    let _ = get_devices();
}

fn read_script_from_file(filename: &str) -> String {
    let script = fs::read_to_string(filename).unwrap();
    script
}

fn main() {
    // get_devices_test();
    // let script = read_script_from_file("dummy.clst");
    register_notify_callback();
    // run_script_test(script);
    run_command_test();
    get_last_err_test();
}

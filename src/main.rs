use std::{io::{Read, Write}, process::Command};


/**
* In percent
*/
const CHANGE_STEP: u32 = 5;

const BRIGHTNESS_FILE: &str = "/sys/class/backlight/intel_backlight/brightness";
const MAX_BRIGHTNESS_FILE: &str = "/sys/class/backlight/intel_backlight/max_brightness";


fn make_comm_pipe() -> bool {
    let mut make_pipe_command = Command::new("mkfifo");
    make_pipe_command.arg("/run/brightness");
    make_pipe_command.status().expect("Unable to execute mkfifo").success()
}


fn change_comm_pipe_perms() -> bool {
    let mut change_perm_command = Command::new("chmod");
    change_perm_command.arg("666");
    change_perm_command.arg("/run/brightness");
    change_perm_command.status().expect("Unable to execute chmod").success()
}


fn get_max_brightness() -> u32 {
    std::fs::read_to_string(MAX_BRIGHTNESS_FILE).expect("Unable to read max brightness").replace("\n", "")
        .parse().expect("Could not parse max brightness")
}


fn read_current_brightness() -> u32 {
    std::fs::read_to_string(BRIGHTNESS_FILE).expect("Unable to read the current brightness").replace("\n", "")
        .parse().expect("Could not parse current brightness")
}


fn read_brightness(current_brightness: u32, step: u32, pipe_handle: &mut std::fs::File) -> Option<u32> {
    let mut contents = vec![];
    pipe_handle.read_to_end(&mut contents).ok()?;
    let raw_data = String::from_utf8(contents).ok()?.replace("\n", ""); 
    if raw_data == "inc" {
        return Some(current_brightness + step);
    }
    if raw_data == "dec" {
        if step > current_brightness { return Some(0); }
        return Some(current_brightness - step);
    }
    raw_data.parse().ok()
}


fn main() {
    if !make_comm_pipe() {
        println!("Could not create named pipe");
        std::process::exit(1);
    }
    if !change_comm_pipe_perms() {
        println!("Could not change named pipe perms");
        std::process::exit(1);
    }
    let max_brightness = get_max_brightness();
    let step = max_brightness / 100 * CHANGE_STEP;
    let mut current_brightness = read_current_brightness();
    loop {
        let mut pipe_handle = std::fs::File::open("/run/brightness").expect("Could not open the named pipe");
        match read_brightness(current_brightness, step, &mut pipe_handle) {
            Some(value) => current_brightness = value,
            None => { continue; },
        }
        if current_brightness > max_brightness { current_brightness = max_brightness; }
        let mut brightness_handle = std::fs::File::options().write(true).open(BRIGHTNESS_FILE).expect("Could not open the brightness file");
        brightness_handle.write(current_brightness.to_string().as_bytes()).expect("Could not change brightness");
    }
}

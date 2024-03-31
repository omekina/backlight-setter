use std::{io::{Read, Write}, process::Command};


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
    std::fs::read_to_string("/sys/class/backlight/intel_backlight/max_brightness").expect("Unable to read max brightness").replace("\n", "")
        .parse().expect("Could not parse max brightness")
}


fn read_brightness(pipe_handle: &mut std::fs::File) -> Option<u32> {
    let mut contents = vec![];
    pipe_handle.read_to_end(&mut contents).ok()?;
    let value = String::from_utf8(contents).ok()?.replace("\n", "").parse::<u32>().ok()?;
    Some(value)
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
    loop {
        println!("Loop");
        let mut pipe_handle = std::fs::File::open("/run/brightness").expect("Could not open the named pipe");
        let Some(requested_brightness) = read_brightness(&mut pipe_handle) else {
            continue;
        };
        if requested_brightness > max_brightness { continue; }
        let mut brightness_handle = std::fs::File::options().write(true).open("/sys/class/backlight/intel_backlight/brightness").expect("Could not open the brightness file");
        brightness_handle.write(requested_brightness.to_string().as_bytes()).expect("Could not change brightness");
    }
}

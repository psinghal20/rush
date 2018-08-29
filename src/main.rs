extern crate libc;
use std::process::Command;
use std::io;

fn main() {
    unsafe {
        let child_pid = libc::fork();
        
        if child_pid == 0 {
            println!("***Child***\nCurrent PID: {} and Child PID: {}\n", libc::getpid(), child_pid);
        }
        else {
            let mut return_status = 0;
            let return_pointer = &mut return_status as *mut i32;
            libc::waitpid(child_pid, return_pointer ,libc::WUNTRACED);
            println!("***Parent***\nCurrent PID: {} and Child PID: {}\n", libc::getpid(), child_pid);
        }

        let mut key_press = String::new();
        io::stdin().read_line(&mut key_press).expect("Failed");
    }
}

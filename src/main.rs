mod structs;

use std::{env, process, thread};
use std::sync::mpsc::channel;
use crate::structs::{Arguments, HELP_KEY_WORD};
use crate::structs::scan_port;


fn main() {
    println!("Hello, friend!\n");

    let args: Vec<String> = env::args().collect();
    let program_name = args[0].clone();
    let arguments = Arguments::new(&args)
        .unwrap_or_else(|error| {
            if error.contains(HELP_KEY_WORD) {
                process::exit(0);
            } else {
                eprintln!("{} problem while parsing argiments: {}", program_name, error);
                process::exit(0);
            }
        });

    let threads_num = arguments.threads;
    let ip_addr = arguments.ipaddr;
    let (t_sender, t_receiver) = channel();
    for i in 0..threads_num {
        let t_sender = t_sender.clone();

        thread::spawn(move || {
            scan_port(t_sender, i, ip_addr, threads_num);
        });
    }

    let mut out = vec![];
    drop(t_sender);
    for port in t_receiver {
        out.push(port);
    }

    println!();

    out.sort();
    for item in out {
        println!("{} is open port!", item);
    }
}

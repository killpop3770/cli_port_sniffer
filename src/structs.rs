use std::io;
use std::io::Write;
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::sync::mpsc::Sender;

const MAX_PORT_NUMBER: u16 = 65535;

pub struct Arguments {
    flag: String,
    pub ipaddr: IpAddr,
    pub threads: u16,
}

impl Arguments {
    pub fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("Too few arguments! Minimal is one...");
        } else if args.len() > 4 {
            return Err("Too many arguments! Maximum is three...");
        }

        let f = args[1].clone();
        return if let Ok(ipaddr) = IpAddr::from_str(&f) {
            Ok(Arguments {
                flag: "".to_string(),
                ipaddr,
                threads: 4,
            })
        } else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("--help") && args.len() == 2 {
                println!("Usage -j too how many threads you want
                \r\n -h or --help to show this help message...");
                Err("help")
            } else if flag.contains("-h") || flag.contains("--help") {
                Err("Too many arguments! Maximum is three...")
            } else if flag.contains("-j") {
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(addr) => addr,
                    Err(_) => return Err("Not valid ip address! Must be only v4 or v6")
                };
                let threads = match args[2].parse::<u16>() {
                    Ok(th) => th,
                    Err(_) => return Err("Not valid number of threads!")
                };
                Ok(Arguments {
                    flag,
                    ipaddr,
                    threads,
                })
            } else {
                Err("Invalid syntax!")
            }
        };
    }
}

pub fn scan_port(t_sender: Sender<u16>, start_port: u16, ip_addr: IpAddr, threads_num: u16) {
    let mut port = start_port + 1;
    loop {
        match TcpStream::connect((ip_addr, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                t_sender.send(port).unwrap();
            }
            Err(_) => {}
        }

        if MAX_PORT_NUMBER - port <= threads_num {
            break;
        }

        port += threads_num;
    }
}
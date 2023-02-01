use std::str::FromStr;
use std::sync::mpsc::{ channel, Sender };
use std::{ env, process };
use std::net::{ IpAddr, TcpStream };
use std::thread;
use std::io::{self, Write};

const MAX: u16 = 65535;

struct Arguments {
    flag: String,
    ip_address: IpAddr,
    threads: u16
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("not enougt arguments");
        } else if args.len() > 4 {
            return Err("too many arguments");
        }

        let flag = args[1].clone();
        if let Ok(address) = IpAddr::from_str(&flag) {
            let arguments = Arguments {
                flag: String::from(""),
                ip_address: address,
                threads: 4
            };
            return Ok(arguments);
        } else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                println!("Usage: -j to selct how many threads you want\r\n-h or -help to show this help message.");
                return Err("help");
            } else if flag.contains("-h") || flag.contains("-help") {
                return Err("too many arguments");
            } else if flag.contains("-j") {
                let ip_adress = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("not a valid IPADDR, must be IPv4 or IPv6")
                };

                let threads = match args[2].parse::<u16>() {
                    Ok(success) => success,
                    Err(_) => return Err("failed to parse thread number")
                };

                let arguments = Arguments {
                    flag: flag,
                    threads: threads,
                    ip_address: ip_adress
                };
                return Ok(arguments);
            } else {
                return Err("invalid syntax")
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let program = args[0].clone();

    let arguments = Arguments::new(&args).unwrap_or_else( |error| {
        if error.contains("help") {
            process::exit(0);
        } else {
            eprintln!("{} problem parsing arguments: {}", program, error);
            process::exit(0);
        }
    });

    let number_of_threads = arguments.threads;
    let (transmitter, receiver) = channel();
    for i in 0..number_of_threads {
        let transmitter = transmitter.clone();

        thread::spawn(move || {
            scan(transmitter, i, arguments.ip_address, number_of_threads);
        });
    }

    let mut out = vec![];
    drop(transmitter);

    for p in receiver {
        out.push(p);
    }

    println!("{}", arguments.flag);
    out.sort();
    for v in out {
        println!("{} is open", v);
    }
}

fn scan(_: Sender<u16>, port: u16, address: IpAddr, number_of_threads: u16) {
    let mut next_port: u16 = port + 1;
    loop {
        match TcpStream::connect((address, next_port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
            },
            Err(_) => {}
        }

        if (MAX - port) <= number_of_threads {
            break;
        } 
        next_port += number_of_threads;
    }
}

/*
Code to see the args input on terminal to run cargo

Code:
fn main() {
    let args: Vec<String> = env::args().collect();
    
    for i in &args {
        println!("{}", i);
    }

    println!("{:?}", args);
}

on runs the command: `cargo run -- -h`
gives a result:
target\debug\ip_sniff.exe
-h
["target\\debug\\ip_sniff.exe", "-h"]
*/
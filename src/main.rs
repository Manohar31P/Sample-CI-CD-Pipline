use std::env;
use std::process;
use std::thread;
use std::sync::mpsc::channel;

mod scan;


fn main(){
    let args: Vec<String> = env::args().collect();
    let f = args[0].clone();

    let arguments = scan::Arguments::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help") {
                process::exit(0);
            } else {
                eprintln!("{} problem parsing arguments: {}", f, err);
                process::exit(0);
            }
        }
    );

    let threads = arguments.threads;
    let ip = arguments.ip;
    let total_ports = arguments.total_ports;
    let port_to_scan = arguments.port_to_scan;

    // Create a simple streaming channel
    let (tx, rx) = channel();

    if total_ports == 1{
        let tx = tx.clone();
        scan::scan_single(tx, ip, port_to_scan);
        process::exit(0);
    }

    for i in 0..threads{
        let tx = tx.clone();
        thread::spawn(move || {
            scan::scan(tx, i, ip, threads, total_ports);
        });
    }

    let mut open_port = vec![];
    drop(tx);

    for port in rx{
        open_port.push(port);
    }

    open_port.sort();
    
    println!("\n Final List of open Ports: ");
    for port in open_port{
        println!("Port {} is OPEN!", port);
    }

}
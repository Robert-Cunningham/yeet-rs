use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// Port to free up
    port: u16,
}

use nix::{
    sys::signal::{self, Signal},
    unistd::Pid,
};
use procfs::net;

fn main() {
    let args = Args::parse();

    let udp = true;
    let tcp = true;

    let port = args.port;

    let inodes_tcp6: Vec<u64> = procfs::net::tcp6()
        .unwrap()
        .into_iter()
        .filter(|tcpentry| tcpentry.local_address.port() == port)
        .map(|tcpentry| tcpentry.inode)
        .collect();

    let inodes_tcp4: Vec<u64> = procfs::net::tcp()
        .unwrap()
        .into_iter()
        .filter(|tcpentry| tcpentry.local_address.port() == port)
        .map(|tcpentry| tcpentry.inode)
        .collect();

    let inodes_udp6: Vec<u64> = procfs::net::udp6()
        .unwrap()
        .into_iter()
        .filter(|tcpentry| tcpentry.local_address.port() == port)
        .map(|tcpentry| tcpentry.inode)
        .collect();

    let inodes_udp4: Vec<u64> = procfs::net::udp()
        .unwrap()
        .into_iter()
        .filter(|tcpentry| tcpentry.local_address.port() == port)
        .map(|tcpentry| tcpentry.inode)
        .collect();

    let mut inodes = Vec::<u64>::new();

    inodes.extend(inodes_tcp4);
    inodes.extend(inodes_tcp6);
    inodes.extend(inodes_udp4);
    inodes.extend(inodes_udp6);

    println!("{:?}", inodes);

    /*
    for p in procfs::net::tcp().unwrap() {
        println!("{:?} -> {:?}", p.local_address.port(), p.inode)
    }
    */

    for process in procfs::process::all_processes().unwrap() {
        /*
        match process.fd() {
            Ok(a) => {
                println!("ok");
            }
            Err(b) => {
                println!("error");
            }
        }
        */
        if let Ok(fds) = process.fd() {
            for fd in fds {
                if let procfs::process::FDTarget::Socket(k) = fd.target {
                    if inodes.contains(&k) {
                        println!("{:?}", process.pid);
                        signal::kill(Pid::from_raw(process.pid), Signal::SIGTERM).unwrap();
                    }
                }
            }
        }
    }

    println!("I yote {}", args.port);
}

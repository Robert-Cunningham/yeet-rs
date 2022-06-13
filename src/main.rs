use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Port to free up
    port: u16,
}

use nix::{
    sys::signal::{self, Signal},
    unistd::Pid,
};
use procfs::net::{TcpNetEntry, UdpNetEntry};

enum NetEntry {
    Tcp(TcpNetEntry),
    Udp(UdpNetEntry),
}

/*
trait NetEntry {
    fn port(&self) -> u16;
    fn inode(&self) -> u64;
}

impl NetEntry for TcpNetEntry {
    fn inode(&self) -> u64 {
        self.inode
    }
    fn port(&self) -> u16 {
        self.local_address.port()
    }
}

impl NetEntry for UdpNetEntry {
    fn inode(&self) -> u64 {
        self.inode
    }
    fn port(&self) -> u16 {
        self.local_address.port()
    }
}
*/

fn main() {
    let args = Args::parse();

    let udp = true;
    let tcp = true;

    let port = args.port;

    let mut net_entries = Vec::<NetEntry>::new();

    /*
    match procfs::net::tcp() {
        Ok(a) => net_entries.extend(a),
        Err(e) => eprintln!("Failed to get IPv4 data"),
    }

    net_entries.extend(procfs::net::tcp().unwrap());
    net_entries.extend(procfs::net::tcp6().unwrap());
    net_entries.extend(procfs::net::udp().unwrap());
    net_entries.extend(procfs::net::udp6().unwrap());
    */

    let inodes_tcp6 = procfs::net::tcp6()
        .unwrap()
        .into_iter()
        .filter(|tcpentry| tcpentry.local_address.port() == port)
        .map(|tcpentry| tcpentry.inode);

    let inodes_tcp4 = procfs::net::tcp()
        .unwrap()
        .into_iter()
        .filter(|tcpentry| tcpentry.local_address.port() == port)
        .map(|tcpentry| tcpentry.inode);

    let inodes_udp6 = procfs::net::udp6()
        .unwrap()
        .into_iter()
        .filter(|tcpentry| tcpentry.local_address.port() == port)
        .map(|tcpentry| tcpentry.inode);

    let inodes_udp4 = procfs::net::udp()
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

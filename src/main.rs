use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Port to free up
    port: u16,
}

use nix::{
    sys::signal::{self, Signal},
    unistd::{getuid, Pid, Uid},
};
use procfs::net::{TcpNetEntry, UdpNetEntry};

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

fn main() {
    let args = Args::parse();

    let tcp = true;
    let udp = true;
    let ipv4 = true;
    let ipv6 = true;

    let port = args.port;

    let mut target_socket_inodes = Vec::<u64>::new();

    if tcp && ipv6 {
        match procfs::net::tcp6() {
            Ok(tcpentries) => {
                let socket_inodes_tcp6 = tcpentries
                    .into_iter()
                    .filter(|tcpentry| tcpentry.local_address.port() == port)
                    .map(|tcpentry| tcpentry.inode);

                target_socket_inodes.extend(socket_inodes_tcp6)
            }
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
    }
    //    .unwrap()
    //    .into_iter()
    //    .filter(|tcpentry| tcpentry.local_address.port() == port)
    //    .map(|tcpentry| tcpentry.inode);

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
        .map(|tcpentry| tcpentry.inode);

    target_socket_inodes.extend(inodes_tcp4);
    //socket_inodes.extend(socket_inodes_tcp6);
    target_socket_inodes.extend(inodes_udp4);
    target_socket_inodes.extend(inodes_udp6);

    let mut open_port_processes = Vec::<i32>::new();

    for process in procfs::process::all_processes().unwrap() {
        if let Ok(open_fds) = process.fd() {
            for open_fd in open_fds {
                if let procfs::process::FDTarget::Socket(socket_inode) = open_fd.target {
                    if target_socket_inodes.contains(&socket_inode) {
                        open_port_processes.push(process.pid)
                    }
                }
            }
        }
    }

    if open_port_processes.len() == 0 {
        eprintln!("No processes found.");
        if !getuid().is_root() {
            println!("Consider re-running yeet as root.")
        }
    }

    open_port_processes.into_iter().for_each(|pid| {
        signal::kill(Pid::from_raw(pid), Signal::SIGTERM).unwrap();
    });

    println!("I yote {}", args.port);
}

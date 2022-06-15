use nix::{
    sys::signal::{self, Signal},
    unistd::{getuid, Pid},
};

pub fn get_pids(port: u16) -> Vec<i32> {
    let socket_inodes = get_socket_inodes_with_port(port);
    let open_port_processes = get_processes_with_sockets(socket_inodes);

    return open_port_processes;
}

fn get_socket_inodes_with_port(port: u16) -> Vec<u64> {
    let mut target_socket_inodes = Vec::<u64>::new();

    match procfs::net::tcp6() {
        Ok(tcpentries) => {
            let socket_inodes_tcp6 = tcpentries
                .into_iter()
                .filter(|tcpentry| tcpentry.local_address.port() == port)
                .map(|tcpentry| tcpentry.inode);

            target_socket_inodes.extend(socket_inodes_tcp6)
        }
        Err(e) => {
            eprintln!("Could not enumerate TCP IPv6 connections.\n{:?}", e);
        }
    }

    match procfs::net::tcp() {
        Ok(tcpentries) => {
            let socket_inodes_tcp4 = tcpentries
                .into_iter()
                .filter(|tcpentry| tcpentry.local_address.port() == port)
                .map(|tcpentry| tcpentry.inode);

            target_socket_inodes.extend(socket_inodes_tcp4)
        }
        Err(e) => {
            eprintln!("Could not enumerate TCP IPv4 connections.\n{:?}", e);
        }
    }

    match procfs::net::udp6() {
        Ok(tcpentries) => {
            let socket_inodes_udp6 = tcpentries
                .into_iter()
                .filter(|tcpentry| tcpentry.local_address.port() == port)
                .map(|tcpentry| tcpentry.inode);

            target_socket_inodes.extend(socket_inodes_udp6)
        }
        Err(e) => {
            eprintln!("Could not enumerate UDP IPv6 connections.\n{:?}", e);
        }
    }

    match procfs::net::udp() {
        Ok(tcpentries) => {
            let socket_inodes_udp4 = tcpentries
                .into_iter()
                .filter(|tcpentry| tcpentry.local_address.port() == port)
                .map(|tcpentry| tcpentry.inode);

            target_socket_inodes.extend(socket_inodes_udp4)
        }
        Err(e) => {
            eprintln!("Could not enumerate UDP IPv4 connections.\n{:?}", e);
        }
    }

    return target_socket_inodes;
}

fn get_processes_with_sockets(target_socket_inodes: Vec<u64>) -> Vec<i32> {
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

    return open_port_processes;
}

pub fn send_sigterm(target_pids: &Vec<i32>) {
    target_pids.into_iter().for_each(|pid| {
        signal::kill(Pid::from_raw(*pid), Signal::SIGTERM).unwrap();
    });
}

pub fn send_sigkill(target_pids: &Vec<i32>) {
    target_pids.into_iter().for_each(|pid| {
        signal::kill(Pid::from_raw(*pid), Signal::SIGKILL).unwrap();
    });
}

pub fn is_root() -> bool {
    getuid().is_root()
}

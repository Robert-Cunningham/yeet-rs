use regex::Regex;
use std::process::Command;

pub fn get_pids(port: u16) -> Vec<i32> {
    let netstat = Command::new("netstat")
        .arg("-nao")
        .output()
        .expect("Failed to run netstat.");

    let raw_netstat_string = String::from_utf8(netstat.stdout.as_slice().to_vec())
        .expect("Failed to parse output from lsof into utf8 string.");

    //   TCP    0.0.0.0:135            0.0.0.0:0              LISTENING       1124
    let re = Regex::new(
        r"(?x)
        \s*(?P<protocol>\S+)\s+ # TCP
        (?P<source_ip>\S+):(?P<source_port>\d+)\s+ # source IP
        \S+\s+ # dest IP
        \S+\s+ # port status
        (?P<pid>\d*) # pid 
    ",
    )
    .unwrap();

    let out: Vec<i32> = raw_netstat_string
        .split('\n')
        .skip(1)
        .filter_map(|netstat_entry| {
            let captures = re
                .captures(netstat_entry)
                .expect("Netstat line doesn't match regex?");

            let p = str::parse::<u16>(&captures["port"]).expect("Invalid port");

            if p == port {
                return Some(str::parse::<i32>(&captures["pid"]).expect("Invalid PID"));
            } else {
                return None;
            };
        })
        .collect();

    return out;
}

pub fn send_sigterm(target_pids: &Vec<i32>) {}

pub fn send_sigkill(target_pids: &Vec<i32>) {}

pub fn is_root() -> bool {
    false
}

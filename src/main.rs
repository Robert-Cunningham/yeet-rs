use clap::Parser;

#[cfg_attr(target_os = "linux", path = "linux.rs")]
#[cfg_attr(target_os = "windows", path = "windows.rs")]
#[cfg_attr(target_os = "macos", path = "macos.rs")]
mod os;

use crate::os::{get_pids, is_root, send_sigkill, send_sigterm};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Port to yeet
    port: u16,
}

fn main() {
    let args = Args::parse();

    let processes = get_pids(args.port);

    if processes.len() == 0 {
        eprintln!("No processes found.");
        if !is_root() {
            eprintln!("Consider re-running yeet as root.")
        }
    }

    send_sigterm(&processes);

    //send_sigkill(&processes);
}

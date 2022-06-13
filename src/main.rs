use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// Port to free up
    port: u16,
}

fn main() {
    let args = Args::parse();

    let port = args.port;

    let inodes: Vec<u64> = procfs::net::tcp()
        .unwrap()
        .into_iter()
        .filter(|tcpentry| tcpentry.local_address.port() == port)
        .map(|tcpentry| tcpentry.inode)
        .collect();

    /*
    for p in procfs::net::tcp().unwrap() {
        println!("{:?} -> {:?}", p.local_address.port(), p.inode)
    }
    */

    for process in procfs::process::all_processes().unwrap() {
        println!("{:?}", process.fd().unwrap());
        for fd in process.fd().unwrap() {
            match fd.target {
                procfs::process::FDTarget::Socket(k) => println!("{}", k),
                _ => println!("Yo"),
            }
        }
    }

    println!("Yeet {}", args.port);
}

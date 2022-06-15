use std::process::Command;

pub fn get_pids() {
    let lsof = Command::new("lsof")
        .arg("-t")
        .arg("-i:5000")
        .output()
        .expect("Failed to execute lsof");

    let raw_pids_string = String::from_utf8(lsof.stdout.as_slice().to_vec())
        .expect("Failed to parse output from lsof into utf8 string.");

    let pids: Vec<i32> = raw_pids_string
        .split('\n')
        .filter_map(|s| str::parse::<i32>(&s).ok())
        .collect();

    pids.iter().for_each(|pid| {
        Command::new("kill")
            .arg(pid.to_string())
            .status()
            .expect("Failed to kill process.");
    });

    println!("{:?}", pids)
}

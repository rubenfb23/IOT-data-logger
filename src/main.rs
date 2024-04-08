use std::fs::File;
use std::io::Write;
use sysinfo::{CpuRefreshKind, RefreshKind, System};
use chrono::Local;
fn main() {

        let mut i = 0;

        let mut s =
            System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()));

        let mut file = File::create("cpu_temp_file.txt").expect("Unable to create file");
    
    loop {

        let time = Local::now();

        std::thread::sleep(std::time::Duration::from_secs(1));

        s.refresh_cpu();
        
        i+=1;

        writeln!(file, "------------------LINE {} ---- {} -----------------", i, time).expect("Unable to write to file");

        writeln!(file, "CPU USAGE").expect("Unable to write to file");

        for cpu in s.cpus() {
            writeln!(file, "{}: {}%", cpu.name(), cpu.cpu_usage())
                .expect("Unable to write to file");
        }}
        
        writeln!(file, "TEMPERATURES").expect("Unable to write to file");



    }
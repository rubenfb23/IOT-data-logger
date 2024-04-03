use std::fs::File;
use std::io::Write;
use std::process::Command;
use sysinfo::{CpuRefreshKind, RefreshKind, System};
fn main() {
    loop {
        let mut s =
            System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()));

        std::thread::sleep(std::time::Duration::from_secs(1));

        s.refresh_cpu();

        let mut file = File::create("output.txt").expect("Unable to create file");

        for cpu in s.cpus() {
            writeln!(file, "{}: {}%", cpu.name(), cpu.cpu_usage())
                .expect("Unable to write to file");
        }
        writeln!(file, "--------------------------------------").expect("Unable to write to file");

        let output = Command::new("shutdown")
            .arg("-h")
            .arg("now")
            .output()
            .expect("Failed to execute command");

        writeln!(file, "status: {}", output.status).expect("Unable to write to file");
        writeln!(file, "stdout: {}", String::from_utf8_lossy(&output.stdout))
            .expect("Unable to write to file");
        writeln!(file, "stderr: {}", String::from_utf8_lossy(&output.stderr))
            .expect("Unable to write to file");
    }
}

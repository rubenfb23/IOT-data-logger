use sysinfo::{System, SystemExt, ProcessorExt, ComponentExt};
use serde_json::json;
use reqwest;
use std::{time::{SystemTime, UNIX_EPOCH, Duration}, thread::sleep};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // API endpoint to send data
    let api = "https://api.webhookinbox.com/i/qRGUgIjq/in/";

    // Duration and sampling interval
    let interval = Duration::from_secs(5); // Interval between samples

    loop {
        // Initialize system information
        let mut sys = System::new_all();

        let mut cpu_usage_samples = vec![];
        let mut cpu_temp_samples = vec![];
        let start_time = SystemTime::now();

        while SystemTime::now().duration_since(start_time)? < Duration::from_secs(60) {
            sys.refresh_all();

            // Gather CPU usage
            let total_usage: f32 = sys.processors().iter()
                .map(|proc| proc.cpu_usage())
                .sum();
            let average_usage = total_usage / sys.processors().len() as f32;
            cpu_usage_samples.push(average_usage);

            // Gather CPU temperature
            let total_temp: f32 = sys.components().iter()
                .filter(|comp| comp.label().contains("CPU"))
                .map(|comp| comp.temperature())
                .sum();
            let average_temp = if !sys.components().is_empty() {
                total_temp / sys.components().len() as f32
            } else {
                0.0
            };
            cpu_temp_samples.push(average_temp);

            sleep(interval);
        }

        // Calculate final averages
        let final_cpu_usage: f32 = cpu_usage_samples.iter().sum::<f32>() / cpu_usage_samples.len() as f32;
        let final_cpu_temperature: f32 = cpu_temp_samples.iter().sum::<f32>() / cpu_temp_samples.len() as f32;

        // Prepare data for HTTP POST
        let now = SystemTime::now();
        let timestamp = now.duration_since(UNIX_EPOCH)?.as_secs();

        let data = json!({
            "time": timestamp,
            "average_cpu_usage": final_cpu_usage,
            "average_cpu_temperature": final_cpu_temperature,
        });

        // Send data via HTTP POST
        let client = reqwest::Client::new();
        let res = client.post(api)
            .header("Content-Type", "application/json")
            .json(&data)
            .send()
            .await?;

        println!("Response: {:?}", res);
    }
}

use clap::{Parser, ValueEnum};
use colored::*;
use rand::RngCore;
use std::time::Instant;
use thiserror::Error;

#[derive(Error, Debug)]
enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
}

#[derive(ValueEnum, Clone, Debug, Default, PartialEq)]
enum Region {
    #[default]
    Auto,
    Russia,
    Europe,
    Usa,
    China,
}

struct Endpoints {
    ping: &'static str,
    download: &'static str,
    upload: &'static str,
}

impl Region {
    fn ping_url(&self) -> &'static str {
        match self {
            Region::Auto | Region::Russia => "https://ya.ru",
            Region::Europe => "https://cloudflare.com",
            Region::Usa => "https://google.com",
            Region::China => "https://baidu.com",
        }
    }

    fn endpoints(&self) -> Endpoints {
        match self {
            Region::Russia => Endpoints {
                ping: "https://ya.ru",
                download: "https://speedtest.selectel.ru/10MB",
                upload: "https://postman-echo.com/post",
            },
            Region::Europe => Endpoints {
                ping: "https://cloudflare.com",
                download: "https://proof.ovh.net/files/10Mb.dat",
                upload: "https://postman-echo.com/post",
            },
            Region::Usa => Endpoints {
                ping: "https://google.com",
                download: "https://cachefly.cachefly.net/10mb.test",
                upload: "https://postman-echo.com/post",
            },
            Region::China => Endpoints {
                ping: "https://baidu.com",
                download: "https://mirrors.tuna.tsinghua.edu.cn/speedtest/10mb.bin",
                upload: "https://postman-echo.com/post",
            },
            Region::Auto => unreachable!(),
        }
    }
}

#[derive(Parser, Debug)]
#[command(name = "net-speed-test", about = "CLI network speed and ping tester")]
struct Cli {
    #[arg(long, short, default_value = "auto")]
    region: Region,

    #[arg(long, default_value_t = 15)]
    timeout: u64,
}

const UPLOAD_SIZE_BYTES: usize = 1 * 1024 * 1024;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), AppError> {
    let cli = Cli::parse();

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(cli.timeout))
        .build()?;

    let target_region = if cli.region == Region::Auto {
        print!("{} ", "Auto-detecting best region...".dimmed());
        let best = detect_best_region(&client).await;
        println!("{}", format!("{} (lowest latency)", format!("{:?}", best).green().bold()));
        best
    } else {
        cli.region
    };

    let endpoints = target_region.endpoints();

    println!("{}", "NetSpeed Test CLI".bright_cyan().bold());
    println!("{}", "===================".bright_cyan());
    println!("Active Region: {:?}", target_region);
    println!();

    print!("{} ", "Measuring ping...".dimmed());
    let ping_ms = measure_ping(&client, endpoints.ping).await?;
    println!("{}", format_ping(ping_ms));

    print!("{} ", "Measuring download speed...".dimmed());
    let download_mbps = measure_download(&client, endpoints.download).await?;
    println!("{}", format_speed(download_mbps));

    print!("{} ", "Measuring upload speed...".dimmed());
    let upload_mbps = measure_upload(&client, endpoints.upload).await?;
    println!("{}", format_speed(upload_mbps));

    println!("\n{}", "Test completed successfully!".green().bold());
    Ok(())
}

async fn detect_best_region(client: &reqwest::Client) -> Region {
    let candidates = [
        (Region::Russia, "https://ya.ru"),
        (Region::Europe, "https://cloudflare.com"),
        (Region::Usa, "https://google.com"),
        (Region::China, "https://baidu.com"),
    ];

    let mut best = Region::Russia;
    let mut min_ms = u128::MAX;

    for (region, url) in candidates {
        let start = Instant::now();
        if client.get(url).send().await.is_ok() {
            let ms = start.elapsed().as_millis();
            if ms < min_ms {
                min_ms = ms;
                best = region;
            }
        }
    }
    best
}

async fn measure_ping(client: &reqwest::Client, url: &str) -> Result<u128, AppError> {
    let start = Instant::now();
    client.get(url).send().await?;
    Ok(start.elapsed().as_millis())
}

async fn measure_download(client: &reqwest::Client, url: &str) -> Result<f64, AppError> {
    let start = Instant::now();
    let response = client.get(url).send().await?;
    let bytes = response.bytes().await?;
    let duration_secs = start.elapsed().as_secs_f64();
    Ok((bytes.len() as f64 * 8.0) / (duration_secs * 1_000_000.0))
}

async fn measure_upload(client: &reqwest::Client, url: &str) -> Result<f64, AppError> {
    let mut data = vec![0u8; UPLOAD_SIZE_BYTES];
    rand::thread_rng().fill_bytes(&mut data);

    let start = Instant::now();
    client.post(url).body(data).send().await?;
    let duration_secs = start.elapsed().as_secs_f64();

    Ok((UPLOAD_SIZE_BYTES as f64 * 8.0) / (duration_secs * 1_000_000.0))
}

fn format_ping(ms: u128) -> String {
    let color = if ms < 50 { Color::Green } else if ms < 150 { Color::Yellow } else { Color::Red };
    format!("{} {}", format!("{} ms", ms).color(color).bold(), "OK".green())
}

fn format_speed(mbps: f64) -> String {
    let color = if mbps > 50.0 { Color::Green } else if mbps > 10.0 { Color::Yellow } else { Color::Red };
    format!("{} {}", format!("{:.2} Mbps", mbps).color(color).bold(), "OK".green())
}
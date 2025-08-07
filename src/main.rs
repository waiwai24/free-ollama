use clap::{Arg, Command};
use env_logger;
use log::error;
use std::process;

use free_ollama::{
    utils::CsvParser,
    scanner::SimpleScanner,
};

#[tokio::main]
async fn main() {
    env_logger::init();
    
    let app = Command::new("free-ollama")
        .version("0.1.0")
        .about("A simple tool for discovering Ollama services")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("Path to Ollama assets CSV file (country,link format)")
                .required(true)
        )
        .arg(
            Arg::new("timeout")
                .short('t')
                .long("timeout")
                .value_name("SECS")
                .help("Request timeout in seconds")
                .default_value("3")
        );

    let matches = app.get_matches();
    
    let input_file = matches.get_one::<String>("input").unwrap();
    let timeout: u64 = matches.get_one::<String>("timeout")
        .unwrap()
        .parse()
        .unwrap_or(1);
        
    let targets = match CsvParser::parse_from_file(input_file) {
        Ok(targets) => targets,
        Err(e) => {
            error!("Failed to parse CSV file: {}", e);
            process::exit(1);
        }
    };
    
    let services = match SimpleScanner::scan_services(targets, timeout).await {
        Ok(services) => services,
        Err(e) => {
            error!("Failed to discover services: {}", e);
            process::exit(1);
        }
    };
    
    let active_services_count = services.iter().filter(|s| s.is_active).count();
    
    println!("Scan completed");
    println!("Found {} active services out of {} total services", active_services_count, services.len());
}

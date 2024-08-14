use clap::Parser;
use colored::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json;
use thiserror::Error;
use tracing::{debug, error};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    username: String,

    #[arg(short, long, default_value = "All CNCF")]
    project: String,

    #[arg(short, long, default_value = "Last quarter")]
    range: String,

    #[arg(short, long, default_value = "Contributions")]
    metric: String,

    #[arg(short, long)]
    verbose: bool,
}

#[derive(Serialize, Debug)]
struct DevActCntCompPayload {
    api: String,
    payload: PayloadDetails,
}

#[derive(Serialize, Debug)]
struct PayloadDetails {
    project: String,
    range: String,
    metric: String,
    repository_group: String,
    country: String,
    companies: Vec<String>,
    github_id: String,
}

#[derive(Deserialize, Debug)]
struct DevActCntCompResponse {
    project: String,
    range: String,
    metric: String,
    rank: Vec<i32>,
    login: Vec<String>,
    company: Vec<String>,
    number: Vec<i32>,
}

#[derive(Deserialize, Debug)]
struct ErrorResponse {
    error: String,
}

#[derive(Error, Debug)]
enum DevStatsError {
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),
    #[error("Failed to parse response: {0}")]
    ParseFailed(#[from] serde_json::Error),
    #[error("API error: {0}")]
    ApiError(String),
}

async fn get_devstats_contributions(
    client: &Client,
    username: &str,
    project: &str,
    range: &str,
    metric: &str,
) -> Result<DevActCntCompResponse, DevStatsError> {
    let url = "https://devstats.cncf.io/api/v1";
    let request_body = DevActCntCompPayload {
        api: "DevActCntComp".to_string(),
        payload: PayloadDetails {
            project: project.to_string(),
            range: range.to_string(),
            metric: metric.to_string(),
            repository_group: "All".to_string(),
            country: "All".to_string(),
            companies: vec!["All".to_string()],
            github_id: username.to_string(),
        },
    };

    debug!("Sending request to: {}", url);
    debug!("Request body: {:?}", request_body);

    let response = client.post(url).json(&request_body).send().await?;

    let status = response.status();
    debug!("Response status: {}", status);

    let response_text = response.text().await?;

    debug!("Raw response: {}", response_text);

    if status.is_success() {
        let parsed_response: DevActCntCompResponse = serde_json::from_str(&response_text)?;
        Ok(parsed_response)
    } else {
        let error_response: ErrorResponse = serde_json::from_str(&response_text)?;
        Err(DevStatsError::ApiError(error_response.error))
    }
}

fn print_user_contributions(username: &str, response: &DevActCntCompResponse) {
    println!("{}", "DevStats Contributions".bold().green());
    println!("User: {}", username.yellow());
    println!("Project: {}", response.project.blue());
    println!("Period: {}", response.range.blue());
    println!("Metric: {}", response.metric.blue());
    println!();

    println!(
        "{:<10} {:<20} {:<30} {}",
        "Rank".bold(),
        "Login".bold(),
        "Company".bold(),
        "Contributions".bold()
    );

    let total_contributions: i32 = response
        .rank
        .iter()
        .zip(&response.login)
        .zip(&response.company)
        .zip(&response.number)
        .filter(|&(((_, login), _), _)| login == username)
        .map(|(((rank, login), company), number)| {
            println!(
                "{:<10} {:<20} {:<30} {}",
                rank.to_string().cyan(),
                login.yellow(),
                company,
                number.to_string().magenta()
            );
            *number
        })
        .sum();

    if total_contributions == 0 {
        println!("No contributions found for the user in the given period.");
    }

    println!(
        "\nTotal contributions: {}",
        total_contributions.to_string().bold().magenta()
    );
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.verbose {
        tracing_subscriber::fmt::init();
    }

    let client = Client::new();

    match get_devstats_contributions(
        &client,
        &args.username,
        &args.project,
        &args.range,
        &args.metric,
    )
    .await
    {
        Ok(contributions) => print_user_contributions(&args.username, &contributions),
        Err(e) => error!("Error: {}", e),
    }

    Ok(())
}

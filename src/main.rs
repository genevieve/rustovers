use clap::Parser;
use aws_config;
use aws_sdk_ec2::{Client, Region, Error};

#[derive(Parser)]
struct Cli {
    iaas: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Cli::parse();

    if args.iaas == "aws" {
        let region_provider = Region::new("us-west-2");
        let shared_config = aws_config::from_env().region(region_provider).load().await;
        let client = Client::new(&shared_config);
        show_regions(&client).await;
    }
    Ok(())
}

async fn show_regions(client: &Client) -> Result<(), Error> {
    let rsp = client.describe_regions().send().await?;

    println!("Regions:");
    for region in rsp.regions().unwrap_or_default() {
        println!("  {}", region.region_name().unwrap());
    }

    Ok(())
}

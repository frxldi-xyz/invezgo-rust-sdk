use std::time::Duration;
use invezgo_sdk::InvezgoClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize client using the builder
    // You would replace "dummy-token" with a valid JWT from Invezgo
    let client = InvezgoClient::builder()
        .api_key("dummy-token")
        .base_url("https://api.invezgo.com") // optional, defaults to api.invezgo.com
        .timeout(Duration::from_secs(10))
        .build()?;

    println!("Invezgo SDK Client Initialized successfully!");

    // 2. Fetch companies from the Analysis sub-client
    // We wrap in a block because calling this without a valid token will return an authentication error
    println!("\nTrying to fetch stock list...");
    match client.analysis().get_stock_list().await {
        Ok(stocks) => {
            println!("Successfully fetched {} stocks!", stocks.len());
            for stock in stocks.iter().take(3) {
                println!("- {} ({}): Sector: {}", stock.name, stock.code, stock.sector);
            }
        }
        Err(e) => {
            println!("Could not fetch stock list (expected if using dummy-token): {}", e);
        }
    }

    // 3. Check API Usage
    println!("\nChecking API usage...");
    match client.usage().usage_api().await {
        Ok(_usage) => {
            println!("API Usage details fetched successfully!");
            // print usage information
        }
        Err(e) => {
            println!("Could not fetch API usage: {}", e);
        }
    }

    Ok(())
}

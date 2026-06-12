# Invezgo Rust SDK

Official fully-typed Rust SDK for the Invezgo API. This SDK is generated 1:1 based on the latest Invezgo OpenAPI specification.

---

## Features
- **1:1 Endpoint Coverage**: Access all 105 endpoints defined in the spec.
- **Fully Typed**: Rich Rust structures for request bodies, path parameters, queries, and response payloads.
- **Robust Error Handling**: Standardized `InvezgoError` wrapping HTTP, deserialization, and API-specific errors.
- **Modern Async Rust**: Built on `reqwest`, `serde`, and `tokio`.

---

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
invezgo-sdk = { path = "./path-to-sdk" } # or git url
tokio = { version = "1", features = ["full"] }
```

---

## Quickstart

Here is a quick example of how to initialize the client and fetch the stock list:

```rust
use std::time::Duration;
use invezgo_sdk::InvezgoClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize client using the builder
    let client = InvezgoClient::builder()
        .api_key("YOUR_JWT_API_KEY")
        .base_url("https://api.invezgo.com") // defaults to api.invezgo.com
        .timeout(Duration::from_secs(30))
        .build()?;

    // 2. Fetch stock list from the analysis API
    println!("Fetching companies...");
    match client.analysis().get_stock_list().await {
        Ok(stocks) => {
            println!("Fetched {} stocks successfully!", stocks.len());
            for stock in stocks.iter().take(5) {
                println!("- {} ({}): Sector: {}", stock.name, stock.code, stock.sector);
            }
        }
        Err(e) => eprintln!("Error fetching stock list: {}", e),
    }

    Ok(())
}
```

---

## SDK Architecture

The client API is structured into modular namespaces mapping directly to the underlying API controllers:

- **`client.analysis()`**: Stock listings, index directories, shareholder reports, historical charts, order books, and real-time trackers.
- **`client.alerts()`**: Set up, test, list, and delete real-time stock alerts.
- **`client.journals()`**: Record, extract, summarize, update, and manage journal entries.
- **`client.watchlists()`**: Custom groups and tracked stocks.
- **`client.trades()`**: Realized trade summaries and tracking.
- **`client.portfolios()`**: Asset summary and portfolios.
- **`client.posts()`**: Community posts, comments, likes, and voting.
- **`client.recommendation()`**: Target recommendations.
- **`client.screener()`**: Preset screeners and active screening engines.
- **`client.search()`**: Full-text search for stocks and users.
- **`client.membership()`**: Subscription levels and transactions.
- **`client.usage()`**: Tracking system usage and quotas.

---

## Error Handling

All client methods return a `Result<T, InvezgoError>`. You can handle error responses as follows:

```rust
use invezgo_sdk::InvezgoError;

match client.analysis().get_stock_list().await {
    Ok(stocks) => { /* ... */ },
    Err(InvezgoError::ApiError { status_code, message, error }) => {
        println!("API error {}: {} ({:?})", status_code, message, error);
    }
    Err(e) => {
        println!("Network or parse error: {}", e);
    }
}
```

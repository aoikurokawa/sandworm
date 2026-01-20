# Arrakis

A Rust client library for the [Dune Analytics API](https://dune.com/docs/api/).

[![Crates.io](https://img.shields.io/crates/v/arrakis.svg)](https://crates.io/crates/arrakis)
[![Documentation](https://docs.rs/arrakis/badge.svg)](https://docs.rs/arrakis)
[![License](https://img.shields.io/crates/l/arrakis.svg)](https://github.com/aoikurokawa/arrakis#license)

## Features

- **Async & Blocking Clients** - Choose between async (tokio) or synchronous APIs
- **Execute SQL Queries** - Run arbitrary SQL queries against Dune's data warehouse
- **Execute Saved Queries** - Run pre-saved queries by ID with optional parameters
- **Pipeline Execution** - Execute query pipelines with dependencies
- **Results Management** - Fetch results in JSON or CSV format with pagination support
- **Execution Control** - Cancel ongoing executions and track execution state
- **Convenience Methods** - High-level methods like `run_sql()` that handle polling automatically

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
arrakis = "0.5"
```

This enables both the async and blocking clients; no extra feature flags are required.

## Quick Start

### Async Client

```rust
use arrakis::DuneClient;

#[tokio::main]
async fn main() -> Result<(), arrakis::DuneError> {
    let client = DuneClient::new("YOUR_API_KEY");

    // Execute SQL and wait for results (with 60 second timeout)
    let results = client
        .run_sql("SELECT * FROM ethereum.transactions LIMIT 10", 60)
        .await?;

    println!("{:?}", results);
    Ok(())
}
```

### Blocking Client

```rust
use arrakis::blocking::DuneClient;

fn main() -> Result<(), arrakis::DuneError> {
    let client = DuneClient::new("YOUR_API_KEY");

    // Execute SQL and wait for results (with 60 second timeout)
    let results = client.run_sql("SELECT * FROM ethereum.transactions LIMIT 10", 60)?;

    println!("{:?}", results);
    Ok(())
}
```

## Usage

### Execute SQL Query

```rust
use arrakis::DuneClient;

let client = DuneClient::new("YOUR_API_KEY");

// Simple execution - returns immediately with execution_id
let response = client.execute_sql("SELECT 1").await?;
println!("Execution ID: {}", response.execution_id);

// Wait for results with timeout
let results = client.wait_for_results(&response.execution_id, 120).await?;
```

### Execute Saved Query

```rust
use arrakis::{DuneClient, QueryParameter};

let client = DuneClient::new("YOUR_API_KEY");

// Execute a saved query with parameters
let params = vec![
    QueryParameter {
        key: "address".to_string(),
        value: "0x...".to_string(),
        parameter_type: "text".to_string(),
    },
];

let response = client.execute_query(12345, Some(params), None).await?;
let results = client.run_query(12345, None, None, 60).await?;
```

### Get Results in CSV Format

```rust
let csv_data = client.get_execution_results_csv(&execution_id, None).await?;
```

### Cancel Execution

```rust
client.cancel_execution(&execution_id).await?;
```

### Check Execution Status

```rust
let status = client.get_execution_status(&execution_id).await?;
match status.state {
    arrakis::ExecutionState::Completed => println!("Done!"),
    arrakis::ExecutionState::Executing => println!("Still running..."),
    arrakis::ExecutionState::Failed => println!("Failed!"),
    _ => {}
}
```

## API Coverage

| Endpoint | Method |
|----------|--------|
| Execute SQL | `execute_sql()` |
| Execute Saved Query | `execute_query()` |
| Execute Pipeline | `execute_pipeline()` |
| Get Execution Status | `get_execution_status()` |
| Get Execution Results (JSON) | `get_execution_results()` |
| Get Execution Results (CSV) | `get_execution_results_csv()` |
| Get Latest Query Results | `get_latest_results()` |
| Get Latest Query Results (CSV) | `get_latest_results_csv()` |
| Cancel Execution | `cancel_execution()` |

## Examples

See the [examples](examples/) directory for more usage patterns:

```bash
# Run the async example
DUNE_API_KEY=your_key cargo run --example run_sql

# Run the blocking example
DUNE_API_KEY=your_key cargo run --example sync_run_sql
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.

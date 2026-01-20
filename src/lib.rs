//! # Sandworm - Dune Analytics API Client
//!
//! A Rust client library for the [Dune Analytics API](https://dune.com/docs/api/).
//!
//! ## Features
//!
//! - Execute arbitrary SQL queries
//! - Execute saved queries by ID
//! - Execute query pipelines
//! - Get execution status and results (JSON and CSV)
//! - Cancel ongoing executions
//! - Convenience methods for running queries and waiting for results
//!
//! ## Quick Start
//!
//! ```no_run
//! use std::time::Duration;
//!
//! use arrakis::{DuneClient, Result};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     // Create a client with your API key
//!     let client = DuneClient::new("your-api-key")?;
//!
//!     // Execute a SQL query and wait for results
//!     let results = client.run_sql(
//!         "SELECT * FROM ethereum.transactions LIMIT 10",
//!         Duration::from_secs(60),
//!     ).await?;
//!
//!     // Process the results
//!     if let Some(result) = results.result {
//!         for row in result.rows {
//!             println!("{:?}", row);
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Manual Execution Flow
//!
//! For more control, you can manually manage the execution lifecycle:
//!
//! ```no_run
//! use arrakis::{DuneClient, ExecutionState};
//!
//! # async fn example() -> arrakis::Result<()> {
//! let client = DuneClient::new("your-api-key")?;
//!
//! // Start execution
//! let exec = client.execute_query(1234567).await?;
//! println!("Execution ID: {}", exec.execution_id);
//!
//! // Poll for status
//! loop {
//!     let status = client.get_execution_status(&exec.execution_id).await?;
//!
//!     match status.state {
//!         ExecutionState::Completed => {
//!             let results = client.get_execution_results(&exec.execution_id).await?;
//!             println!("Got results!");
//!             break;
//!         }
//!         ExecutionState::Failed => {
//!             println!("Query failed!");
//!             break;
//!         }
//!         _ => {
//!             tokio::time::sleep(std::time::Duration::from_secs(1)).await;
//!         }
//!     }
//! }
//! # Ok(())
//! # }
//! ```

mod client;
mod error;
mod types;

pub use client::DuneClient;
pub use error::{DuneError, Result};
pub use types::*;

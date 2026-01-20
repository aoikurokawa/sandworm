use std::time::Duration;

use reqwest::{Client, header};

use crate::{
    error::{DuneError, Result},
    types::*,
};

const BASE_URL: &str = "https://api.dune.com/api";
const DEFAULT_TIMEOUT_SECS: u64 = 30;

/// Client for interacting with the Dune Analytics API.
#[derive(Debug, Clone)]
pub struct DuneClient {
    client: Client,
    base_url: String,
}

impl DuneClient {
    /// Creates a new Dune client with the given API key.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your Dune Analytics API key.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use arrakis::DuneClient;
    ///
    /// let client = DuneClient::new("your-api-key").unwrap();
    /// ```
    pub fn new(api_key: impl Into<String>) -> Result<Self> {
        Self::with_base_url(api_key, BASE_URL)
    }

    /// Creates a new Dune client with a custom base URL.
    pub fn with_base_url(api_key: impl Into<String>, base_url: impl Into<String>) -> Result<Self> {
        let api_key = api_key.into();
        if api_key.is_empty() {
            return Err(DuneError::InvalidApiKey);
        }

        let mut headers = header::HeaderMap::new();
        headers.insert(
            "X-Dune-Api-Key",
            header::HeaderValue::from_str(&api_key).map_err(|_| DuneError::InvalidApiKey)?,
        );

        let client = Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECS))
            .build()?;

        Ok(Self {
            client,
            base_url: base_url.into(),
        })
    }

    // ==================== Execute Endpoints ====================

    /// Executes an arbitrary SQL query.
    ///
    /// # Arguments
    ///
    /// * `sql` - The SQL query to execute.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example() -> arrakis::Result<()> {
    /// use arrakis::DuneClient;
    ///
    /// let client = DuneClient::new("your-api-key")?;
    /// let response = client.execute_sql("SELECT * FROM ethereum.transactions LIMIT 10").await?;
    /// println!("Execution ID: {}", response.execution_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn execute_sql(&self, sql: impl Into<String>) -> Result<ExecuteResponse> {
        let request = ExecuteSqlRequest {
            sql: sql.into(),
            ..Default::default()
        };
        self.execute_sql_with_options(request).await
    }

    /// Executes an arbitrary SQL query with additional options.
    pub async fn execute_sql_with_options(
        &self,
        request: ExecuteSqlRequest,
    ) -> Result<ExecuteResponse> {
        let url = format!("{}/v1/sql/execute", self.base_url);

        let response = self.client.post(&url).json(&request).send().await?;

        self.handle_response(response).await
    }

    /// Executes a saved query by its ID.
    ///
    /// # Arguments
    ///
    /// * `query_id` - The ID of the saved query to execute.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example() -> arrakis::Result<()> {
    /// use arrakis::DuneClient;
    ///
    /// let client = DuneClient::new("your-api-key")?;
    /// let response = client.execute_query(1234567).await?;
    /// println!("Execution ID: {}", response.execution_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn execute_query(&self, query_id: u64) -> Result<ExecuteResponse> {
        self.execute_query_with_options(query_id, ExecuteQueryRequest::default())
            .await
    }

    /// Executes a saved query with additional options.
    pub async fn execute_query_with_options(
        &self,
        query_id: u64,
        request: ExecuteQueryRequest,
    ) -> Result<ExecuteResponse> {
        let url = format!("{}/v1/query/{}/execute", self.base_url, query_id);

        let response = self.client.post(&url).json(&request).send().await?;

        self.handle_response(response).await
    }

    /// Executes a query pipeline with all its dependencies.
    ///
    /// # Arguments
    ///
    /// * `query_id` - The ID of the root query in the pipeline.
    pub async fn execute_pipeline(&self, query_id: u64) -> Result<PipelineExecuteResponse> {
        self.execute_pipeline_with_options(query_id, ExecuteQueryRequest::default())
            .await
    }

    /// Executes a query pipeline with additional options.
    pub async fn execute_pipeline_with_options(
        &self,
        query_id: u64,
        request: ExecuteQueryRequest,
    ) -> Result<PipelineExecuteResponse> {
        let url = format!("{}/v1/query/{}/pipeline/execute", self.base_url, query_id);

        let response = self.client.post(&url).json(&request).send().await?;

        self.handle_response(response).await
    }

    // ==================== Status & Results Endpoints ====================

    /// Gets the status of a query execution.
    ///
    /// # Arguments
    ///
    /// * `execution_id` - The execution ID returned from an execute call.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example() -> arrakis::Result<()> {
    /// use arrakis::DuneClient;
    ///
    /// let client = DuneClient::new("your-api-key")?;
    /// let status = client.get_execution_status("01234567-89ab-cdef-0123-456789abcdef").await?;
    /// println!("State: {:?}", status.state);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_execution_status(
        &self,
        execution_id: &str,
    ) -> Result<ExecutionStatusResponse> {
        let url = format!("{}/v1/execution/{}/status", self.base_url, execution_id);

        let response = self.client.get(&url).send().await?;

        self.handle_response(response).await
    }

    /// Gets the results of a query execution in JSON format.
    ///
    /// # Arguments
    ///
    /// * `execution_id` - The execution ID returned from an execute call.
    pub async fn get_execution_results(
        &self,
        execution_id: &str,
    ) -> Result<ExecutionResultsResponse> {
        self.get_execution_results_with_options(execution_id, ResultOptions::default())
            .await
    }

    /// Gets the results of a query execution with additional options.
    pub async fn get_execution_results_with_options(
        &self,
        execution_id: &str,
        options: ResultOptions,
    ) -> Result<ExecutionResultsResponse> {
        let url = format!("{}/v1/execution/{}/results", self.base_url, execution_id);

        let response = self
            .client
            .get(&url)
            .query(&options.to_query_params())
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Gets the results of a query execution in CSV format.
    ///
    /// # Arguments
    ///
    /// * `execution_id` - The execution ID returned from an execute call.
    pub async fn get_execution_results_csv(&self, execution_id: &str) -> Result<String> {
        self.get_execution_results_csv_with_options(execution_id, ResultOptions::default())
            .await
    }

    /// Gets the results of a query execution in CSV format with options.
    pub async fn get_execution_results_csv_with_options(
        &self,
        execution_id: &str,
        options: ResultOptions,
    ) -> Result<String> {
        let url = format!(
            "{}/v1/execution/{}/results/csv",
            self.base_url, execution_id
        );

        let response = self
            .client
            .get(&url)
            .query(&options.to_query_params())
            .send()
            .await?;

        self.handle_text_response(response).await
    }

    /// Gets the latest results of a saved query in JSON format.
    ///
    /// # Arguments
    ///
    /// * `query_id` - The ID of the saved query.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example() -> arrakis::Result<()> {
    /// use arrakis::DuneClient;
    ///
    /// let client = DuneClient::new("your-api-key")?;
    /// let results = client.get_latest_results(1234567).await?;
    /// if let Some(result) = results.result {
    ///     println!("Got {} rows", result.rows.len());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_latest_results(&self, query_id: u64) -> Result<ExecutionResultsResponse> {
        self.get_latest_results_with_options(query_id, ResultOptions::default())
            .await
    }

    /// Gets the latest results of a saved query with additional options.
    pub async fn get_latest_results_with_options(
        &self,
        query_id: u64,
        options: ResultOptions,
    ) -> Result<ExecutionResultsResponse> {
        let url = format!("{}/v1/query/{}/results", self.base_url, query_id);

        let response = self
            .client
            .get(&url)
            .query(&options.to_query_params())
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Gets the latest results of a saved query in CSV format.
    ///
    /// # Arguments
    ///
    /// * `query_id` - The ID of the saved query.
    pub async fn get_latest_results_csv(&self, query_id: u64) -> Result<String> {
        self.get_latest_results_csv_with_options(query_id, ResultOptions::default())
            .await
    }

    /// Gets the latest results of a saved query in CSV format with options.
    pub async fn get_latest_results_csv_with_options(
        &self,
        query_id: u64,
        options: ResultOptions,
    ) -> Result<String> {
        let url = format!("{}/v1/query/{}/results/csv", self.base_url, query_id);

        let response = self
            .client
            .get(&url)
            .query(&options.to_query_params())
            .send()
            .await?;

        self.handle_text_response(response).await
    }

    // ==================== Cancel Endpoint ====================

    /// Cancels an ongoing query execution.
    ///
    /// # Arguments
    ///
    /// * `execution_id` - The execution ID to cancel.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example() -> arrakis::Result<()> {
    /// use arrakis::DuneClient;
    ///
    /// let client = DuneClient::new("your-api-key")?;
    /// let response = client.cancel_execution("01234567-89ab-cdef-0123-456789abcdef").await?;
    /// if response.success {
    ///     println!("Execution cancelled successfully");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn cancel_execution(&self, execution_id: &str) -> Result<CancelExecutionResponse> {
        let url = format!("{}/v1/execution/{}/cancel", self.base_url, execution_id);

        let response = self.client.post(&url).send().await?;

        self.handle_response(response).await
    }

    // ==================== Convenience Methods ====================

    /// Executes a SQL query and waits for the results.
    ///
    /// This is a convenience method that combines `execute_sql`, polling for status,
    /// and fetching results.
    ///
    /// # Arguments
    ///
    /// * `sql` - The SQL query to execute.
    /// * `timeout` - Maximum time to wait for results.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example() -> arrakis::Result<()> {
    /// use std::time::Duration;
    ///
    /// use arrakis::DuneClient;
    ///
    /// let client = DuneClient::new("your-api-key")?;
    /// let results = client.run_sql(
    ///     "SELECT * FROM ethereum.transactions LIMIT 10",
    ///     Duration::from_secs(60),
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn run_sql(
        &self,
        sql: impl Into<String>,
        timeout: Duration,
    ) -> Result<ExecutionResultsResponse> {
        let execute_response = self.execute_sql(sql).await?;
        self.wait_for_results(&execute_response.execution_id, timeout)
            .await
    }

    /// Executes a saved query and waits for the results.
    pub async fn run_query(
        &self,
        query_id: u64,
        timeout: Duration,
    ) -> Result<ExecutionResultsResponse> {
        let execute_response = self.execute_query(query_id).await?;
        self.wait_for_results(&execute_response.execution_id, timeout)
            .await
    }

    /// Waits for a query execution to complete and returns the results.
    ///
    /// # Arguments
    ///
    /// * `execution_id` - The execution ID to wait for.
    /// * `timeout` - Maximum time to wait.
    pub async fn wait_for_results(
        &self,
        execution_id: &str,
        timeout: Duration,
    ) -> Result<ExecutionResultsResponse> {
        let start = std::time::Instant::now();
        let poll_interval = Duration::from_secs(1);

        loop {
            if start.elapsed() > timeout {
                return Err(DuneError::Timeout {
                    seconds: timeout.as_secs(),
                });
            }

            let status = self.get_execution_status(execution_id).await?;

            match status.state {
                ExecutionState::Completed => {
                    return self.get_execution_results(execution_id).await;
                }
                ExecutionState::Failed => {
                    return Err(DuneError::ExecutionFailed {
                        message: format!(
                            "Query execution failed for execution_id {}: {:?}",
                            execution_id, status
                        ),
                    });
                }
                ExecutionState::Cancelled => {
                    return Err(DuneError::Cancelled);
                }
                ExecutionState::Pending | ExecutionState::Executing => {
                    tokio::time::sleep(poll_interval).await;
                }
            }
        }
    }

    // ==================== Internal Helpers ====================

    async fn handle_response<T: serde::de::DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> Result<T> {
        let status = response.status();

        if status.is_success() {
            let body = response.text().await?;
            serde_json::from_str(&body).map_err(DuneError::from)
        } else {
            let body = response.text().await.unwrap_or_default();

            // Try to parse error message from response
            if let Ok(error_response) = serde_json::from_str::<serde_json::Value>(&body) {
                if let Some(message) = error_response.get("error").and_then(|e| e.as_str()) {
                    return Err(DuneError::Api {
                        message: message.to_string(),
                    });
                }
            }

            Err(DuneError::Api {
                message: format!("HTTP {}: {}", status, body),
            })
        }
    }

    async fn handle_text_response(&self, response: reqwest::Response) -> Result<String> {
        let status = response.status();

        if status.is_success() {
            Ok(response.text().await?)
        } else {
            let body = response.text().await.unwrap_or_default();
            Err(DuneError::Api {
                message: format!("HTTP {}: {}", status, body),
            })
        }
    }
}

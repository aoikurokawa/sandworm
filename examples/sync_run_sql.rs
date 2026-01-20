use arrakis::{Result, blocking::DuneClient};
use std::time::Duration;

fn main() -> Result<()> {
    let api_key =
        std::env::var("DUNE_API_KEY").expect("DUNE_API_KEY environment variable must be set");
    let client = DuneClient::new(api_key)?;

    let results = client.run_sql(
        "SELECT
                block_slot / 432000 AS epoch,
                SUM(balance_change) AS net_sol
            FROM solana.account_activity
            WHERE address = 'GSyXx6WRm2o6Qu4RWxTH17swLZKpTKQdQTS2uGcus1NF'
              AND tx_success = true
              AND balance_change > 0
            GROUP BY 1
            ORDER BY 1 ASC;",
        Duration::from_secs(60),
    )?;

    if let Some(result) = results.result {
        for row in result.rows {
            println!("{:?}", row);
        }
    }
    Ok(())
}

use std::error::Error;
pub mod models;
pub mod mempool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    mempool::get_recommended_fees().await?;
    Ok(())
}

mod models;

use std::error::Error;

use models::{Mempool, RecommendedFees};

pub async fn get_mempool() -> Result<Mempool, Box<dyn Error>> {
    let http_response = reqwest::get("https://mempool.space/api/mempool").await?;
    let response = http_response.json::<models::Mempool>().await?;

    Ok(response)
}

pub async fn get_recommended_fees() -> Result<RecommendedFees, Box<dyn Error>> {
    let http_response = reqwest::get("https://mempool.space/api/v1/fees/recommended").await?;
    let response = http_response.json::<models::RecommendedFees>().await?;
    
    Ok(response)
}
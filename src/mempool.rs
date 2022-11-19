
use crate::models;
use std::error::Error;

pub async fn get_mempool() -> Result<(), Box<dyn Error>> {
    let http_response = reqwest::get("https://mempool.space/api/mempool").await?;
    let response = http_response.json::<models::Mempool>().await?;
    println!("{:#?}", response);
    Ok(())
}

pub async fn get_recommended_fees() -> Result<(), Box<dyn Error>> {
    let http_response = reqwest::get("https://mempool.space/api/v1/fees/recommended").await?;
    let response = http_response.json::<models::RecommendedFees>().await?;
    println!("{:#?}", response);
    Ok(())
}
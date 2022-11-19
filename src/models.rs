use serde::{Serialize, Deserialize};

#[derive(Deserialize, Debug)]
pub struct Response {
    pub coins: Vec<Coin>,
}

/*
 * The #[serde(rename = "name")] attribute is used to rename the field name
 * to match the JSON field name.
 */
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Coin {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub symbol: String,
    pub price: f32,
    pub price_btc: f64,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct Mempool {
    pub count: u32,
    pub vsize: u32,
    pub total_fee: u32,
    pub fee_histogram: Vec<FeeHistogram>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FeeHistogram {
    pub a: f32,
    pub b: u32,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecommendedFees {
    pub fastest_fee: u32,
    pub half_hour_fee: u32,
    pub hour_fee: u32,
    pub economy_fee: u32,
    pub minimum_fee: u32,
}
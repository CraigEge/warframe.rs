use serde::Deserialize;

use super::{
    Category,
    Component,
};

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Sentinel {
    pub armor: i64,

    pub build_price: i64,

    pub build_quantity: i64,

    pub build_time: i64,

    pub category: Category,

    pub components: Vec<Component>,

    pub consume_on_build: bool,

    pub description: String,

    pub health: i64,

    pub image_name: String,

    pub is_prime: bool,

    pub masterable: bool,

    pub name: String,

    pub power: i64,

    pub product_category: String,

    pub shield: i64,

    pub skip_build_time_price: i64,

    pub stamina: f64,

    pub tradable: bool,

    #[serde(rename = "type")]
    pub sentinel_type: String,

    pub unique_name: String,
}

#[tokio::test]
async fn test_sentinel_query() -> Result<(), Box<dyn std::error::Error>> {
    let _sentinel = reqwest::get("https://api.warframestat.us/items/nautilus%20prime/")
        .await?
        .json::<Sentinel>()
        .await?;
    Ok(())
}

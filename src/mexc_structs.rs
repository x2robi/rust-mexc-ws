use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MexcSubscribeMessage {
    pub method: String,
    pub params: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct MexcSubscribeResponseMessage {
    pub id: i64,
    pub code: i64,
    pub msg: String,
}

#[derive(Serialize, Deserialize)]
pub struct MexcKlineData {
    pub t: i64,
    pub o: String,
    pub c: String,
    pub h: String,
    pub l: String,
    pub v: String,
    pub a: String,
    #[serde(rename = "T")]
    pub t_0: i64,
    pub i: String,
}

#[derive(Serialize, Deserialize)]
pub struct MexcKlineDataRoot {
    pub e: String,
    pub k: MexcKlineData,
}
#[derive(Serialize, Deserialize)]
pub struct MexcKlineMessage {
    pub d: MexcKlineDataRoot,
    pub c: String,
    pub t: i64,
    pub s: String,
}

#[derive(Serialize, Deserialize)]
pub struct MexcExchangeInfoElem {
    pub symbol: String,
    pub status: String,
    #[serde(rename = "baseAsset")]
    pub base_asset: String,
    #[serde(rename = "baseAssetPrecision")]
    pub base_asset_precision: i64,
    #[serde(rename = "quoteAsset")]
    pub quote_asset: String,
    #[serde(rename = "quotePrecision")]
    pub quote_precision: i64,
    #[serde(rename = "quoteAssetPrecision")]
    pub quote_asset_precision: i64,
    #[serde(rename = "baseCommissionPrecision")]
    pub base_commission_precision: i64,
    #[serde(rename = "quoteCommissionPrecision")]
    pub quote_commission_precision: i64,
    #[serde(rename = "orderTypes")]
    pub order_types: Vec<String>,
    #[serde(rename = "isSpotTradingAllowed")]
    pub is_spot_trading_allowed: bool,
    #[serde(rename = "isMarginTradingAllowed")]
    pub is_margin_trading_allowed: bool,
    #[serde(rename = "quoteAmountPrecision")]
    pub quote_amount_precision: String,
    #[serde(rename = "baseSizePrecision")]
    pub base_size_precision: String,
    pub permissions: Vec<String>,
    #[serde(rename = "maxQuoteAmount")]
    pub max_quote_amount: String,
    #[serde(rename = "makerCommission")]
    pub maker_commission: String,
    #[serde(rename = "takerCommission")]
    pub taker_commission: String,
    #[serde(rename = "quoteAmountPrecisionMarket")]
    pub quote_amount_precision_market: String,
    #[serde(rename = "maxQuoteAmountMarket")]
    pub max_quote_amount_market: String,
    #[serde(rename = "fullName")]
    pub full_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct MexcExchangeInfo {
    pub timezone: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    pub symbols: Vec<MexcExchangeInfoElem>,
}

pub type MexcKlineResponse = Vec<(u64, String, String, String, String, String, u64, String)>;
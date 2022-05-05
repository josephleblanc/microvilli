use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PoolDetails {
    #[serde(alias = "accruedNumBlocksRewards")]
    pub accrued_num_blocks_rewards: Option<u32>,
    pub arb: Option<f64>,
    #[serde(alias = "blocksPerYear")]
    pub blocks_per_year: Option<u32>,
    #[serde(alias = "dailySwapFees")]
    pub daily_swap_fees: Option<u32>,
    #[serde(alias = "poolApr")]
    pub pool_apr: Option<f64>,
    #[serde(alias = "poolBalance")]
    pub pool_balance: Option<f64>,
    #[serde(alias = "poolBalanceInRowan")]
    pub pool_balance_in_rowan: Option<f64>,
    #[serde(alias = "poolDepth")]
    pub pool_depth: Option<f64>,
    #[serde(alias = "poolTVL")]
    pub pool_tvl: Option<f64>,
    #[serde(alias = "priceToken")]
    pub price_token: Option<f64>,
    #[serde(alias = "rewardApr")]
    pub reward_apr: Option<f64>,
    #[serde(alias = "rewardPeriodNativeDistributed")]
    pub reward_period_native_distributed: Option<f64>,
    pub symbol: String,
    pub volume: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PoolRatio {
    #[serde(alias = "Pool")]
    pub pool: Pool,
    #[serde(alias = "clpModuleAddress")]
    pub clp_module_address: String,
    pub height: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pool {
    #[serde(alias = "externalAsset")]
    pub external_asset: ExternalAsset,
    #[serde(alias = "nativeAsset")]
    pub native_asset: NativeAsset,
    #[serde(alias = "poolUnits")]
    pub pool_units: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalAsset {
    pub symbol: String,
    pub balance: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NativeAsset {
    pub symbol: String,
    pub balance: String,
}

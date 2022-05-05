use futures::future::try_join_all;
use serde::{Deserialize, Serialize};

pub async fn sif_prices() -> Vec<PoolRatio> {
    let symbol_list = [
        "atom", "eth", "luna", "cro", "juno", "xprt", "akt", "regen", "dvpn", "iris", "iov", "ngm",
        "usdt", "eeur",
    ];

    let mut request_vec = vec![];
    for symbol in symbol_list {
        let dest: String = format!("https://data.sifchain.finance/beta/pool/{}", symbol);
        let response = reqwest::get(dest);

        request_vec.push(response);
    }
    let responses = try_join_all(request_vec).await.unwrap();

    let mut pool_ratios = vec![];
    for response in responses {
        let json_response = response.json().await.ok();
        let pool_ratio = match json_response {
            Some(json) => serde_json::from_value::<PoolRatio>(json).ok(),
            None => None,
        };
        match pool_ratio {
            Some(p) => pool_ratios.push(p),
            None => (),
        };
    }
    for p in pool_ratios.iter() {
        println!(
            "{}: {}\n{}: {}\n",
            p.pool.external_asset.symbol,
            p.pool.external_asset.balance,
            p.pool.native_asset.symbol,
            p.pool.native_asset.balance,
        );
    }
    pool_ratios
}

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

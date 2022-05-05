use serde::{Deserialize, Serialize};
pub mod osmosis {
    pub mod gamm {
        pub mod v1beta1 {
            include!("built/osmosis.gamm.v1beta1.rs");
        }
    }
}
pub mod google {
    pub mod api {
        include!("built/google.api.rs");
    }
}
pub mod cosmos_proto {
    include!("built/cosmos_proto.rs");
}

pub mod cosmos {
    pub mod bank {
        pub mod v1beta1 {
            include!("built/cosmos.bank.v1beta1.rs");
        }
    }
    pub mod base {
        pub mod v1beta1 {
            include!("built/cosmos.base.v1beta1.rs");
        }
        pub mod query {
            pub mod v1beta1 {
                include!("built/cosmos.base.query.v1beta1.rs");
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MyPoolSummary {
    pub address: String,
    pub id: u64,
    pub pool_params: MyPoolParams,
    pub future_pool_governor: String,
    pub total_shares: MyTotalShares,
    pub pool_assets: Vec<MyPoolAssets>,
    pub total_weight: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MyPoolAssets {
    pub token: MyToken,
    pub weight: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MyToken {
    pub denom: String,
    pub amount: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MyTotalShares {
    pub denom: String,
    pub amount: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MyPoolParams {
    pub swap_fee: f64,
    pub exit_fee: f64,
    pub smooth_weight_change_params: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PoolsWrapper {
    pub pools: Vec<Pool>,
    pub pagination: Pagination,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pagination {
    pub next_key: Option<String>,
    pub total: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pool {
    #[serde(alias = "@type")]
    pub type_info: String,
    pub address: String,
    pub id: String,
    #[serde(alias = "poolParams")]
    pub pool_params: PoolParams,
    pub future_pool_governor: String,
    #[serde(alias = "totalShares")]
    pub total_shares: TotalShares,
    #[serde(alias = "poolAssets")]
    pub pool_assets: Vec<PoolAssets>,
    #[serde(alias = "totalWeight")]
    pub total_weight: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PoolParams {
    #[serde(alias = "swapFee")]
    pub swap_fee: String,
    #[serde(alias = "exitFee")]
    pub exit_fee: String,
    #[serde(alias = "smoothWeightChangeParams")]
    pub smooth_weight_change_params: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TotalShares {
    pub denom: String,
    pub amount: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PoolAssets {
    pub token: Token,
    pub weight: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub denom: String,
    pub amount: String,
}

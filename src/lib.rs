use serde::{Serialize, Deserialize};

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
    pub pool_id: String,
    pub pair_symbols: Vec<String>,
    pub pair_raw: Vec<String>,
    pub swap_fee: f64,
    pub exit_fee: f64,
    pub smooth_weight_change_params: Option<String>,
    pub total_shares: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PoolsWrapper {
    pub pools: Vec<Pool>,
    pub pagination: Pagination,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pagination {
    pub next_key: Option<String>,
    pub total: String
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Pool {
    #[serde(alias = "@type")]
    pub type_info: String,
    pub address: String,
    pub id: String,
    pub poolParams: PoolParams,
    pub future_pool_governor: String,
    pub totalShares: TotalShares,
    pub poolAssets: Vec<PoolAssets>,
    pub totalWeight: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PoolParams {
    pub swapFee: String,
    pub exitFee: String,
    pub smoothWeightChangeParams: Option<String>
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

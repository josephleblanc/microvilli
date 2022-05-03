use microvilli::{PoolsWrapper, MyPoolSummary};
use microvilli::osmosis::gamm::v1beta1::{
    query_client::QueryClient,
    QueryNumPoolsRequest,
    QuerySpotPriceRequest,
};

use reqwest;
use tokio;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("main - no error");
    let mut denom_dict: HashMap<&str, &str> = HashMap::new();
    let insert_list: Vec<(&str, &str)> = vec![
        ("ibc/8318FD63C42203D16DDCAF49FE10E8590669B3219A3E87676AC9DA50722687FB", "rowan"),
        ("ibc/0EF15DF2F02480ADE0BB6E85D9EBB5DAEA2836D3860E9F97F9AADE4F57A31AA0", "luna"),
        ("ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2", "atom"),
        ("ibc/46B44899322F3CD854D2D46DEEF881958467CDD4B3B10086DA49296BBED94BED", "juno"),
        ("ibc/7C4D60AA95E5A7558B0A364860979CA34B7FF8AAF255B87AF9E879374470CEC0", "iris"),
        ("ibc/9712DBB13B9631EDFA9BF61B55F1B2D290B2ADB67E3A4EB3A875F3B6081B3B84", "dvpn"),
        ("ibc/E6931F78057F7CC5DA0FD6CEF82FF39373A6E0452BF1FD76910B93292CF356C1", "cro"),
        ("ibc/A0CC0CF735BFB30E730C70019D4218A1244FF383503FF7579C9201AB93CA9293", "xprt"),
        ("ibc/1DCC8A6CB5689018431323953344A9F6CC4D0BFB261E88C9F7777372C10CD076", "regen"),
        ("ibc/52B1AA623B34EB78FD767CEA69E8D7FA6C9CFE1FBF49C5406268FD325E2CC2AC", "iov"),
        ("ibc/1DC495FCEFDA068A3820F903EDBD78B942FBD204D7E93D3BA2B432E9669D1A59", "ngm"),
        ("ibc/5973C068568365FFF40DEDCF1A1CB7582B6116B731CD31A12231AE25E20B871F", "eeur"),
        ("ibc/BE1BB42D4BE3C30D50B68D7C41DB4DFCE9678E8EF8C539F6E6A9345048894FCC", "ixo"),
        ("ibc/0954E1C28EB7AF5B72D24F3BC2B47BBB2FDF91BDDFD57B74B99E133AED40972A", "scrt"),
        ("ibc/1480B8FD20AD5FCAE81EA87584D269547DD4D436843C1D20F15E00EB64743EF4", "akt")
    ];
    for (key, val) in insert_list {
        denom_dict.insert(key, val);
    }

    let mut client = QueryClient::connect("http://osmosis.strange.love:9090").await?;
    let query_spotswap = QuerySpotPriceRequest {
        pool_id: 1,
        base_asset_denom: String::from("ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2"),
        quote_asset_denom: String::from("uosmo"),
    };

    let request = tonic::Request::new( query_spotswap );

    let response = client.spot_price(request).await?;
    let msg = response.into_inner();

    println!("msg: {:?}", msg);

    Ok(())
}

async fn pools_summary() ->  Vec<MyPoolSummary>{
    let mut denom_dict: HashMap<&str, &str> = HashMap::new();
    let mut rev_dict: HashMap<&str, &str> = HashMap::new();
    let insert_list: Vec<(&str, &str)> = vec![
        ("ibc/8318FD63C42203D16DDCAF49FE10E8590669B3219A3E87676AC9DA50722687FB", "rowan"),
        ("ibc/0EF15DF2F02480ADE0BB6E85D9EBB5DAEA2836D3860E9F97F9AADE4F57A31AA0", "luna"),
        ("ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2", "atom"),
        ("ibc/46B44899322F3CD854D2D46DEEF881958467CDD4B3B10086DA49296BBED94BED", "juno"),
        ("ibc/7C4D60AA95E5A7558B0A364860979CA34B7FF8AAF255B87AF9E879374470CEC0", "iris"),
        ("ibc/9712DBB13B9631EDFA9BF61B55F1B2D290B2ADB67E3A4EB3A875F3B6081B3B84", "dvpn"),
        ("ibc/E6931F78057F7CC5DA0FD6CEF82FF39373A6E0452BF1FD76910B93292CF356C1", "cro"),
        ("ibc/A0CC0CF735BFB30E730C70019D4218A1244FF383503FF7579C9201AB93CA9293", "xprt"),
        ("ibc/1DCC8A6CB5689018431323953344A9F6CC4D0BFB261E88C9F7777372C10CD076", "regen"),
        ("ibc/52B1AA623B34EB78FD767CEA69E8D7FA6C9CFE1FBF49C5406268FD325E2CC2AC", "iov"),
        ("ibc/1DC495FCEFDA068A3820F903EDBD78B942FBD204D7E93D3BA2B432E9669D1A59", "ngm"),
        ("ibc/5973C068568365FFF40DEDCF1A1CB7582B6116B731CD31A12231AE25E20B871F", "eeur"),
        ("ibc/BE1BB42D4BE3C30D50B68D7C41DB4DFCE9678E8EF8C539F6E6A9345048894FCC", "ixo"),
        ("ibc/0954E1C28EB7AF5B72D24F3BC2B47BBB2FDF91BDDFD57B74B99E133AED40972A", "scrt"),
        ("ibc/1480B8FD20AD5FCAE81EA87584D269547DD4D436843C1D20F15E00EB64743EF4", "akt")
    ];
    for (key, val) in insert_list {
        denom_dict.insert(key, val);
        rev_dict.insert(val, key);
    }

    let endpoint = "https://osmosis.stakesystems.io/osmosis/gamm/v1beta1/pools?pagination.limit=694&pagination.count_total=false";
    let pools_wrapper: PoolsWrapper = reqwest::get(endpoint)
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    // pools_dict has the form <pool_id, [pair1, pair2, ..., pairn]>, and can accommodate as many
    // pools as there need to be. pairs are in their symbol form for readability.
    let mut pools_dict: HashMap<String, Vec<&str>> = HashMap::new();

    // Iterate through the pools received, looking for pools which match the pools in the
    // insert_list above. Since there are many pools available on Osmosis, it is useful to use the
    // insert_list as a way to specify which pools may be of interest. The pools chosen only need
    // to match the tokens listed, as those are the only ones on both Sifchain and Osmosis.
    // This loop will also grab information difficult to retreive through the gRPC, since the gRPC
    // uses proto.any on responses to this query and decodes using a go struct.
    // Other things this loop should get are:
    //  swap fee
    //  exit fee
    //  smooth weight change params
    //  total shares
    let mut summary_vec: Vec<MyPoolSummary> = vec![];
    for pool in pools_wrapper.pools {
        for pool_asset in &pool.poolAssets {
            if denom_dict.contains_key(pool_asset.token.denom.as_str()) {
                let pool_id = pools_dict
                    .entry(pool.id.to_owned())
                    .or_insert(vec![denom_dict[pool_asset.token.denom.as_str()]]);
                if !pool_id.contains(&denom_dict[pool_asset.token.denom.as_str()]) {
                    pool_id.push(denom_dict[pool_asset.token.denom.as_str()]);
                    if pool_id.len() == 2 {
                        summary_vec.push( MyPoolSummary {
                            pool_id: String::from((&pool.id).clone()),
                            pair_symbols: pool_id.iter()
                                .map(|symbol| String::from(*symbol))
                                .collect::<Vec<String>>(),
                            pair_raw: pool_id.iter()
                                .map(|symbol| String::from(rev_dict[symbol]))
                                .collect::<Vec<String>>(),
                            swap_fee: pool.poolParams.swapFee.parse::<f64>().unwrap(),
                            exit_fee: pool.poolParams.exitFee.parse::<f64>().unwrap(),
                            smooth_weight_change_params: pool.poolParams
                                .smoothWeightChangeParams.clone(),
                            total_shares: pool.totalShares.amount.parse::<f64>().unwrap(),
                        });
                    }
                }
            }
        }
    }

    summary_vec
}

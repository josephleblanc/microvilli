mod sifs_hound;
use futures::future::{join, try_join_all};
use microvilli::osmosis::gamm::v1beta1::{query_client::QueryClient, QuerySpotPriceRequest};
use microvilli::{MyPoolAssets, MyPoolParams, MyPoolSummary, MyToken, MyTotalShares, PoolsWrapper};
use std::collections::HashMap;
use std::error::Error;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("main - no error");
    let mut denom_dict: HashMap<&str, &str> = HashMap::new();
    let insert_list: Vec<(&str, &str)> = vec![
        (
            "ibc/8318FD63C42203D16DDCAF49FE10E8590669B3219A3E87676AC9DA50722687FB",
            "rowan",
        ),
        (
            "ibc/0EF15DF2F02480ADE0BB6E85D9EBB5DAEA2836D3860E9F97F9AADE4F57A31AA0",
            "luna",
        ),
        (
            "ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2",
            "atom",
        ),
        (
            "ibc/46B44899322F3CD854D2D46DEEF881958467CDD4B3B10086DA49296BBED94BED",
            "juno",
        ),
        (
            "ibc/7C4D60AA95E5A7558B0A364860979CA34B7FF8AAF255B87AF9E879374470CEC0",
            "iris",
        ),
        (
            "ibc/9712DBB13B9631EDFA9BF61B55F1B2D290B2ADB67E3A4EB3A875F3B6081B3B84",
            "dvpn",
        ),
        (
            "ibc/E6931F78057F7CC5DA0FD6CEF82FF39373A6E0452BF1FD76910B93292CF356C1",
            "cro",
        ),
        (
            "ibc/A0CC0CF735BFB30E730C70019D4218A1244FF383503FF7579C9201AB93CA9293",
            "xprt",
        ),
        (
            "ibc/1DCC8A6CB5689018431323953344A9F6CC4D0BFB261E88C9F7777372C10CD076",
            "regen",
        ),
        (
            "ibc/52B1AA623B34EB78FD767CEA69E8D7FA6C9CFE1FBF49C5406268FD325E2CC2AC",
            "iov",
        ),
        (
            "ibc/1DC495FCEFDA068A3820F903EDBD78B942FBD204D7E93D3BA2B432E9669D1A59",
            "ngm",
        ),
        (
            "ibc/5973C068568365FFF40DEDCF1A1CB7582B6116B731CD31A12231AE25E20B871F",
            "eeur",
        ),
        (
            "ibc/BE1BB42D4BE3C30D50B68D7C41DB4DFCE9678E8EF8C539F6E6A9345048894FCC",
            "ixo",
        ),
        (
            "ibc/0954E1C28EB7AF5B72D24F3BC2B47BBB2FDF91BDDFD57B74B99E133AED40972A",
            "scrt",
        ),
        (
            "ibc/1480B8FD20AD5FCAE81EA87584D269547DD4D436843C1D20F15E00EB64743EF4",
            "akt",
        ),
    ];

    let pools_sum_vec = pools_summary(insert_list.clone()).await.unwrap();

    for (key, val) in &insert_list {
        denom_dict.insert(key, val);
    }

    let client = QueryClient::connect("http://osmosis.strange.love:9090").await?;

    //let requests = pools_summary.iter().map(|pool_summary| {
    //    tonic::Request::new(QuerySpotPriceRequest {
    //        pool_id: pool_summary.id,
    //        base_asset_denom: pool_summary.pool_assets[0].token.denom.clone(),
    //        quote_asset_denom: pool_summary.pool_assets[1].token.denom.clone(),
    //    })
    //});

    let mut interval = time::interval(Duration::from_secs(6));
    loop {
        let start_time = Instant::now();

        let insert_list_clone = insert_list.clone();
        let denom_dict_clone = denom_dict.clone();

        //let (tx1, mut rx) = mpsc::channel(2);
        //let tx2 = tx1.clone();

        let osmo_handle = tokio::spawn(async move {
            let pools_sum_vec = pools_summary(insert_list_clone.clone()).await.unwrap();
            let client = QueryClient::connect("http://osmosis.strange.love:9090")
                .await
                .unwrap();
            //let mut responses = vec![];
            let requests = pools_sum_vec.iter().map(|pool_summary| {
                tonic::Request::new(QuerySpotPriceRequest {
                    pool_id: pool_summary.id,
                    base_asset_denom: pool_summary.pool_assets[0].token.denom.clone(),
                    quote_asset_denom: pool_summary.pool_assets[1].token.denom.clone(),
                })
            });
            let mut responses = vec![];
            for request in requests.clone() {
                let mut new_client = client.clone();
                responses.push(async move { new_client.spot_price(request).await });
            }
            let results = try_join_all(responses).await.unwrap();
            let out_vec = results
                .into_iter()
                .map(|response| response.into_inner().spot_price.parse::<f64>().unwrap())
                .zip(pools_sum_vec.iter().map(|p| {
                    (
                        p.pool_assets[0].token.denom.to_owned(),
                        p.pool_assets[1].token.denom.to_owned(),
                    )
                }))
                .collect::<Vec<(f64, (String, String))>>();
            out_vec
            //tx1.send(out_vec).await;
        });
        //.await?;

        let sifs_handle = tokio::spawn(async move {
            let sif_pools = sifs_hound::sif_prices().await;
            let sif_prices = sif_pools
                .iter()
                .map(|sif_p| {
                    (
                        sif_p.pool.pool_units.parse::<f64>().unwrap(),
                        (
                            sif_p.pool.native_asset.symbol.to_owned(),
                            sif_p.pool.external_asset.symbol.to_owned(),
                        ),
                    )
                })
                .collect::<Vec<(f64, (String, String))>>();
            sif_prices
            //tx2.send(sif_prices).await;
        });
        //.await?;
        let (osmo_vec, sifs_vec) = join(osmo_handle, sifs_handle).await;
        println!("{:?}", osmo_vec.unwrap()[0]);
        println!("{:?}", sifs_vec.unwrap()[0]);
        //let received_results = rx.recv().await.unwrap();

        //let mut results_iter = received_results.iter();
        //println!("{:?}", results_iter.next());
        //println!(
        //    "{}/{}",
        //    denom_dict_clone[&pools_sum_vec[0].pool_assets[0].token.denom.as_str()],
        //    denom_dict_clone[&pools_sum_vec[0].pool_assets[1].token.denom.as_str()]
        //);
        let end_time_processing = Instant::now();
        interval.tick().await;
        let end_time_waiting = Instant::now();
        println!(
            "{} processing time",
            (end_time_processing - start_time).as_secs()
        );
        println!(
            "{} time waiting",
            (end_time_waiting - end_time_processing).as_secs()
        );
        println!("{} total time", (end_time_waiting - start_time).as_secs());
    }

    #[allow(unreachable_code)]
    Ok(())
}

async fn pools_summary(
    insert_list: Vec<(&str, &str)>,
) -> Result<Vec<MyPoolSummary>, Box<dyn Error>> {
    let mut denom_dict: HashMap<&str, &str> = HashMap::new();
    let mut rev_dict: HashMap<&str, &str> = HashMap::new();

    for (key, val) in insert_list {
        denom_dict.insert(key, val);
        rev_dict.insert(val, key);
    }

    let endpoint = "https://osmosis.stakesystems.io/osmosis/gamm/v1beta1/pools?pagination.limit=694&pagination.count_total=false";
    let pools_wrapper: PoolsWrapper = reqwest::get(endpoint).await.unwrap().json().await.unwrap();

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
        for pool_asset in &pool.pool_assets {
            if denom_dict.contains_key(pool_asset.token.denom.as_str()) {
                let pool_id = pools_dict
                    .entry(pool.id.to_owned())
                    .or_insert_with(|| vec![denom_dict[pool_asset.token.denom.as_str()]]);
                if !pool_id.contains(&denom_dict[pool_asset.token.denom.as_str()]) {
                    pool_id.push(denom_dict[pool_asset.token.denom.as_str()]);
                    if pool_id.len() == 2 {
                        summary_vec.push(MyPoolSummary {
                            address: pool.address.to_owned(),
                            id: pool.id.parse::<u64>().unwrap(),
                            pool_params: MyPoolParams {
                                swap_fee: pool.pool_params.swap_fee.parse::<f64>().unwrap(),
                                exit_fee: pool.pool_params.exit_fee.parse::<f64>().unwrap(),
                                smooth_weight_change_params: pool
                                    .pool_params
                                    .smooth_weight_change_params
                                    .to_owned(),
                            },
                            future_pool_governor: pool.future_pool_governor.to_owned(),
                            total_shares: MyTotalShares {
                                denom: pool.total_shares.denom.to_owned(),
                                amount: pool.total_shares.amount.parse::<f64>().unwrap(),
                            },
                            pool_assets: pool
                                .pool_assets
                                .iter()
                                .map(|pool_asset| MyPoolAssets {
                                    token: MyToken {
                                        denom: pool_asset.token.denom.to_owned(),
                                        amount: pool_asset.token.amount.parse::<f64>().unwrap(),
                                    },
                                    weight: pool_asset.weight.parse::<f64>().unwrap(),
                                })
                                .collect(),
                            total_weight: pool.total_weight.parse::<f64>().unwrap(),
                        });
                    }
                }
            }
        }
    }
    Ok(summary_vec)
}

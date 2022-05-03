fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .include_file("lib.rs")
        .out_dir("src/built")
        .compile(
            &[
                "osmosis/gamm/v1beta1/query.proto",
                "osmosis/gamm/v1beta1/tx.proto",
                "cosmos/bank/v1beta1/query.proto"
                //"gogoproto/gogo.proto",
                //"cosmos/base/v1beta1/coin.proto",
                //"cosmos/base/query/v1beta1/pagination.proto",
                //"cosmos_proto/cosmos.proto",
                //"google/api/annotations.proto",
                //"google/protobuf/any.proto",
            ],
            &["proto/"],
        )
        .unwrap();

    Ok(())
}
//fn main() {}

use tx_fees::query_client;

pub mod tx_fees {
    tonic::include_proto!("osmosis.txfees.v1beta1");
}

fn main() {
    println!("Hello, world!");

}

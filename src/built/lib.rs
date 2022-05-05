pub mod google {
    pub mod protobuf {
        include!("google.protobuf.rs");
    }
    pub mod api {
        include!("google.api.rs");
    }
}
pub mod cosmos {
    pub mod base {
        pub mod v1beta1 {
            include!("cosmos.base.v1beta1.rs");
        }
        pub mod query {
            pub mod v1beta1 {
                include!("cosmos.base.query.v1beta1.rs");
            }
        }
    }
    pub mod msg {
        pub mod v1 {
            include!("cosmos.msg.v1.rs");
        }
    }
    pub mod bank {
        pub mod v1beta1 {
            include!("cosmos.bank.v1beta1.rs");
        }
    }
}
pub mod gogoproto {
    include!("gogoproto.rs");
}
pub mod cosmos_proto {
    include!("cosmos_proto.rs");
}
pub mod osmosis {
    pub mod gamm {
        pub mod v1beta1 {
            include!("osmosis.gamm.v1beta1.rs");
        }
    }
}

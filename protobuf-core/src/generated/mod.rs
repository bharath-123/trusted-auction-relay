#![allow(
    unreachable_pub,
    clippy::pedantic,
    clippy::needless_borrows_for_generic_args,
    clippy::arithmetic_side_effects,
    clippy::needless_lifetimes
)]

#[path = ""]
pub mod astria {
    #[path = ""]
    pub mod auction {
        pub mod v1alpha1 {
            include!("astria.auction.v1alpha1.rs");
        }
    }
}

#[path = ""]
pub mod trusted_relay {

    #[path = ""]
    pub mod sequencer_apis {
        pub mod v1 {
            include!("trustedrelay.sequencerapis.v1.rs");
        }
    }

    #[path = ""]
    pub mod auctioneer_apis {
        pub mod v1 {
            include!("trustedrelay.auctioneerapis.v1.rs");
        }
    }
}

pub mod generated;
mod resources {
    pub use crate::generated::{
        account::*, account_link::*, application::*, application_fee::*,
        connect_collection_transfer::*, fee_refund::*, login_link::*, person::*, recipient::*,
        topup::*, transfer::*, transfer_reversal::*,
    };
}

pub mod generated;
mod resources {
    pub use crate::generated::{
        issuing_authorization::*, issuing_card::*, issuing_cardholder::*, issuing_dispute::*,
        issuing_settlement::*, issuing_transaction::*,
    };
}

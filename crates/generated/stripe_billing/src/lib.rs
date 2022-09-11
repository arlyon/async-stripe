pub mod generated;
mod resources {
    pub use crate::generated::{
        billing_portal_configuration::*, billing_portal_session::*, discount::*, invoice::*,
        invoiceitem::*, line_item::*, plan::*, price::*, promotion_code::*, quote::*,
        subscription::*, subscription_item::*, subscription_schedule::*, tax_id::*, tax_rate::*,
        usage_record_summary::*,
    };
}

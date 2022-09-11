pub mod generated;
mod resources {
    pub use crate::generated::{
        apps_secret::*, financial_connections_account::*, financial_connections_account_owner::*,
        financial_connections_account_ownership::*, financial_connections_session::*,
        funding_instructions::*, terminal_configuration::*, treasury_credit_reversal::*,
        treasury_debit_reversal::*, treasury_financial_account::*,
        treasury_financial_account_features::*, treasury_inbound_transfer::*,
        treasury_outbound_payment::*, treasury_outbound_transfer::*, treasury_received_credit::*,
        treasury_received_debit::*, treasury_transaction::*, treasury_transaction_entry::*,
    };
}

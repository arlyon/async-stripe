pub mod generated;
mod resources {
    pub use crate::generated::{
        apple_pay_domain::*, balance::*, balance_transaction::*, capability::*, cash_balance::*,
        charge::*, country_spec::*, coupon::*, credit_note::*, credit_note_line_item::*,
        customer::*, customer_balance_transaction::*, dispute::*, ephemeral_key::*,
        exchange_rate::*, file::*, file_link::*, identity_verification_report::*,
        identity_verification_session::*, item::*, mandate::*, notification_event_data::*,
        payment_intent::*, payout::*, platform_tax_fee::*, product::*,
        radar_early_fraud_warning::*, radar_value_list::*, radar_value_list_item::*, refund::*,
        reporting_report_run::*, reporting_report_type::*, reserve_transaction::*,
        setup_attempt::*, setup_intent::*, shipping_rate::*, sku::*,
        source_mandate_notification::*, source_transaction::*, tax_code::*,
        tax_deducted_at_source::*, terminal_connection_token::*, terminal_location::*,
        terminal_reader::*, test_helpers_test_clock::*, token::*, usage_record::*,
    };
}

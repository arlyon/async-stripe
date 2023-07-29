#![recursion_limit = "256"]
extern crate self as stripe_billing;

#[cfg(feature = "min-ser")]
miniserde::make_place!(Place);
pub mod credit_note;
pub use credit_note::CreditNote;
pub mod customer_balance_transaction;
pub use customer_balance_transaction::CustomerBalanceTransaction;
pub mod billing_portal;
pub mod credit_note_line_item;
pub use credit_note_line_item::CreditNoteLineItem;
pub mod invoice_item;
pub use invoice_item::InvoiceItem;
pub mod usage_record;
pub use usage_record::UsageRecord;
pub mod usage_record_summary;
pub use usage_record_summary::UsageRecordSummary;

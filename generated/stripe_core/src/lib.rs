#![recursion_limit = "256"]
extern crate self as stripe_core;

#[cfg(feature = "min-ser")]
miniserde::make_place!(Place);
pub mod balance;
pub use balance::Balance;
pub mod event;
pub use event::Event;
pub mod source_mandate_notification;
pub use source_mandate_notification::SourceMandateNotification;
pub mod token;
pub use token::Token;
pub mod customer_cash_balance_transaction;
pub use customer_cash_balance_transaction::CustomerCashBalanceTransaction;
pub mod payment_intent_type_specific_payment_method_options_client;
pub use payment_intent_type_specific_payment_method_options_client::PaymentIntentTypeSpecificPaymentMethodOptionsClient;

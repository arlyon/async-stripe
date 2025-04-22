#![recursion_limit = "256"]
#![allow(clippy::large_enum_variant)]
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::invalid_html_tags)]

//! This crate provides Rust bindings to the Stripe HTTP API, specifically
//! for requests mentioned in the `Terminal` section of the [Stripe API docs](https://stripe.com/docs/api)

extern crate self as stripe_terminal;

miniserde::make_place!(Place);
#[doc(hidden)]
pub mod deleted_terminal_configuration;
#[doc(inline)]
pub use deleted_terminal_configuration::*;
#[doc(hidden)]
pub mod deleted_terminal_location;
#[doc(inline)]
pub use deleted_terminal_location::*;
#[doc(hidden)]
pub mod deleted_terminal_reader;
#[doc(inline)]
pub use deleted_terminal_reader::*;
pub use terminal_configuration::types::*;
pub mod terminal_configuration;
pub use terminal_connection_token::types::*;
pub mod terminal_connection_token;
pub use terminal_location::types::*;
pub mod terminal_location;
pub use terminal_reader::types::*;
#[doc(hidden)]
pub mod terminal_configuration_configuration_resource_currency_specific_config;
pub mod terminal_reader;
#[doc(inline)]
pub use terminal_configuration_configuration_resource_currency_specific_config::*;
#[doc(hidden)]
pub mod terminal_configuration_configuration_resource_device_type_specific_config;
#[doc(inline)]
pub use terminal_configuration_configuration_resource_device_type_specific_config::*;
#[doc(hidden)]
pub mod terminal_configuration_configuration_resource_enterprise_peap_wifi;
#[doc(inline)]
pub use terminal_configuration_configuration_resource_enterprise_peap_wifi::*;
#[doc(hidden)]
pub mod terminal_configuration_configuration_resource_enterprise_tls_wifi;
#[doc(inline)]
pub use terminal_configuration_configuration_resource_enterprise_tls_wifi::*;
#[doc(hidden)]
pub mod terminal_configuration_configuration_resource_offline_config;
#[doc(inline)]
pub use terminal_configuration_configuration_resource_offline_config::*;
#[doc(hidden)]
pub mod terminal_configuration_configuration_resource_personal_psk_wifi;
#[doc(inline)]
pub use terminal_configuration_configuration_resource_personal_psk_wifi::*;
#[doc(hidden)]
pub mod terminal_configuration_configuration_resource_reboot_window;
#[doc(inline)]
pub use terminal_configuration_configuration_resource_reboot_window::*;
#[doc(hidden)]
pub mod terminal_configuration_configuration_resource_tipping;
#[doc(inline)]
pub use terminal_configuration_configuration_resource_tipping::*;
#[doc(hidden)]
pub mod terminal_configuration_configuration_resource_wifi_config;
#[doc(inline)]
pub use terminal_configuration_configuration_resource_wifi_config::*;
#[doc(hidden)]
pub mod terminal_reader_reader_resource_cart;
#[doc(inline)]
pub use terminal_reader_reader_resource_cart::*;
#[doc(hidden)]
pub mod terminal_reader_reader_resource_line_item;
#[doc(inline)]
pub use terminal_reader_reader_resource_line_item::*;
#[doc(hidden)]
pub mod terminal_reader_reader_resource_process_config;
#[doc(inline)]
pub use terminal_reader_reader_resource_process_config::*;
#[doc(hidden)]
pub mod terminal_reader_reader_resource_process_payment_intent_action;
#[doc(inline)]
pub use terminal_reader_reader_resource_process_payment_intent_action::*;
#[doc(hidden)]
pub mod terminal_reader_reader_resource_process_setup_config;
#[doc(inline)]
pub use terminal_reader_reader_resource_process_setup_config::*;
#[doc(hidden)]
pub mod terminal_reader_reader_resource_process_setup_intent_action;
#[doc(inline)]
pub use terminal_reader_reader_resource_process_setup_intent_action::*;
#[doc(hidden)]
pub mod terminal_reader_reader_resource_reader_action;
#[doc(inline)]
pub use terminal_reader_reader_resource_reader_action::*;
#[doc(hidden)]
pub mod terminal_reader_reader_resource_refund_payment_action;
#[doc(inline)]
pub use terminal_reader_reader_resource_refund_payment_action::*;
#[doc(hidden)]
pub mod terminal_reader_reader_resource_refund_payment_config;
#[doc(inline)]
pub use terminal_reader_reader_resource_refund_payment_config::*;
#[doc(hidden)]
pub mod terminal_reader_reader_resource_set_reader_display_action;
#[doc(inline)]
pub use terminal_reader_reader_resource_set_reader_display_action::*;
#[doc(hidden)]
pub mod terminal_reader_reader_resource_tipping_config;
#[doc(inline)]
pub use terminal_reader_reader_resource_tipping_config::*;

// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{TerminalReaderId};
use crate::params::{Expandable, Metadata, Object};
use crate::resources::{Charge, Currency, PaymentIntent, Refund, SetupIntent, TerminalLocation};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "TerminalReaderReader".
///
/// For more details see <https://stripe.com/docs/api/terminal/readers/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TerminalReader {
    /// Unique identifier for the object.
    pub id: TerminalReaderId,

    /// The most recent action performed by the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<TerminalReaderReaderResourceReaderAction>,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    /// The current software version of the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_sw_version: Option<String>,

    /// Type of reader, one of `bbpos_wisepad3`, `stripe_m2`, `bbpos_chipper2x`, `bbpos_wisepos_e`, `verifone_P400`, or `simulated_wisepos_e`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<TerminalReaderDeviceType>,

    /// The local IP address of the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,

    /// Custom label given to the reader for easier identification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub livemode: Option<bool>,

    /// The location identifier of the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Expandable<TerminalLocation>>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Serial number of the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,

    /// The networking status of the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl Object for TerminalReader {
    type Id = TerminalReaderId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "terminal.reader"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TerminalReaderReaderResourceReaderAction {

    /// Failure code, only set if status is `failed`.
    pub failure_code: Option<String>,

    /// Detailed failure message, only set if status is `failed`.
    pub failure_message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_payment_intent: Option<TerminalReaderReaderResourceProcessPaymentIntentAction>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_setup_intent: Option<TerminalReaderReaderResourceProcessSetupIntentAction>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_payment: Option<TerminalReaderReaderResourceRefundPaymentAction>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_reader_display: Option<TerminalReaderReaderResourceSetReaderDisplayAction>,

    /// Status of the action performed by the reader.
    pub status: TerminalReaderReaderResourceReaderActionStatus,

    /// Type of action performed by the reader.
    #[serde(rename = "type")]
    pub type_: TerminalReaderReaderResourceReaderActionType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TerminalReaderReaderResourceProcessPaymentIntentAction {

    /// Most recent PaymentIntent processed by the reader.
    pub payment_intent: Expandable<PaymentIntent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_config: Option<TerminalReaderReaderResourceProcessConfig>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TerminalReaderReaderResourceProcessConfig {

    /// Override showing a tipping selection screen on this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_tipping: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipping: Option<TerminalReaderReaderResourceTippingConfig>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TerminalReaderReaderResourceProcessSetupIntentAction {

    /// ID of a card PaymentMethod generated from the card_present PaymentMethod that may be attached to a Customer for future transactions.
    ///
    /// Only present if it was possible to generate a card PaymentMethod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_card: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_config: Option<TerminalReaderReaderResourceProcessSetupConfig>,

    /// Most recent SetupIntent processed by the reader.
    pub setup_intent: Expandable<SetupIntent>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TerminalReaderReaderResourceProcessSetupConfig {
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TerminalReaderReaderResourceRefundPaymentAction {

    /// The amount being refunded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// Charge that is being refunded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<Expandable<Charge>>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Payment intent that is being refunded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<Expandable<PaymentIntent>>,

    /// The reason for the refund.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<TerminalReaderReaderResourceRefundPaymentActionReason>,

    /// Unique identifier for the refund object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund: Option<Expandable<Refund>>,

    /// Boolean indicating whether the application fee should be refunded when refunding this charge.
    ///
    /// If a full charge refund is given, the full application fee will be refunded.
    /// Otherwise, the application fee will be refunded in an amount proportional to the amount of the charge refunded.
    /// An application fee can be refunded only by the application that created the charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_application_fee: Option<bool>,

    /// Boolean indicating whether the transfer should be reversed when refunding this charge.
    ///
    /// The transfer will be reversed proportionally to the amount being refunded (either the entire or partial amount).
    /// A transfer can be reversed only by the application that created the charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_transfer: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TerminalReaderReaderResourceSetReaderDisplayAction {

    /// Cart object to be displayed by the reader.
    pub cart: Option<TerminalReaderReaderResourceCart>,

    /// Type of information to be displayed by the reader.
    #[serde(rename = "type")]
    pub type_: TerminalReaderReaderResourceSetReaderDisplayActionType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TerminalReaderReaderResourceCart {

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// List of line items in the cart.
    pub line_items: Vec<TerminalReaderReaderResourceLineItem>,

    /// Tax amount for the entire cart.
    ///
    /// A positive integer in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub tax: Option<i64>,

    /// Total amount for the entire cart, including tax.
    ///
    /// A positive integer in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub total: i64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TerminalReaderReaderResourceLineItem {

    /// The amount of the line item.
    ///
    /// A positive integer in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount: i64,

    /// Description of the line item.
    pub description: String,

    /// The quantity of the line item.
    pub quantity: u64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TerminalReaderReaderResourceTippingConfig {

    /// Amount used to calculate tip suggestions on tipping selection screen for this transaction.
    ///
    /// Must be a positive integer in the smallest currency unit (e.g., 100 cents to represent $1.00 or 100 to represent ¥100, a zero-decimal currency).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_eligible: Option<i64>,
}

/// An enum representing the possible values of an `TerminalReader`'s `device_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TerminalReaderDeviceType {
    BbposChipper2x,
    BbposWisepad3,
    BbposWiseposE,
    SimulatedWiseposE,
    StripeM2,
    #[serde(rename = "verifone_P400")]
    VerifoneP400,
}

impl TerminalReaderDeviceType {
    pub fn as_str(self) -> &'static str {
        match self {
            TerminalReaderDeviceType::BbposChipper2x => "bbpos_chipper2x",
            TerminalReaderDeviceType::BbposWisepad3 => "bbpos_wisepad3",
            TerminalReaderDeviceType::BbposWiseposE => "bbpos_wisepos_e",
            TerminalReaderDeviceType::SimulatedWiseposE => "simulated_wisepos_e",
            TerminalReaderDeviceType::StripeM2 => "stripe_m2",
            TerminalReaderDeviceType::VerifoneP400 => "verifone_P400",
        }
    }
}

impl AsRef<str> for TerminalReaderDeviceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TerminalReaderDeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TerminalReaderDeviceType {
    fn default() -> Self {
        Self::BbposChipper2x
    }
}

/// An enum representing the possible values of an `TerminalReaderReaderResourceReaderAction`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TerminalReaderReaderResourceReaderActionStatus {
    Failed,
    InProgress,
    Succeeded,
}

impl TerminalReaderReaderResourceReaderActionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            TerminalReaderReaderResourceReaderActionStatus::Failed => "failed",
            TerminalReaderReaderResourceReaderActionStatus::InProgress => "in_progress",
            TerminalReaderReaderResourceReaderActionStatus::Succeeded => "succeeded",
        }
    }
}

impl AsRef<str> for TerminalReaderReaderResourceReaderActionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TerminalReaderReaderResourceReaderActionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TerminalReaderReaderResourceReaderActionStatus {
    fn default() -> Self {
        Self::Failed
    }
}

/// An enum representing the possible values of an `TerminalReaderReaderResourceReaderAction`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TerminalReaderReaderResourceReaderActionType {
    ProcessPaymentIntent,
    ProcessSetupIntent,
    RefundPayment,
    SetReaderDisplay,
}

impl TerminalReaderReaderResourceReaderActionType {
    pub fn as_str(self) -> &'static str {
        match self {
            TerminalReaderReaderResourceReaderActionType::ProcessPaymentIntent => "process_payment_intent",
            TerminalReaderReaderResourceReaderActionType::ProcessSetupIntent => "process_setup_intent",
            TerminalReaderReaderResourceReaderActionType::RefundPayment => "refund_payment",
            TerminalReaderReaderResourceReaderActionType::SetReaderDisplay => "set_reader_display",
        }
    }
}

impl AsRef<str> for TerminalReaderReaderResourceReaderActionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TerminalReaderReaderResourceReaderActionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TerminalReaderReaderResourceReaderActionType {
    fn default() -> Self {
        Self::ProcessPaymentIntent
    }
}

/// An enum representing the possible values of an `TerminalReaderReaderResourceRefundPaymentAction`'s `reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TerminalReaderReaderResourceRefundPaymentActionReason {
    Duplicate,
    Fraudulent,
    RequestedByCustomer,
}

impl TerminalReaderReaderResourceRefundPaymentActionReason {
    pub fn as_str(self) -> &'static str {
        match self {
            TerminalReaderReaderResourceRefundPaymentActionReason::Duplicate => "duplicate",
            TerminalReaderReaderResourceRefundPaymentActionReason::Fraudulent => "fraudulent",
            TerminalReaderReaderResourceRefundPaymentActionReason::RequestedByCustomer => "requested_by_customer",
        }
    }
}

impl AsRef<str> for TerminalReaderReaderResourceRefundPaymentActionReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TerminalReaderReaderResourceRefundPaymentActionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TerminalReaderReaderResourceRefundPaymentActionReason {
    fn default() -> Self {
        Self::Duplicate
    }
}

/// An enum representing the possible values of an `TerminalReaderReaderResourceSetReaderDisplayAction`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TerminalReaderReaderResourceSetReaderDisplayActionType {
    Cart,
}

impl TerminalReaderReaderResourceSetReaderDisplayActionType {
    pub fn as_str(self) -> &'static str {
        match self {
            TerminalReaderReaderResourceSetReaderDisplayActionType::Cart => "cart",
        }
    }
}

impl AsRef<str> for TerminalReaderReaderResourceSetReaderDisplayActionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TerminalReaderReaderResourceSetReaderDisplayActionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TerminalReaderReaderResourceSetReaderDisplayActionType {
    fn default() -> Self {
        Self::Cart
    }
}

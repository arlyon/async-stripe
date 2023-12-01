// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{CustomerCashBalanceTransactionId};
use crate::params::{Expandable, Object, Timestamp};
use crate::resources::{BalanceTransaction, Currency, Customer, PaymentIntent, Refund};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "CustomerCashBalanceTransaction".
///
/// For more details see <https://stripe.com/docs/api/cash_balance_transactions/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CustomerCashBalanceTransaction {
    /// Unique identifier for the object.
    pub id: CustomerCashBalanceTransactionId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjusted_for_overdraft: Option<CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraft>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_to_payment: Option<CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransaction>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// The customer whose available cash balance changed as a result of this transaction.
    pub customer: Expandable<Customer>,

    /// The total available cash balance for the specified currency after this transaction was applied.
    ///
    /// Represented in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub ending_balance: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub funded: Option<CustomerBalanceResourceCashBalanceTransactionResourceFundedTransaction>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The amount by which the cash balance changed, represented in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    ///
    /// A positive value represents funds being added to the cash balance, a negative value represents funds being removed from the cash balance.
    pub net_amount: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunded_from_payment: Option<CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransaction>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transferred_to_balance: Option<CustomerBalanceResourceCashBalanceTransactionResourceTransferredToBalance>,

    /// The type of the cash balance transaction.
    ///
    /// New types may be added in future.
    /// See [Customer Balance](https://stripe.com/docs/payments/customer-balance#types) to learn more about these types.
    #[serde(rename = "type")]
    pub type_: CustomerCashBalanceTransactionType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unapplied_from_payment: Option<CustomerBalanceResourceCashBalanceTransactionResourceUnappliedFromPaymentTransaction>,
}

impl Object for CustomerCashBalanceTransaction {
    type Id = CustomerCashBalanceTransactionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "customer_cash_balance_transaction"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraft {

    /// The [Balance Transaction](https://stripe.com/docs/api/balance_transactions/object) that corresponds to funds taken out of your Stripe balance.
    pub balance_transaction: Expandable<BalanceTransaction>,

    /// The [Cash Balance Transaction](https://stripe.com/docs/api/cash_balance_transactions/object) that brought the customer balance negative, triggering the clawback of funds.
    pub linked_transaction: Expandable<CustomerCashBalanceTransaction>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransaction {

    /// The [Payment Intent](https://stripe.com/docs/api/payment_intents/object) that funds were applied to.
    pub payment_intent: Expandable<PaymentIntent>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceFundedTransaction {

    pub bank_transfer: CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransfer,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransfer {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_bank_transfer: Option<CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceEuBankTransfer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gb_bank_transfer: Option<CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceGbBankTransfer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub jp_bank_transfer: Option<CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceJpBankTransfer>,

    /// The user-supplied reference field on the bank transfer.
    pub reference: Option<String>,

    /// The funding method type used to fund the customer balance.
    ///
    /// Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, `mx_bank_transfer`, or `us_bank_transfer`.
    #[serde(rename = "type")]
    pub type_: CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_transfer: Option<CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceUsBankTransfer>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceEuBankTransfer {

    /// The BIC of the bank of the sender of the funding.
    pub bic: Option<String>,

    /// The last 4 digits of the IBAN of the sender of the funding.
    pub iban_last4: Option<String>,

    /// The full name of the sender, as supplied by the sending bank.
    pub sender_name: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceGbBankTransfer {

    /// The last 4 digits of the account number of the sender of the funding.
    pub account_number_last4: Option<String>,

    /// The full name of the sender, as supplied by the sending bank.
    pub sender_name: Option<String>,

    /// The sort code of the bank of the sender of the funding.
    pub sort_code: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceJpBankTransfer {

    /// The name of the bank of the sender of the funding.
    pub sender_bank: Option<String>,

    /// The name of the bank branch of the sender of the funding.
    pub sender_branch: Option<String>,

    /// The full name of the sender, as supplied by the sending bank.
    pub sender_name: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceUsBankTransfer {

    /// The banking network used for this funding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceUsBankTransferNetwork>,

    /// The full name of the sender, as supplied by the sending bank.
    pub sender_name: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransaction {

    /// The [Refund](https://stripe.com/docs/api/refunds/object) that moved these funds into the customer's cash balance.
    pub refund: Expandable<Refund>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceTransferredToBalance {

    /// The [Balance Transaction](https://stripe.com/docs/api/balance_transactions/object) that corresponds to funds transferred to your Stripe balance.
    pub balance_transaction: Expandable<BalanceTransaction>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceUnappliedFromPaymentTransaction {

    /// The [Payment Intent](https://stripe.com/docs/api/payment_intents/object) that funds were unapplied from.
    pub payment_intent: Expandable<PaymentIntent>,
}

/// An enum representing the possible values of an `CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceUsBankTransfer`'s `network` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceUsBankTransferNetwork {
    Ach,
    DomesticWireUs,
    Swift,
}

impl CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceUsBankTransferNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceUsBankTransferNetwork::Ach => "ach",
            CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceUsBankTransferNetwork::DomesticWireUs => "domestic_wire_us",
            CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceUsBankTransferNetwork::Swift => "swift",
        }
    }
}

impl AsRef<str> for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceUsBankTransferNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceUsBankTransferNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceUsBankTransferNetwork {
    fn default() -> Self {
        Self::Ach
    }
}

/// An enum representing the possible values of an `CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransfer`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType {
    EuBankTransfer,
    GbBankTransfer,
    JpBankTransfer,
    MxBankTransfer,
    UsBankTransfer,
}

impl CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType {
    pub fn as_str(self) -> &'static str {
        match self {
            CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType::EuBankTransfer => "eu_bank_transfer",
            CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType::GbBankTransfer => "gb_bank_transfer",
            CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType::JpBankTransfer => "jp_bank_transfer",
            CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType::MxBankTransfer => "mx_bank_transfer",
            CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType::UsBankTransfer => "us_bank_transfer",
        }
    }
}

impl AsRef<str> for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType {
    fn default() -> Self {
        Self::EuBankTransfer
    }
}

/// An enum representing the possible values of an `CustomerCashBalanceTransaction`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CustomerCashBalanceTransactionType {
    AdjustedForOverdraft,
    AppliedToPayment,
    Funded,
    FundingReversed,
    RefundedFromPayment,
    ReturnCanceled,
    ReturnInitiated,
    TransferredToBalance,
    UnappliedFromPayment,
}

impl CustomerCashBalanceTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            CustomerCashBalanceTransactionType::AdjustedForOverdraft => "adjusted_for_overdraft",
            CustomerCashBalanceTransactionType::AppliedToPayment => "applied_to_payment",
            CustomerCashBalanceTransactionType::Funded => "funded",
            CustomerCashBalanceTransactionType::FundingReversed => "funding_reversed",
            CustomerCashBalanceTransactionType::RefundedFromPayment => "refunded_from_payment",
            CustomerCashBalanceTransactionType::ReturnCanceled => "return_canceled",
            CustomerCashBalanceTransactionType::ReturnInitiated => "return_initiated",
            CustomerCashBalanceTransactionType::TransferredToBalance => "transferred_to_balance",
            CustomerCashBalanceTransactionType::UnappliedFromPayment => "unapplied_from_payment",
        }
    }
}

impl AsRef<str> for CustomerCashBalanceTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CustomerCashBalanceTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CustomerCashBalanceTransactionType {
    fn default() -> Self {
        Self::AdjustedForOverdraft
    }
}

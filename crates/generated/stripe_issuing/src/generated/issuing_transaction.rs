// ======================================
// This file was automatically generated.
// ======================================

use async_stripe_common::resources::Currency;
use async_stripe_core::{
    ids::IssuingTransactionId,
    params::{Expandable, Metadata, Object, Timestamp},
    resources::IssuingTransactionType,
};
use serde::{Deserialize, Serialize};

use crate::resources::{
    BalanceTransaction, IssuingAuthorization, IssuingCard, IssuingCardholder, IssuingDispute,
    IssuingTransactionPurchaseDetails,
};

/// The resource representing a Stripe "IssuingTransaction".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingTransaction {
    /// Unique identifier for the object.
    pub id: IssuingTransactionId,

    /// The transaction amount, which will be reflected in your balance.
    ///
    /// This amount is in your currency and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount: i64,

    /// Detailed breakdown of amount components.
    ///
    /// These amounts are denominated in `currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_details: Option<IssuingTransactionAmountDetails>,

    /// The `Authorization` object that led to this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<Expandable<IssuingAuthorization>>,

    /// ID of the [balance transaction](https://stripe.com/docs/api/balance_transactions) associated with this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_transaction: Option<Expandable<BalanceTransaction>>,

    /// The card used to make this transaction.
    pub card: Expandable<IssuingCard>,

    /// The cardholder to whom this transaction belongs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder: Option<Expandable<IssuingCardholder>>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// If you've disputed the transaction, the ID of the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispute: Option<Expandable<IssuingDispute>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The amount that the merchant will receive, denominated in `merchant_currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    ///
    /// It will be different from `amount` if the merchant is taking payment in a different currency.
    pub merchant_amount: i64,

    /// The currency with which the merchant is taking payment.
    pub merchant_currency: Currency,

    pub merchant_data: MerchantData,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// Additional purchase information that is optionally provided by the merchant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_details: Option<IssuingTransactionPurchaseDetails>,

    /// [Treasury](https://stripe.com/docs/api/treasury) details related to this transaction if it was created on a [FinancialAccount](/docs/api/treasury/financial_accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<IssuingTransactionTreasury>,

    /// The nature of the transaction.
    #[serde(rename = "type")]
    pub type_: IssuingTransactionType,

    /// The digital wallet used for this transaction.
    ///
    /// One of `apple_pay`, `google_pay`, or `samsung_pay`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet: Option<IssuingTransactionWallet>,
}

impl Object for IssuingTransaction {
    type Id = IssuingTransactionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "issuing.transaction"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MerchantData {
    /// A categorization of the seller's type of business.
    ///
    /// See our [merchant categories guide](https://stripe.com/docs/issuing/merchant-categories) for a list of possible values.
    pub category: String,

    /// The merchant category code for the seller’s business.
    pub category_code: String,

    /// City where the seller is located.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// Country where the seller is located.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Name of the seller.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Identifier assigned to the seller by the card brand.
    pub network_id: String,

    /// Postal code where the seller is located.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    /// State where the seller is located.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingTransactionAmountDetails {
    /// The fee charged by the ATM for the cash withdrawal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atm_fee: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingTransactionTreasury {
    /// The Treasury [ReceivedCredit](https://stripe.com/docs/api/treasury/received_debits) representing this Issuing transaction if it is a refund.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_credit: Option<String>,

    /// The Treasury [ReceivedDebit](https://stripe.com/docs/api/treasury/received_credits) representing this Issuing transaction if it is a capture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_debit: Option<String>,
}

/// An enum representing the possible values of an `IssuingTransaction`'s `wallet` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingTransactionWallet {
    ApplePay,
    GooglePay,
    SamsungPay,
}

impl IssuingTransactionWallet {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingTransactionWallet::ApplePay => "apple_pay",
            IssuingTransactionWallet::GooglePay => "google_pay",
            IssuingTransactionWallet::SamsungPay => "samsung_pay",
        }
    }
}

impl AsRef<str> for IssuingTransactionWallet {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingTransactionWallet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for IssuingTransactionWallet {
    fn default() -> Self {
        Self::ApplePay
    }
}

/// Any use of an [issued card](https://stripe.com/docs/issuing) that results in funds entering or leaving
/// your Stripe account, such as a completed purchase or refund, is represented by an Issuing
/// `Transaction` object.
///
/// Related guide: [Issued Card Transactions](https://stripe.com/docs/issuing/purchases/transactions).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Transaction {
    /// The transaction amount, which will be reflected in your balance.
    ///
    /// This amount is in your currency and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount: i64,
    /// Detailed breakdown of amount components.
    ///
    /// These amounts are denominated in `currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount_details: Option<stripe_core::issuing::transaction::amount_details::AmountDetails>,
    /// The `Authorization` object that led to this transaction.
    pub authorization:
        Option<stripe_types::Expandable<stripe_core::issuing::authorization::Authorization>>,
    /// ID of the [balance transaction](https://stripe.com/docs/api/balance_transactions) associated with this transaction.
    pub balance_transaction:
        Option<stripe_types::Expandable<stripe_core::balance_transaction::BalanceTransaction>>,
    /// The card used to make this transaction.
    pub card: stripe_types::Expandable<stripe_core::issuing::card::Card>,
    /// The cardholder to whom this transaction belongs.
    pub cardholder: Option<stripe_types::Expandable<stripe_core::issuing::cardholder::Cardholder>>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// If you've disputed the transaction, the ID of the dispute.
    pub dispute: Option<stripe_types::Expandable<stripe_core::issuing::dispute::Dispute>>,
    /// Unique identifier for the object.
    pub id: stripe_core::issuing::transaction::IssuingTransactionId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The amount that the merchant will receive, denominated in `merchant_currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    ///
    /// It will be different from `amount` if the merchant is taking payment in a different currency.
    pub merchant_amount: i64,
    /// The currency with which the merchant is taking payment.
    pub merchant_currency: stripe_types::Currency,
    pub merchant_data: stripe_core::issuing::authorization::merchant_data::MerchantData,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: stripe_types::Metadata,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: TransactionObject,
    /// Additional purchase information that is optionally provided by the merchant.
    pub purchase_details:
        Option<stripe_core::issuing::transaction::purchase_details::PurchaseDetails>,
    /// [Treasury](https://stripe.com/docs/api/treasury) details related to this transaction if it was created on a [FinancialAccount](/docs/api/treasury/financial_accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<stripe_core::issuing::transaction::treasury::Treasury>,
    /// The nature of the transaction.
    #[serde(rename = "type")]
    pub type_: TransactionType,
    /// The digital wallet used for this transaction.
    ///
    /// One of `apple_pay`, `google_pay`, or `samsung_pay`.
    pub wallet: Option<TransactionWallet>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Transaction {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum TransactionObject {
    #[serde(rename = "issuing.transaction")]
    IssuingTransaction,
}

impl TransactionObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::IssuingTransaction => "issuing.transaction",
        }
    }
}

impl AsRef<str> for TransactionObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TransactionObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The nature of the transaction.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum TransactionType {
    Capture,
    Refund,
}

impl TransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Capture => "capture",
            Self::Refund => "refund",
        }
    }
}

impl AsRef<str> for TransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The digital wallet used for this transaction.
///
/// One of `apple_pay`, `google_pay`, or `samsung_pay`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum TransactionWallet {
    ApplePay,
    GooglePay,
    SamsungPay,
}

impl TransactionWallet {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ApplePay => "apple_pay",
            Self::GooglePay => "google_pay",
            Self::SamsungPay => "samsung_pay",
        }
    }
}

impl AsRef<str> for TransactionWallet {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TransactionWallet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl stripe_types::Object for Transaction {
    type Id = stripe_core::issuing::transaction::IssuingTransactionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(IssuingTransactionId, "ipi_");
pub mod amount_details;
pub mod requests;
pub use amount_details::AmountDetails;
pub mod purchase_details;
pub use purchase_details::PurchaseDetails;
pub mod treasury;
pub use treasury::Treasury;

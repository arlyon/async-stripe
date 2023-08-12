/// Any use of an [issued card](https://stripe.com/docs/issuing) that results in funds entering or leaving
/// your Stripe account, such as a completed purchase or refund, is represented by an Issuing
/// `Transaction` object.
///
/// Related guide: [Issued card transactions](https://stripe.com/docs/issuing/purchases/transactions).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IssuingTransaction {
    /// The transaction amount, which will be reflected in your balance.
    ///
    /// This amount is in your currency and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount: i64,
    /// Detailed breakdown of amount components.
    ///
    /// These amounts are denominated in `currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount_details: Option<stripe_types::IssuingTransactionAmountDetails>,
    /// The `Authorization` object that led to this transaction.
    pub authorization: Option<stripe_types::Expandable<stripe_types::IssuingAuthorization>>,
    /// ID of the [balance transaction](https://stripe.com/docs/api/balance_transactions) associated with this transaction.
    pub balance_transaction: Option<stripe_types::Expandable<stripe_types::BalanceTransaction>>,
    /// The card used to make this transaction.
    pub card: stripe_types::Expandable<stripe_types::IssuingCard>,
    /// The cardholder to whom this transaction belongs.
    pub cardholder: Option<stripe_types::Expandable<stripe_types::IssuingCardholder>>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// If you've disputed the transaction, the ID of the dispute.
    pub dispute: Option<stripe_types::Expandable<stripe_types::IssuingDispute>>,
    /// Unique identifier for the object.
    pub id: stripe_types::issuing_transaction::IssuingTransactionId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The amount that the merchant will receive, denominated in `merchant_currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    ///
    /// It will be different from `amount` if the merchant is taking payment in a different currency.
    pub merchant_amount: i64,
    /// The currency with which the merchant is taking payment.
    pub merchant_currency: stripe_types::Currency,
    pub merchant_data: stripe_types::IssuingAuthorizationMerchantData,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// Additional purchase information that is optionally provided by the merchant.
    pub purchase_details: Option<stripe_types::IssuingTransactionPurchaseDetails>,
    /// [Treasury](https://stripe.com/docs/api/treasury) details related to this transaction if it was created on a [FinancialAccount](/docs/api/treasury/financial_accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<stripe_types::IssuingTransactionTreasury>,
    /// The nature of the transaction.
    #[serde(rename = "type")]
    pub type_: IssuingTransactionType,
    /// The digital wallet used for this transaction.
    ///
    /// One of `apple_pay`, `google_pay`, or `samsung_pay`.
    pub wallet: Option<IssuingTransactionWallet>,
}
/// The nature of the transaction.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingTransactionType {
    Capture,
    Refund,
}

impl IssuingTransactionType {
    pub fn as_str(self) -> &'static str {
        use IssuingTransactionType::*;
        match self {
            Capture => "capture",
            Refund => "refund",
        }
    }
}

impl std::str::FromStr for IssuingTransactionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingTransactionType::*;
        match s {
            "capture" => Ok(Capture),
            "refund" => Ok(Refund),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for IssuingTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingTransactionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingTransactionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for IssuingTransactionType"))
    }
}
/// The digital wallet used for this transaction.
///
/// One of `apple_pay`, `google_pay`, or `samsung_pay`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingTransactionWallet {
    ApplePay,
    GooglePay,
    SamsungPay,
}

impl IssuingTransactionWallet {
    pub fn as_str(self) -> &'static str {
        use IssuingTransactionWallet::*;
        match self {
            ApplePay => "apple_pay",
            GooglePay => "google_pay",
            SamsungPay => "samsung_pay",
        }
    }
}

impl std::str::FromStr for IssuingTransactionWallet {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingTransactionWallet::*;
        match s {
            "apple_pay" => Ok(ApplePay),
            "google_pay" => Ok(GooglePay),
            "samsung_pay" => Ok(SamsungPay),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingTransactionWallet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingTransactionWallet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingTransactionWallet {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for IssuingTransactionWallet"))
    }
}
impl stripe_types::Object for IssuingTransaction {
    type Id = stripe_types::issuing_transaction::IssuingTransactionId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
stripe_types::def_id!(IssuingTransactionId, "ipi_");

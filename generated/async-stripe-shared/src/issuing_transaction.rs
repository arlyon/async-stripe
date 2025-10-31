/// Any use of an [issued card](https://stripe.com/docs/issuing) that results in funds entering or leaving.
/// your Stripe account, such as a completed purchase or refund, is represented by an Issuing
/// `Transaction` object.
///
/// Related guide: [Issued card transactions](https://stripe.com/docs/issuing/purchases/transactions)
///
/// For more details see <<https://stripe.com/docs/api/issuing/transactions/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingTransaction {
    /// The transaction amount, which will be reflected in your balance.
    /// This amount is in your currency and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount: i64,
    /// Detailed breakdown of amount components.
    /// These amounts are denominated in `currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount_details: Option<stripe_shared::IssuingTransactionAmountDetails>,
    /// The `Authorization` object that led to this transaction.
    pub authorization: Option<stripe_types::Expandable<stripe_shared::IssuingAuthorization>>,
    /// ID of the [balance transaction](https://stripe.com/docs/api/balance_transactions) associated with this transaction.
    pub balance_transaction: Option<stripe_types::Expandable<stripe_shared::BalanceTransaction>>,
    /// The card used to make this transaction.
    pub card: stripe_types::Expandable<stripe_shared::IssuingCard>,
    /// The cardholder to whom this transaction belongs.
    pub cardholder: Option<stripe_types::Expandable<stripe_shared::IssuingCardholder>>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// If you've disputed the transaction, the ID of the dispute.
    pub dispute: Option<stripe_types::Expandable<stripe_shared::IssuingDispute>>,
    /// Unique identifier for the object.
    pub id: stripe_shared::IssuingTransactionId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The amount that the merchant will receive, denominated in `merchant_currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    /// It will be different from `amount` if the merchant is taking payment in a different currency.
    pub merchant_amount: i64,
    /// The currency with which the merchant is taking payment.
    pub merchant_currency: stripe_types::Currency,
    pub merchant_data: stripe_shared::IssuingAuthorizationMerchantData,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// Details about the transaction, such as processing dates, set by the card network.
    pub network_data: Option<stripe_shared::IssuingTransactionNetworkData>,
    /// Additional purchase information that is optionally provided by the merchant.
    pub purchase_details: Option<stripe_shared::IssuingTransactionPurchaseDetails>,
    /// [Token](https://stripe.com/docs/api/issuing/tokens/object) object used for this transaction.
    /// If a network token was not used for this transaction, this field will be null.
    pub token: Option<stripe_types::Expandable<stripe_shared::IssuingToken>>,
    /// [Treasury](https://stripe.com/docs/api/treasury) details related to this transaction if it was created on a [FinancialAccount](/docs/api/treasury/financial_accounts.
    pub treasury: Option<stripe_shared::IssuingTransactionTreasury>,
    /// The nature of the transaction.
    #[cfg_attr(feature = "deserialize", serde(rename = "type"))]
    pub type_: stripe_shared::IssuingTransactionType,
    /// The digital wallet used for this transaction. One of `apple_pay`, `google_pay`, or `samsung_pay`.
    pub wallet: Option<IssuingTransactionWallet>,
}
#[doc(hidden)]
pub struct IssuingTransactionBuilder {
    amount: Option<i64>,
    amount_details: Option<Option<stripe_shared::IssuingTransactionAmountDetails>>,
    authorization: Option<Option<stripe_types::Expandable<stripe_shared::IssuingAuthorization>>>,
    balance_transaction:
        Option<Option<stripe_types::Expandable<stripe_shared::BalanceTransaction>>>,
    card: Option<stripe_types::Expandable<stripe_shared::IssuingCard>>,
    cardholder: Option<Option<stripe_types::Expandable<stripe_shared::IssuingCardholder>>>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    dispute: Option<Option<stripe_types::Expandable<stripe_shared::IssuingDispute>>>,
    id: Option<stripe_shared::IssuingTransactionId>,
    livemode: Option<bool>,
    merchant_amount: Option<i64>,
    merchant_currency: Option<stripe_types::Currency>,
    merchant_data: Option<stripe_shared::IssuingAuthorizationMerchantData>,
    metadata: Option<std::collections::HashMap<String, String>>,
    network_data: Option<Option<stripe_shared::IssuingTransactionNetworkData>>,
    purchase_details: Option<Option<stripe_shared::IssuingTransactionPurchaseDetails>>,
    token: Option<Option<stripe_types::Expandable<stripe_shared::IssuingToken>>>,
    treasury: Option<Option<stripe_shared::IssuingTransactionTreasury>>,
    type_: Option<stripe_shared::IssuingTransactionType>,
    wallet: Option<Option<IssuingTransactionWallet>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingTransaction {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingTransaction>,
        builder: IssuingTransactionBuilder,
    }

    impl Visitor for Place<IssuingTransaction> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingTransactionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingTransactionBuilder {
        type Out = IssuingTransaction;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "amount_details" => Deserialize::begin(&mut self.amount_details),
                "authorization" => Deserialize::begin(&mut self.authorization),
                "balance_transaction" => Deserialize::begin(&mut self.balance_transaction),
                "card" => Deserialize::begin(&mut self.card),
                "cardholder" => Deserialize::begin(&mut self.cardholder),
                "created" => Deserialize::begin(&mut self.created),
                "currency" => Deserialize::begin(&mut self.currency),
                "dispute" => Deserialize::begin(&mut self.dispute),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "merchant_amount" => Deserialize::begin(&mut self.merchant_amount),
                "merchant_currency" => Deserialize::begin(&mut self.merchant_currency),
                "merchant_data" => Deserialize::begin(&mut self.merchant_data),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "network_data" => Deserialize::begin(&mut self.network_data),
                "purchase_details" => Deserialize::begin(&mut self.purchase_details),
                "token" => Deserialize::begin(&mut self.token),
                "treasury" => Deserialize::begin(&mut self.treasury),
                "type" => Deserialize::begin(&mut self.type_),
                "wallet" => Deserialize::begin(&mut self.wallet),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                amount_details: Deserialize::default(),
                authorization: Deserialize::default(),
                balance_transaction: Deserialize::default(),
                card: Deserialize::default(),
                cardholder: Deserialize::default(),
                created: Deserialize::default(),
                currency: Deserialize::default(),
                dispute: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                merchant_amount: Deserialize::default(),
                merchant_currency: Deserialize::default(),
                merchant_data: Deserialize::default(),
                metadata: Deserialize::default(),
                network_data: Deserialize::default(),
                purchase_details: Deserialize::default(),
                token: Deserialize::default(),
                treasury: Deserialize::default(),
                type_: Deserialize::default(),
                wallet: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(amount),
                Some(amount_details),
                Some(authorization),
                Some(balance_transaction),
                Some(card),
                Some(cardholder),
                Some(created),
                Some(currency),
                Some(dispute),
                Some(id),
                Some(livemode),
                Some(merchant_amount),
                Some(merchant_currency),
                Some(merchant_data),
                Some(metadata),
                Some(network_data),
                Some(purchase_details),
                Some(token),
                Some(treasury),
                Some(type_),
                Some(wallet),
            ) = (
                self.amount,
                self.amount_details,
                self.authorization.take(),
                self.balance_transaction.take(),
                self.card.take(),
                self.cardholder.take(),
                self.created,
                self.currency.take(),
                self.dispute.take(),
                self.id.take(),
                self.livemode,
                self.merchant_amount,
                self.merchant_currency.take(),
                self.merchant_data.take(),
                self.metadata.take(),
                self.network_data.take(),
                self.purchase_details.take(),
                self.token.take(),
                self.treasury.take(),
                self.type_,
                self.wallet,
            )
            else {
                return None;
            };
            Some(Self::Out {
                amount,
                amount_details,
                authorization,
                balance_transaction,
                card,
                cardholder,
                created,
                currency,
                dispute,
                id,
                livemode,
                merchant_amount,
                merchant_currency,
                merchant_data,
                metadata,
                network_data,
                purchase_details,
                token,
                treasury,
                type_,
                wallet,
            })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for IssuingTransaction {
        type Builder = IssuingTransactionBuilder;
    }

    impl FromValueOpt for IssuingTransaction {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingTransactionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "amount_details" => b.amount_details = FromValueOpt::from_value(v),
                    "authorization" => b.authorization = FromValueOpt::from_value(v),
                    "balance_transaction" => b.balance_transaction = FromValueOpt::from_value(v),
                    "card" => b.card = FromValueOpt::from_value(v),
                    "cardholder" => b.cardholder = FromValueOpt::from_value(v),
                    "created" => b.created = FromValueOpt::from_value(v),
                    "currency" => b.currency = FromValueOpt::from_value(v),
                    "dispute" => b.dispute = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "merchant_amount" => b.merchant_amount = FromValueOpt::from_value(v),
                    "merchant_currency" => b.merchant_currency = FromValueOpt::from_value(v),
                    "merchant_data" => b.merchant_data = FromValueOpt::from_value(v),
                    "metadata" => b.metadata = FromValueOpt::from_value(v),
                    "network_data" => b.network_data = FromValueOpt::from_value(v),
                    "purchase_details" => b.purchase_details = FromValueOpt::from_value(v),
                    "token" => b.token = FromValueOpt::from_value(v),
                    "treasury" => b.treasury = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    "wallet" => b.wallet = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingTransaction {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("IssuingTransaction", 22)?;
        s.serialize_field("amount", &self.amount)?;
        s.serialize_field("amount_details", &self.amount_details)?;
        s.serialize_field("authorization", &self.authorization)?;
        s.serialize_field("balance_transaction", &self.balance_transaction)?;
        s.serialize_field("card", &self.card)?;
        s.serialize_field("cardholder", &self.cardholder)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("dispute", &self.dispute)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("merchant_amount", &self.merchant_amount)?;
        s.serialize_field("merchant_currency", &self.merchant_currency)?;
        s.serialize_field("merchant_data", &self.merchant_data)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("network_data", &self.network_data)?;
        s.serialize_field("purchase_details", &self.purchase_details)?;
        s.serialize_field("token", &self.token)?;
        s.serialize_field("treasury", &self.treasury)?;
        s.serialize_field("type", &self.type_)?;
        s.serialize_field("wallet", &self.wallet)?;

        s.serialize_field("object", "issuing.transaction")?;
        s.end()
    }
}
/// The digital wallet used for this transaction. One of `apple_pay`, `google_pay`, or `samsung_pay`.
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
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingTransactionWallet::*;
        match s {
            "apple_pay" => Ok(ApplePay),
            "google_pay" => Ok(GooglePay),
            "samsung_pay" => Ok(SamsungPay),
            _ => Err(stripe_types::StripeParseError),
        }
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
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingTransactionWallet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IssuingTransactionWallet {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingTransactionWallet> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingTransactionWallet::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingTransactionWallet);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingTransactionWallet {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for IssuingTransactionWallet"))
    }
}
impl stripe_types::Object for IssuingTransaction {
    type Id = stripe_shared::IssuingTransactionId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(IssuingTransactionId);
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
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingTransactionType::*;
        match s {
            "capture" => Ok(Capture),
            "refund" => Ok(Refund),
            _ => Err(stripe_types::StripeParseError),
        }
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
impl miniserde::Deserialize for IssuingTransactionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingTransactionType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingTransactionType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingTransactionType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingTransactionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for IssuingTransactionType"))
    }
}

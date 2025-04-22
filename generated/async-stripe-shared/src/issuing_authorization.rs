/// When an [issued card](https://stripe.com/docs/issuing) is used to make a purchase, an Issuing `Authorization`.
/// object is created.
/// [Authorizations](https://stripe.com/docs/issuing/purchases/authorizations) must be approved for the.
/// purchase to be completed successfully.
///
/// Related guide: [Issued card authorizations](https://stripe.com/docs/issuing/purchases/authorizations).
///
/// For more details see <<https://stripe.com/docs/api/issuing/authorizations/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingAuthorization {
    /// The total amount that was authorized or rejected.
    /// This amount is in `currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    /// `amount` should be the same as `merchant_amount`, unless `currency` and `merchant_currency` are different.
    pub amount: i64,
    /// Detailed breakdown of amount components.
    /// These amounts are denominated in `currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount_details: Option<stripe_shared::IssuingAuthorizationAmountDetails>,
    /// Whether the authorization has been approved.
    pub approved: bool,
    /// How the card details were provided.
    pub authorization_method: stripe_shared::IssuingAuthorizationAuthorizationMethod,
    /// List of balance transactions associated with this authorization.
    pub balance_transactions: Vec<stripe_shared::BalanceTransaction>,
    pub card: stripe_shared::IssuingCard,
    /// The cardholder to whom this authorization belongs.
    pub cardholder: Option<stripe_types::Expandable<stripe_shared::IssuingCardholder>>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The currency of the cardholder.
    /// This currency can be different from the currency presented at authorization and the `merchant_currency` field on this authorization.
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Fleet-specific information for authorizations using Fleet cards.
    pub fleet: Option<stripe_shared::IssuingAuthorizationFleetData>,
    /// Fraud challenges sent to the cardholder, if this authorization was declined for fraud risk reasons.
    pub fraud_challenges: Option<Vec<stripe_shared::IssuingAuthorizationFraudChallenge>>,
    /// Information about fuel that was purchased with this transaction.
    /// Typically this information is received from the merchant after the authorization has been approved and the fuel dispensed.
    pub fuel: Option<stripe_shared::IssuingAuthorizationFuelData>,
    /// Unique identifier for the object.
    pub id: stripe_shared::IssuingAuthorizationId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The total amount that was authorized or rejected.
    /// This amount is in the `merchant_currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    /// `merchant_amount` should be the same as `amount`, unless `merchant_currency` and `currency` are different.
    pub merchant_amount: i64,
    /// The local currency that was presented to the cardholder for the authorization.
    /// This currency can be different from the cardholder currency and the `currency` field on this authorization.
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub merchant_currency: stripe_types::Currency,
    pub merchant_data: stripe_shared::IssuingAuthorizationMerchantData,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// Details about the authorization, such as identifiers, set by the card network.
    pub network_data: Option<stripe_shared::IssuingAuthorizationNetworkData>,
    /// The pending authorization request.
    /// This field will only be non-null during an `issuing_authorization.request` webhook.
    pub pending_request: Option<stripe_shared::IssuingAuthorizationPendingRequest>,
    /// History of every time a `pending_request` authorization was approved/declined, either by you directly or by Stripe (e.g.
    /// based on your spending_controls).
    /// If the merchant changes the authorization by performing an incremental authorization, you can look at this field to see the previous requests for the authorization.
    /// This field can be helpful in determining why a given authorization was approved/declined.
    pub request_history: Vec<stripe_shared::IssuingAuthorizationRequest>,
    /// The current status of the authorization in its lifecycle.
    pub status: stripe_shared::IssuingAuthorizationStatus,
    /// [Token](https://stripe.com/docs/api/issuing/tokens/object) object used for this authorization.
    /// If a network token was not used for this authorization, this field will be null.
    pub token: Option<stripe_types::Expandable<stripe_shared::IssuingToken>>,
    /// List of [transactions](https://stripe.com/docs/api/issuing/transactions) associated with this authorization.
    pub transactions: Vec<stripe_shared::IssuingTransaction>,
    /// [Treasury](https://stripe.com/docs/api/treasury) details related to this authorization if it was created on a [FinancialAccount](https://stripe.com/docs/api/treasury/financial_accounts).
    pub treasury: Option<stripe_shared::IssuingAuthorizationTreasury>,
    pub verification_data: stripe_shared::IssuingAuthorizationVerificationData,
    /// Whether the authorization bypassed fraud risk checks because the cardholder has previously completed a fraud challenge on a similar high-risk authorization from the same merchant.
    pub verified_by_fraud_challenge: Option<bool>,
    /// The digital wallet used for this transaction.
    /// One of `apple_pay`, `google_pay`, or `samsung_pay`.
    /// Will populate as `null` when no digital wallet was utilized.
    pub wallet: Option<String>,
}
#[doc(hidden)]
pub struct IssuingAuthorizationBuilder {
    amount: Option<i64>,
    amount_details: Option<Option<stripe_shared::IssuingAuthorizationAmountDetails>>,
    approved: Option<bool>,
    authorization_method: Option<stripe_shared::IssuingAuthorizationAuthorizationMethod>,
    balance_transactions: Option<Vec<stripe_shared::BalanceTransaction>>,
    card: Option<stripe_shared::IssuingCard>,
    cardholder: Option<Option<stripe_types::Expandable<stripe_shared::IssuingCardholder>>>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    fleet: Option<Option<stripe_shared::IssuingAuthorizationFleetData>>,
    fraud_challenges: Option<Option<Vec<stripe_shared::IssuingAuthorizationFraudChallenge>>>,
    fuel: Option<Option<stripe_shared::IssuingAuthorizationFuelData>>,
    id: Option<stripe_shared::IssuingAuthorizationId>,
    livemode: Option<bool>,
    merchant_amount: Option<i64>,
    merchant_currency: Option<stripe_types::Currency>,
    merchant_data: Option<stripe_shared::IssuingAuthorizationMerchantData>,
    metadata: Option<std::collections::HashMap<String, String>>,
    network_data: Option<Option<stripe_shared::IssuingAuthorizationNetworkData>>,
    pending_request: Option<Option<stripe_shared::IssuingAuthorizationPendingRequest>>,
    request_history: Option<Vec<stripe_shared::IssuingAuthorizationRequest>>,
    status: Option<stripe_shared::IssuingAuthorizationStatus>,
    token: Option<Option<stripe_types::Expandable<stripe_shared::IssuingToken>>>,
    transactions: Option<Vec<stripe_shared::IssuingTransaction>>,
    treasury: Option<Option<stripe_shared::IssuingAuthorizationTreasury>>,
    verification_data: Option<stripe_shared::IssuingAuthorizationVerificationData>,
    verified_by_fraud_challenge: Option<Option<bool>>,
    wallet: Option<Option<String>>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingAuthorization {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingAuthorization>,
        builder: IssuingAuthorizationBuilder,
    }

    impl Visitor for Place<IssuingAuthorization> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingAuthorizationBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingAuthorizationBuilder {
        type Out = IssuingAuthorization;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "amount_details" => Deserialize::begin(&mut self.amount_details),
                "approved" => Deserialize::begin(&mut self.approved),
                "authorization_method" => Deserialize::begin(&mut self.authorization_method),
                "balance_transactions" => Deserialize::begin(&mut self.balance_transactions),
                "card" => Deserialize::begin(&mut self.card),
                "cardholder" => Deserialize::begin(&mut self.cardholder),
                "created" => Deserialize::begin(&mut self.created),
                "currency" => Deserialize::begin(&mut self.currency),
                "fleet" => Deserialize::begin(&mut self.fleet),
                "fraud_challenges" => Deserialize::begin(&mut self.fraud_challenges),
                "fuel" => Deserialize::begin(&mut self.fuel),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "merchant_amount" => Deserialize::begin(&mut self.merchant_amount),
                "merchant_currency" => Deserialize::begin(&mut self.merchant_currency),
                "merchant_data" => Deserialize::begin(&mut self.merchant_data),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "network_data" => Deserialize::begin(&mut self.network_data),
                "pending_request" => Deserialize::begin(&mut self.pending_request),
                "request_history" => Deserialize::begin(&mut self.request_history),
                "status" => Deserialize::begin(&mut self.status),
                "token" => Deserialize::begin(&mut self.token),
                "transactions" => Deserialize::begin(&mut self.transactions),
                "treasury" => Deserialize::begin(&mut self.treasury),
                "verification_data" => Deserialize::begin(&mut self.verification_data),
                "verified_by_fraud_challenge" => {
                    Deserialize::begin(&mut self.verified_by_fraud_challenge)
                }
                "wallet" => Deserialize::begin(&mut self.wallet),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                amount_details: Deserialize::default(),
                approved: Deserialize::default(),
                authorization_method: Deserialize::default(),
                balance_transactions: Deserialize::default(),
                card: Deserialize::default(),
                cardholder: Deserialize::default(),
                created: Deserialize::default(),
                currency: Deserialize::default(),
                fleet: Deserialize::default(),
                fraud_challenges: Deserialize::default(),
                fuel: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                merchant_amount: Deserialize::default(),
                merchant_currency: Deserialize::default(),
                merchant_data: Deserialize::default(),
                metadata: Deserialize::default(),
                network_data: Deserialize::default(),
                pending_request: Deserialize::default(),
                request_history: Deserialize::default(),
                status: Deserialize::default(),
                token: Deserialize::default(),
                transactions: Deserialize::default(),
                treasury: Deserialize::default(),
                verification_data: Deserialize::default(),
                verified_by_fraud_challenge: Deserialize::default(),
                wallet: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(amount),
                Some(amount_details),
                Some(approved),
                Some(authorization_method),
                Some(balance_transactions),
                Some(card),
                Some(cardholder),
                Some(created),
                Some(currency),
                Some(fleet),
                Some(fraud_challenges),
                Some(fuel),
                Some(id),
                Some(livemode),
                Some(merchant_amount),
                Some(merchant_currency),
                Some(merchant_data),
                Some(metadata),
                Some(network_data),
                Some(pending_request),
                Some(request_history),
                Some(status),
                Some(token),
                Some(transactions),
                Some(treasury),
                Some(verification_data),
                Some(verified_by_fraud_challenge),
                Some(wallet),
            ) = (
                self.amount,
                self.amount_details,
                self.approved,
                self.authorization_method,
                self.balance_transactions.take(),
                self.card.take(),
                self.cardholder.take(),
                self.created,
                self.currency,
                self.fleet.take(),
                self.fraud_challenges.take(),
                self.fuel.take(),
                self.id.take(),
                self.livemode,
                self.merchant_amount,
                self.merchant_currency,
                self.merchant_data.take(),
                self.metadata.take(),
                self.network_data.take(),
                self.pending_request,
                self.request_history.take(),
                self.status,
                self.token.take(),
                self.transactions.take(),
                self.treasury.take(),
                self.verification_data.take(),
                self.verified_by_fraud_challenge,
                self.wallet.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                amount,
                amount_details,
                approved,
                authorization_method,
                balance_transactions,
                card,
                cardholder,
                created,
                currency,
                fleet,
                fraud_challenges,
                fuel,
                id,
                livemode,
                merchant_amount,
                merchant_currency,
                merchant_data,
                metadata,
                network_data,
                pending_request,
                request_history,
                status,
                token,
                transactions,
                treasury,
                verification_data,
                verified_by_fraud_challenge,
                wallet,
            })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for IssuingAuthorization {
        type Builder = IssuingAuthorizationBuilder;
    }

    impl FromValueOpt for IssuingAuthorization {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingAuthorizationBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "amount_details" => b.amount_details = FromValueOpt::from_value(v),
                    "approved" => b.approved = FromValueOpt::from_value(v),
                    "authorization_method" => b.authorization_method = FromValueOpt::from_value(v),
                    "balance_transactions" => b.balance_transactions = FromValueOpt::from_value(v),
                    "card" => b.card = FromValueOpt::from_value(v),
                    "cardholder" => b.cardholder = FromValueOpt::from_value(v),
                    "created" => b.created = FromValueOpt::from_value(v),
                    "currency" => b.currency = FromValueOpt::from_value(v),
                    "fleet" => b.fleet = FromValueOpt::from_value(v),
                    "fraud_challenges" => b.fraud_challenges = FromValueOpt::from_value(v),
                    "fuel" => b.fuel = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "merchant_amount" => b.merchant_amount = FromValueOpt::from_value(v),
                    "merchant_currency" => b.merchant_currency = FromValueOpt::from_value(v),
                    "merchant_data" => b.merchant_data = FromValueOpt::from_value(v),
                    "metadata" => b.metadata = FromValueOpt::from_value(v),
                    "network_data" => b.network_data = FromValueOpt::from_value(v),
                    "pending_request" => b.pending_request = FromValueOpt::from_value(v),
                    "request_history" => b.request_history = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    "token" => b.token = FromValueOpt::from_value(v),
                    "transactions" => b.transactions = FromValueOpt::from_value(v),
                    "treasury" => b.treasury = FromValueOpt::from_value(v),
                    "verification_data" => b.verification_data = FromValueOpt::from_value(v),
                    "verified_by_fraud_challenge" => {
                        b.verified_by_fraud_challenge = FromValueOpt::from_value(v)
                    }
                    "wallet" => b.wallet = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingAuthorization {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("IssuingAuthorization", 29)?;
        s.serialize_field("amount", &self.amount)?;
        s.serialize_field("amount_details", &self.amount_details)?;
        s.serialize_field("approved", &self.approved)?;
        s.serialize_field("authorization_method", &self.authorization_method)?;
        s.serialize_field("balance_transactions", &self.balance_transactions)?;
        s.serialize_field("card", &self.card)?;
        s.serialize_field("cardholder", &self.cardholder)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("fleet", &self.fleet)?;
        s.serialize_field("fraud_challenges", &self.fraud_challenges)?;
        s.serialize_field("fuel", &self.fuel)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("merchant_amount", &self.merchant_amount)?;
        s.serialize_field("merchant_currency", &self.merchant_currency)?;
        s.serialize_field("merchant_data", &self.merchant_data)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("network_data", &self.network_data)?;
        s.serialize_field("pending_request", &self.pending_request)?;
        s.serialize_field("request_history", &self.request_history)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("token", &self.token)?;
        s.serialize_field("transactions", &self.transactions)?;
        s.serialize_field("treasury", &self.treasury)?;
        s.serialize_field("verification_data", &self.verification_data)?;
        s.serialize_field("verified_by_fraud_challenge", &self.verified_by_fraud_challenge)?;
        s.serialize_field("wallet", &self.wallet)?;

        s.serialize_field("object", "issuing.authorization")?;
        s.end()
    }
}
impl stripe_types::Object for IssuingAuthorization {
    type Id = stripe_shared::IssuingAuthorizationId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(IssuingAuthorizationId);
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingAuthorizationAuthorizationMethod {
    Chip,
    Contactless,
    KeyedIn,
    Online,
    Swipe,
}
impl IssuingAuthorizationAuthorizationMethod {
    pub fn as_str(self) -> &'static str {
        use IssuingAuthorizationAuthorizationMethod::*;
        match self {
            Chip => "chip",
            Contactless => "contactless",
            KeyedIn => "keyed_in",
            Online => "online",
            Swipe => "swipe",
        }
    }
}

impl std::str::FromStr for IssuingAuthorizationAuthorizationMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingAuthorizationAuthorizationMethod::*;
        match s {
            "chip" => Ok(Chip),
            "contactless" => Ok(Contactless),
            "keyed_in" => Ok(KeyedIn),
            "online" => Ok(Online),
            "swipe" => Ok(Swipe),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for IssuingAuthorizationAuthorizationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingAuthorizationAuthorizationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingAuthorizationAuthorizationMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IssuingAuthorizationAuthorizationMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingAuthorizationAuthorizationMethod> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            IssuingAuthorizationAuthorizationMethod::from_str(s).map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingAuthorizationAuthorizationMethod);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingAuthorizationAuthorizationMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for IssuingAuthorizationAuthorizationMethod")
        })
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingAuthorizationStatus {
    Closed,
    Expired,
    Pending,
    Reversed,
}
impl IssuingAuthorizationStatus {
    pub fn as_str(self) -> &'static str {
        use IssuingAuthorizationStatus::*;
        match self {
            Closed => "closed",
            Expired => "expired",
            Pending => "pending",
            Reversed => "reversed",
        }
    }
}

impl std::str::FromStr for IssuingAuthorizationStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingAuthorizationStatus::*;
        match s {
            "closed" => Ok(Closed),
            "expired" => Ok(Expired),
            "pending" => Ok(Pending),
            "reversed" => Ok(Reversed),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for IssuingAuthorizationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingAuthorizationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingAuthorizationStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IssuingAuthorizationStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingAuthorizationStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingAuthorizationStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingAuthorizationStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingAuthorizationStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for IssuingAuthorizationStatus"))
    }
}

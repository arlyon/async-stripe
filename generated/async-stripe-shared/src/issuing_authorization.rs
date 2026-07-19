/// When an [issued card](https://docs.stripe.com/issuing) is used to make a purchase, an Issuing `Authorization`.
/// object is created.
/// [Authorizations](https://docs.stripe.com/issuing/purchases/authorizations) must be approved for the.
/// purchase to be completed successfully.
///
/// Related guide: [Issued card authorizations](https://docs.stripe.com/issuing/purchases/authorizations).
///
/// For more details see <<https://stripe.com/docs/api/issuing/authorizations/object>>.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingAuthorization {
    /// The total amount that was authorized or rejected.
    /// This amount is in `currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    /// `amount` should be the same as `merchant_amount`, unless `currency` and `merchant_currency` are different.
    pub amount: i64,
    /// Detailed breakdown of amount components.
    /// These amounts are denominated in `currency` and in the [smallest currency unit](https://docs.stripe.com/currencies#zero-decimal).
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
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
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
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
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
    /// [Token](https://docs.stripe.com/api/issuing/tokens/object) object used for this authorization.
    /// If a network token was not used for this authorization, this field will be null.
    pub token: Option<stripe_types::Expandable<stripe_shared::IssuingToken>>,
    /// List of [transactions](https://docs.stripe.com/api/issuing/transactions) associated with this authorization.
    pub transactions: Vec<stripe_shared::IssuingTransaction>,
    /// [Treasury](https://docs.stripe.com/api/treasury) details related to this authorization if it was created on a [FinancialAccount](https://docs.stripe.com/api/treasury/financial_accounts).
    pub treasury: Option<stripe_shared::IssuingAuthorizationTreasury>,
    pub verification_data: stripe_shared::IssuingAuthorizationVerificationData,
    /// Whether the authorization bypassed fraud risk checks because the cardholder has previously completed a fraud challenge on a similar high-risk authorization from the same merchant.
    pub verified_by_fraud_challenge: Option<bool>,
    /// The digital wallet used for this transaction.
    /// One of `apple_pay`, `google_pay`, or `samsung_pay`.
    /// Will populate as `null` when no digital wallet was utilized.
    pub wallet: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingAuthorization {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingAuthorization").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: IssuingAuthorizationBuilder {
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
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.builder.amount),
                "amount_details" => Deserialize::begin(&mut self.builder.amount_details),
                "approved" => Deserialize::begin(&mut self.builder.approved),
                "authorization_method" => {
                    Deserialize::begin(&mut self.builder.authorization_method)
                }
                "balance_transactions" => {
                    Deserialize::begin(&mut self.builder.balance_transactions)
                }
                "card" => Deserialize::begin(&mut self.builder.card),
                "cardholder" => Deserialize::begin(&mut self.builder.cardholder),
                "created" => Deserialize::begin(&mut self.builder.created),
                "currency" => Deserialize::begin(&mut self.builder.currency),
                "fleet" => Deserialize::begin(&mut self.builder.fleet),
                "fraud_challenges" => Deserialize::begin(&mut self.builder.fraud_challenges),
                "fuel" => Deserialize::begin(&mut self.builder.fuel),
                "id" => Deserialize::begin(&mut self.builder.id),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "merchant_amount" => Deserialize::begin(&mut self.builder.merchant_amount),
                "merchant_currency" => Deserialize::begin(&mut self.builder.merchant_currency),
                "merchant_data" => Deserialize::begin(&mut self.builder.merchant_data),
                "metadata" => Deserialize::begin(&mut self.builder.metadata),
                "network_data" => Deserialize::begin(&mut self.builder.network_data),
                "pending_request" => Deserialize::begin(&mut self.builder.pending_request),
                "request_history" => Deserialize::begin(&mut self.builder.request_history),
                "status" => Deserialize::begin(&mut self.builder.status),
                "token" => Deserialize::begin(&mut self.builder.token),
                "transactions" => Deserialize::begin(&mut self.builder.transactions),
                "treasury" => Deserialize::begin(&mut self.builder.treasury),
                "verification_data" => Deserialize::begin(&mut self.builder.verification_data),
                "verified_by_fraud_challenge" => {
                    Deserialize::begin(&mut self.builder.verified_by_fraud_challenge)
                }
                "wallet" => Deserialize::begin(&mut self.builder.wallet),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
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
                self.builder.amount,
                self.builder.amount_details,
                self.builder.approved,
                self.builder.authorization_method.take(),
                self.builder.balance_transactions.take(),
                self.builder.card.take(),
                self.builder.cardholder.take(),
                self.builder.created,
                self.builder.currency.take(),
                self.builder.fleet.take(),
                self.builder.fraud_challenges.take(),
                self.builder.fuel.take(),
                self.builder.id.take(),
                self.builder.livemode,
                self.builder.merchant_amount,
                self.builder.merchant_currency.take(),
                self.builder.merchant_data.take(),
                self.builder.metadata.take(),
                self.builder.network_data.take(),
                self.builder.pending_request.take(),
                self.builder.request_history.take(),
                self.builder.status.take(),
                self.builder.token.take(),
                self.builder.transactions.take(),
                self.builder.treasury.take(),
                self.builder.verification_data.take(),
                self.builder.verified_by_fraud_challenge,
                self.builder.wallet.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(IssuingAuthorization {
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
            });
            Ok(())
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum IssuingAuthorizationAuthorizationMethod {
    Chip,
    Contactless,
    KeyedIn,
    Online,
    Swipe,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl IssuingAuthorizationAuthorizationMethod {
    pub fn as_str(&self) -> &str {
        use IssuingAuthorizationAuthorizationMethod::*;
        match self {
            Chip => "chip",
            Contactless => "contactless",
            KeyedIn => "keyed_in",
            Online => "online",
            Swipe => "swipe",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for IssuingAuthorizationAuthorizationMethod {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingAuthorizationAuthorizationMethod::*;
        match s {
            "chip" => Ok(Chip),
            "contactless" => Ok(Contactless),
            "keyed_in" => Ok(KeyedIn),
            "online" => Ok(Online),
            "swipe" => Ok(Swipe),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "IssuingAuthorizationAuthorizationMethod"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for IssuingAuthorizationAuthorizationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for IssuingAuthorizationAuthorizationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingAuthorizationAuthorizationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(IssuingAuthorizationAuthorizationMethod)).finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for IssuingAuthorizationAuthorizationMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<IssuingAuthorizationAuthorizationMethod> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingAuthorizationAuthorizationMethod::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingAuthorizationAuthorizationMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum IssuingAuthorizationStatus {
    Closed,
    Expired,
    Pending,
    Reversed,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl IssuingAuthorizationStatus {
    pub fn as_str(&self) -> &str {
        use IssuingAuthorizationStatus::*;
        match self {
            Closed => "closed",
            Expired => "expired",
            Pending => "pending",
            Reversed => "reversed",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for IssuingAuthorizationStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingAuthorizationStatus::*;
        match s {
            "closed" => Ok(Closed),
            "expired" => Ok(Expired),
            "pending" => Ok(Pending),
            "reversed" => Ok(Reversed),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "IssuingAuthorizationStatus");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for IssuingAuthorizationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for IssuingAuthorizationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingAuthorizationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(IssuingAuthorizationStatus)).finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for IssuingAuthorizationStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<IssuingAuthorizationStatus> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingAuthorizationStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingAuthorizationStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}

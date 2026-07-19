/// Stripe Treasury provides users with a container for money called a FinancialAccount that is separate from their Payments balance.
/// FinancialAccounts serve as the source and destination of Treasury’s money movement APIs.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryFinancialAccount {
    /// The array of paths to active Features in the Features hash.
    pub active_features: Option<Vec<stripe_treasury::TreasuryFinancialAccountArray>>,
    pub balance: stripe_treasury::TreasuryFinancialAccountsResourceBalance,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: String,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    pub features: Option<stripe_treasury::TreasuryFinancialAccountFeatures>,
    /// The set of credentials that resolve to a FinancialAccount.
    pub financial_addresses:
        Vec<stripe_treasury::TreasuryFinancialAccountsResourceFinancialAddress>,
    /// Unique identifier for the object.
    pub id: stripe_treasury::TreasuryFinancialAccountId,
    pub is_default: Option<bool>,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The nickname for the FinancialAccount.
    pub nickname: Option<String>,
    /// The array of paths to pending Features in the Features hash.
    pub pending_features: Option<Vec<stripe_treasury::TreasuryFinancialAccountArray>>,
    /// The set of functionalities that the platform can restrict on the FinancialAccount.
    pub platform_restrictions:
        Option<stripe_treasury::TreasuryFinancialAccountsResourcePlatformRestrictions>,
    /// The array of paths to restricted Features in the Features hash.
    pub restricted_features: Option<Vec<stripe_treasury::TreasuryFinancialAccountArray>>,
    /// Status of this FinancialAccount.
    pub status: stripe_treasury::TreasuryFinancialAccountStatus,
    pub status_details: stripe_treasury::TreasuryFinancialAccountsResourceStatusDetails,
    /// The currencies the FinancialAccount can hold a balance in.
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    pub supported_currencies: Vec<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryFinancialAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TreasuryFinancialAccount").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TreasuryFinancialAccountBuilder {
    active_features: Option<Option<Vec<stripe_treasury::TreasuryFinancialAccountArray>>>,
    balance: Option<stripe_treasury::TreasuryFinancialAccountsResourceBalance>,
    country: Option<String>,
    created: Option<stripe_types::Timestamp>,
    features: Option<Option<stripe_treasury::TreasuryFinancialAccountFeatures>>,
    financial_addresses:
        Option<Vec<stripe_treasury::TreasuryFinancialAccountsResourceFinancialAddress>>,
    id: Option<stripe_treasury::TreasuryFinancialAccountId>,
    is_default: Option<Option<bool>>,
    livemode: Option<bool>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    nickname: Option<Option<String>>,
    pending_features: Option<Option<Vec<stripe_treasury::TreasuryFinancialAccountArray>>>,
    platform_restrictions:
        Option<Option<stripe_treasury::TreasuryFinancialAccountsResourcePlatformRestrictions>>,
    restricted_features: Option<Option<Vec<stripe_treasury::TreasuryFinancialAccountArray>>>,
    status: Option<stripe_treasury::TreasuryFinancialAccountStatus>,
    status_details: Option<stripe_treasury::TreasuryFinancialAccountsResourceStatusDetails>,
    supported_currencies: Option<Vec<String>>,
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

    impl Deserialize for TreasuryFinancialAccount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryFinancialAccount>,
        builder: TreasuryFinancialAccountBuilder,
    }

    impl Visitor for Place<TreasuryFinancialAccount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasuryFinancialAccountBuilder {
                    active_features: Deserialize::default(),
                    balance: Deserialize::default(),
                    country: Deserialize::default(),
                    created: Deserialize::default(),
                    features: Deserialize::default(),
                    financial_addresses: Deserialize::default(),
                    id: Deserialize::default(),
                    is_default: Deserialize::default(),
                    livemode: Deserialize::default(),
                    metadata: Deserialize::default(),
                    nickname: Deserialize::default(),
                    pending_features: Deserialize::default(),
                    platform_restrictions: Deserialize::default(),
                    restricted_features: Deserialize::default(),
                    status: Deserialize::default(),
                    status_details: Deserialize::default(),
                    supported_currencies: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "active_features" => Deserialize::begin(&mut self.builder.active_features),
                "balance" => Deserialize::begin(&mut self.builder.balance),
                "country" => Deserialize::begin(&mut self.builder.country),
                "created" => Deserialize::begin(&mut self.builder.created),
                "features" => Deserialize::begin(&mut self.builder.features),
                "financial_addresses" => Deserialize::begin(&mut self.builder.financial_addresses),
                "id" => Deserialize::begin(&mut self.builder.id),
                "is_default" => Deserialize::begin(&mut self.builder.is_default),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "metadata" => Deserialize::begin(&mut self.builder.metadata),
                "nickname" => Deserialize::begin(&mut self.builder.nickname),
                "pending_features" => Deserialize::begin(&mut self.builder.pending_features),
                "platform_restrictions" => {
                    Deserialize::begin(&mut self.builder.platform_restrictions)
                }
                "restricted_features" => Deserialize::begin(&mut self.builder.restricted_features),
                "status" => Deserialize::begin(&mut self.builder.status),
                "status_details" => Deserialize::begin(&mut self.builder.status_details),
                "supported_currencies" => {
                    Deserialize::begin(&mut self.builder.supported_currencies)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(active_features),
                Some(balance),
                Some(country),
                Some(created),
                Some(features),
                Some(financial_addresses),
                Some(id),
                Some(is_default),
                Some(livemode),
                Some(metadata),
                Some(nickname),
                Some(pending_features),
                Some(platform_restrictions),
                Some(restricted_features),
                Some(status),
                Some(status_details),
                Some(supported_currencies),
            ) = (
                self.builder.active_features.take(),
                self.builder.balance.take(),
                self.builder.country.take(),
                self.builder.created,
                self.builder.features.take(),
                self.builder.financial_addresses.take(),
                self.builder.id.take(),
                self.builder.is_default,
                self.builder.livemode,
                self.builder.metadata.take(),
                self.builder.nickname.take(),
                self.builder.pending_features.take(),
                self.builder.platform_restrictions.take(),
                self.builder.restricted_features.take(),
                self.builder.status.take(),
                self.builder.status_details.take(),
                self.builder.supported_currencies.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(TreasuryFinancialAccount {
                active_features,
                balance,
                country,
                created,
                features,
                financial_addresses,
                id,
                is_default,
                livemode,
                metadata,
                nickname,
                pending_features,
                platform_restrictions,
                restricted_features,
                status,
                status_details,
                supported_currencies,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for TreasuryFinancialAccount {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("TreasuryFinancialAccount", 18)?;
        s.serialize_field("active_features", &self.active_features)?;
        s.serialize_field("balance", &self.balance)?;
        s.serialize_field("country", &self.country)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("features", &self.features)?;
        s.serialize_field("financial_addresses", &self.financial_addresses)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("is_default", &self.is_default)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("nickname", &self.nickname)?;
        s.serialize_field("pending_features", &self.pending_features)?;
        s.serialize_field("platform_restrictions", &self.platform_restrictions)?;
        s.serialize_field("restricted_features", &self.restricted_features)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("status_details", &self.status_details)?;
        s.serialize_field("supported_currencies", &self.supported_currencies)?;

        s.serialize_field("object", "treasury.financial_account")?;
        s.end()
    }
}
impl stripe_types::Object for TreasuryFinancialAccount {
    type Id = stripe_treasury::TreasuryFinancialAccountId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(TreasuryFinancialAccountId);
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TreasuryFinancialAccountArray {
    CardIssuing,
    DepositInsurance,
    FinancialAddressesAba,
    FinancialAddressesAbaForwarding,
    InboundTransfersAch,
    IntraStripeFlows,
    OutboundPaymentsAch,
    OutboundPaymentsUsDomesticWire,
    OutboundTransfersAch,
    OutboundTransfersUsDomesticWire,
    RemoteDepositCapture,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TreasuryFinancialAccountArray {
    pub fn as_str(&self) -> &str {
        use TreasuryFinancialAccountArray::*;
        match self {
            CardIssuing => "card_issuing",
            DepositInsurance => "deposit_insurance",
            FinancialAddressesAba => "financial_addresses.aba",
            FinancialAddressesAbaForwarding => "financial_addresses.aba.forwarding",
            InboundTransfersAch => "inbound_transfers.ach",
            IntraStripeFlows => "intra_stripe_flows",
            OutboundPaymentsAch => "outbound_payments.ach",
            OutboundPaymentsUsDomesticWire => "outbound_payments.us_domestic_wire",
            OutboundTransfersAch => "outbound_transfers.ach",
            OutboundTransfersUsDomesticWire => "outbound_transfers.us_domestic_wire",
            RemoteDepositCapture => "remote_deposit_capture",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TreasuryFinancialAccountArray {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryFinancialAccountArray::*;
        match s {
            "card_issuing" => Ok(CardIssuing),
            "deposit_insurance" => Ok(DepositInsurance),
            "financial_addresses.aba" => Ok(FinancialAddressesAba),
            "financial_addresses.aba.forwarding" => Ok(FinancialAddressesAbaForwarding),
            "inbound_transfers.ach" => Ok(InboundTransfersAch),
            "intra_stripe_flows" => Ok(IntraStripeFlows),
            "outbound_payments.ach" => Ok(OutboundPaymentsAch),
            "outbound_payments.us_domestic_wire" => Ok(OutboundPaymentsUsDomesticWire),
            "outbound_transfers.ach" => Ok(OutboundTransfersAch),
            "outbound_transfers.us_domestic_wire" => Ok(OutboundTransfersUsDomesticWire),
            "remote_deposit_capture" => Ok(RemoteDepositCapture),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TreasuryFinancialAccountArray"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TreasuryFinancialAccountArray {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TreasuryFinancialAccountArray {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryFinancialAccountArray {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TreasuryFinancialAccountArray)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TreasuryFinancialAccountArray {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for TreasuryFinancialAccountArray {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<TreasuryFinancialAccountArray> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TreasuryFinancialAccountArray::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TreasuryFinancialAccountArray {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TreasuryFinancialAccountStatus {
    Closed,
    Open,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TreasuryFinancialAccountStatus {
    pub fn as_str(&self) -> &str {
        use TreasuryFinancialAccountStatus::*;
        match self {
            Closed => "closed",
            Open => "open",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TreasuryFinancialAccountStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryFinancialAccountStatus::*;
        match s {
            "closed" => Ok(Closed),
            "open" => Ok(Open),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TreasuryFinancialAccountStatus"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TreasuryFinancialAccountStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TreasuryFinancialAccountStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryFinancialAccountStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TreasuryFinancialAccountStatus)).finish_non_exhaustive()
    }
}
impl serde::Serialize for TreasuryFinancialAccountStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for TreasuryFinancialAccountStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<TreasuryFinancialAccountStatus> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TreasuryFinancialAccountStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TreasuryFinancialAccountStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}

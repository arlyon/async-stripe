/// An issuing token object is created when an issued card is added to a digital wallet.
/// As a [card issuer](https://docs.stripe.com/issuing), you can [view and manage these tokens](https://docs.stripe.com/issuing/controls/token-management) through Stripe.
///
/// For more details see <<https://stripe.com/docs/api/issuing/tokens/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingToken {
    /// Card associated with this token.
    pub card: stripe_types::Expandable<stripe_shared::IssuingCard>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The hashed ID derived from the device ID from the card network associated with the token.
    pub device_fingerprint: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_shared::IssuingTokenId,
    /// The last four digits of the token.
    pub last4: Option<String>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The token service provider / card network associated with the token.
    pub network: IssuingTokenNetwork,
    pub network_data: Option<stripe_shared::IssuingNetworkTokenNetworkData>,
    /// Time at which the token was last updated by the card network.
    /// Measured in seconds since the Unix epoch.
    pub network_updated_at: stripe_types::Timestamp,
    /// The usage state of the token.
    pub status: stripe_shared::IssuingTokenStatus,
    /// The digital wallet for this token, if one was used.
    pub wallet_provider: Option<IssuingTokenWalletProvider>,
}
#[doc(hidden)]
pub struct IssuingTokenBuilder {
    card: Option<stripe_types::Expandable<stripe_shared::IssuingCard>>,
    created: Option<stripe_types::Timestamp>,
    device_fingerprint: Option<Option<String>>,
    id: Option<stripe_shared::IssuingTokenId>,
    last4: Option<Option<String>>,
    livemode: Option<bool>,
    network: Option<IssuingTokenNetwork>,
    network_data: Option<Option<stripe_shared::IssuingNetworkTokenNetworkData>>,
    network_updated_at: Option<stripe_types::Timestamp>,
    status: Option<stripe_shared::IssuingTokenStatus>,
    wallet_provider: Option<Option<IssuingTokenWalletProvider>>,
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

    impl Deserialize for IssuingToken {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingToken>,
        builder: IssuingTokenBuilder,
    }

    impl Visitor for Place<IssuingToken> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingTokenBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingTokenBuilder {
        type Out = IssuingToken;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "card" => Deserialize::begin(&mut self.card),
                "created" => Deserialize::begin(&mut self.created),
                "device_fingerprint" => Deserialize::begin(&mut self.device_fingerprint),
                "id" => Deserialize::begin(&mut self.id),
                "last4" => Deserialize::begin(&mut self.last4),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "network" => Deserialize::begin(&mut self.network),
                "network_data" => Deserialize::begin(&mut self.network_data),
                "network_updated_at" => Deserialize::begin(&mut self.network_updated_at),
                "status" => Deserialize::begin(&mut self.status),
                "wallet_provider" => Deserialize::begin(&mut self.wallet_provider),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                card: Deserialize::default(),
                created: Deserialize::default(),
                device_fingerprint: Deserialize::default(),
                id: Deserialize::default(),
                last4: Deserialize::default(),
                livemode: Deserialize::default(),
                network: Deserialize::default(),
                network_data: Deserialize::default(),
                network_updated_at: Deserialize::default(),
                status: Deserialize::default(),
                wallet_provider: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(card),
                Some(created),
                Some(device_fingerprint),
                Some(id),
                Some(last4),
                Some(livemode),
                Some(network),
                Some(network_data),
                Some(network_updated_at),
                Some(status),
                Some(wallet_provider),
            ) = (
                self.card.take(),
                self.created,
                self.device_fingerprint.take(),
                self.id.take(),
                self.last4.take(),
                self.livemode,
                self.network.take(),
                self.network_data.take(),
                self.network_updated_at,
                self.status.take(),
                self.wallet_provider.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                card,
                created,
                device_fingerprint,
                id,
                last4,
                livemode,
                network,
                network_data,
                network_updated_at,
                status,
                wallet_provider,
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

    impl ObjectDeser for IssuingToken {
        type Builder = IssuingTokenBuilder;
    }

    impl FromValueOpt for IssuingToken {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingTokenBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "card" => b.card = FromValueOpt::from_value(v),
                    "created" => b.created = FromValueOpt::from_value(v),
                    "device_fingerprint" => b.device_fingerprint = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "last4" => b.last4 = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "network" => b.network = FromValueOpt::from_value(v),
                    "network_data" => b.network_data = FromValueOpt::from_value(v),
                    "network_updated_at" => b.network_updated_at = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    "wallet_provider" => b.wallet_provider = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingToken {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("IssuingToken", 12)?;
        s.serialize_field("card", &self.card)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("device_fingerprint", &self.device_fingerprint)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("last4", &self.last4)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("network", &self.network)?;
        s.serialize_field("network_data", &self.network_data)?;
        s.serialize_field("network_updated_at", &self.network_updated_at)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("wallet_provider", &self.wallet_provider)?;

        s.serialize_field("object", "issuing.token")?;
        s.end()
    }
}
/// The token service provider / card network associated with the token.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum IssuingTokenNetwork {
    Mastercard,
    Visa,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl IssuingTokenNetwork {
    pub fn as_str(&self) -> &str {
        use IssuingTokenNetwork::*;
        match self {
            Mastercard => "mastercard",
            Visa => "visa",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for IssuingTokenNetwork {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingTokenNetwork::*;
        match s {
            "mastercard" => Ok(Mastercard),
            "visa" => Ok(Visa),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "IssuingTokenNetwork");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for IssuingTokenNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingTokenNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingTokenNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IssuingTokenNetwork {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingTokenNetwork> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingTokenNetwork::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingTokenNetwork);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingTokenNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The digital wallet for this token, if one was used.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum IssuingTokenWalletProvider {
    ApplePay,
    GooglePay,
    SamsungPay,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl IssuingTokenWalletProvider {
    pub fn as_str(&self) -> &str {
        use IssuingTokenWalletProvider::*;
        match self {
            ApplePay => "apple_pay",
            GooglePay => "google_pay",
            SamsungPay => "samsung_pay",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for IssuingTokenWalletProvider {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingTokenWalletProvider::*;
        match s {
            "apple_pay" => Ok(ApplePay),
            "google_pay" => Ok(GooglePay),
            "samsung_pay" => Ok(SamsungPay),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "IssuingTokenWalletProvider");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for IssuingTokenWalletProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingTokenWalletProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingTokenWalletProvider {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IssuingTokenWalletProvider {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingTokenWalletProvider> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingTokenWalletProvider::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingTokenWalletProvider);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingTokenWalletProvider {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
impl stripe_types::Object for IssuingToken {
    type Id = stripe_shared::IssuingTokenId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(IssuingTokenId);
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum IssuingTokenStatus {
    Active,
    Deleted,
    Requested,
    Suspended,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl IssuingTokenStatus {
    pub fn as_str(&self) -> &str {
        use IssuingTokenStatus::*;
        match self {
            Active => "active",
            Deleted => "deleted",
            Requested => "requested",
            Suspended => "suspended",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for IssuingTokenStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingTokenStatus::*;
        match s {
            "active" => Ok(Active),
            "deleted" => Ok(Deleted),
            "requested" => Ok(Requested),
            "suspended" => Ok(Suspended),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "IssuingTokenStatus");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for IssuingTokenStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingTokenStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingTokenStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IssuingTokenStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingTokenStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingTokenStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingTokenStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingTokenStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}

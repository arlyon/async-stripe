#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BankConnectionsResourceAccountNumberDetails {
    /// When the account number is expected to expire, if applicable.
    pub expected_expiry_date: Option<stripe_types::Timestamp>,
    /// The type of account number associated with the account.
    pub identifier_type: BankConnectionsResourceAccountNumberDetailsIdentifierType,
    /// Whether the account number is currently active and usable for transactions.
    pub status: BankConnectionsResourceAccountNumberDetailsStatus,
    /// The payment networks that the account number can be used for.
    pub supported_networks: Vec<BankConnectionsResourceAccountNumberDetailsSupportedNetworks>,
}
#[doc(hidden)]
pub struct BankConnectionsResourceAccountNumberDetailsBuilder {
    expected_expiry_date: Option<Option<stripe_types::Timestamp>>,
    identifier_type: Option<BankConnectionsResourceAccountNumberDetailsIdentifierType>,
    status: Option<BankConnectionsResourceAccountNumberDetailsStatus>,
    supported_networks: Option<Vec<BankConnectionsResourceAccountNumberDetailsSupportedNetworks>>,
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

    impl Deserialize for BankConnectionsResourceAccountNumberDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BankConnectionsResourceAccountNumberDetails>,
        builder: BankConnectionsResourceAccountNumberDetailsBuilder,
    }

    impl Visitor for Place<BankConnectionsResourceAccountNumberDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BankConnectionsResourceAccountNumberDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BankConnectionsResourceAccountNumberDetailsBuilder {
        type Out = BankConnectionsResourceAccountNumberDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "expected_expiry_date" => Deserialize::begin(&mut self.expected_expiry_date),
                "identifier_type" => Deserialize::begin(&mut self.identifier_type),
                "status" => Deserialize::begin(&mut self.status),
                "supported_networks" => Deserialize::begin(&mut self.supported_networks),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                expected_expiry_date: Deserialize::default(),
                identifier_type: Deserialize::default(),
                status: Deserialize::default(),
                supported_networks: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(expected_expiry_date),
                Some(identifier_type),
                Some(status),
                Some(supported_networks),
            ) = (
                self.expected_expiry_date,
                self.identifier_type.take(),
                self.status.take(),
                self.supported_networks.take(),
            )
            else {
                return None;
            };
            Some(Self::Out { expected_expiry_date, identifier_type, status, supported_networks })
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

    impl ObjectDeser for BankConnectionsResourceAccountNumberDetails {
        type Builder = BankConnectionsResourceAccountNumberDetailsBuilder;
    }

    impl FromValueOpt for BankConnectionsResourceAccountNumberDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BankConnectionsResourceAccountNumberDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "expected_expiry_date" => b.expected_expiry_date = FromValueOpt::from_value(v),
                    "identifier_type" => b.identifier_type = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    "supported_networks" => b.supported_networks = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The type of account number associated with the account.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum BankConnectionsResourceAccountNumberDetailsIdentifierType {
    AccountNumber,
    TokenizedAccountNumber,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl BankConnectionsResourceAccountNumberDetailsIdentifierType {
    pub fn as_str(&self) -> &str {
        use BankConnectionsResourceAccountNumberDetailsIdentifierType::*;
        match self {
            AccountNumber => "account_number",
            TokenizedAccountNumber => "tokenized_account_number",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for BankConnectionsResourceAccountNumberDetailsIdentifierType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BankConnectionsResourceAccountNumberDetailsIdentifierType::*;
        match s {
            "account_number" => Ok(AccountNumber),
            "tokenized_account_number" => Ok(TokenizedAccountNumber),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "BankConnectionsResourceAccountNumberDetailsIdentifierType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for BankConnectionsResourceAccountNumberDetailsIdentifierType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BankConnectionsResourceAccountNumberDetailsIdentifierType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for BankConnectionsResourceAccountNumberDetailsIdentifierType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for BankConnectionsResourceAccountNumberDetailsIdentifierType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<BankConnectionsResourceAccountNumberDetailsIdentifierType>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            BankConnectionsResourceAccountNumberDetailsIdentifierType::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    BankConnectionsResourceAccountNumberDetailsIdentifierType
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BankConnectionsResourceAccountNumberDetailsIdentifierType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Whether the account number is currently active and usable for transactions.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum BankConnectionsResourceAccountNumberDetailsStatus {
    Deactivated,
    Transactable,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl BankConnectionsResourceAccountNumberDetailsStatus {
    pub fn as_str(&self) -> &str {
        use BankConnectionsResourceAccountNumberDetailsStatus::*;
        match self {
            Deactivated => "deactivated",
            Transactable => "transactable",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for BankConnectionsResourceAccountNumberDetailsStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BankConnectionsResourceAccountNumberDetailsStatus::*;
        match s {
            "deactivated" => Ok(Deactivated),
            "transactable" => Ok(Transactable),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "BankConnectionsResourceAccountNumberDetailsStatus"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for BankConnectionsResourceAccountNumberDetailsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BankConnectionsResourceAccountNumberDetailsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for BankConnectionsResourceAccountNumberDetailsStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for BankConnectionsResourceAccountNumberDetailsStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<BankConnectionsResourceAccountNumberDetailsStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            BankConnectionsResourceAccountNumberDetailsStatus::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(BankConnectionsResourceAccountNumberDetailsStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BankConnectionsResourceAccountNumberDetailsStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The payment networks that the account number can be used for.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum BankConnectionsResourceAccountNumberDetailsSupportedNetworks {
    Ach,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl BankConnectionsResourceAccountNumberDetailsSupportedNetworks {
    pub fn as_str(&self) -> &str {
        use BankConnectionsResourceAccountNumberDetailsSupportedNetworks::*;
        match self {
            Ach => "ach",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for BankConnectionsResourceAccountNumberDetailsSupportedNetworks {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BankConnectionsResourceAccountNumberDetailsSupportedNetworks::*;
        match s {
            "ach" => Ok(Ach),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "BankConnectionsResourceAccountNumberDetailsSupportedNetworks"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for BankConnectionsResourceAccountNumberDetailsSupportedNetworks {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BankConnectionsResourceAccountNumberDetailsSupportedNetworks {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for BankConnectionsResourceAccountNumberDetailsSupportedNetworks {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for BankConnectionsResourceAccountNumberDetailsSupportedNetworks {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<BankConnectionsResourceAccountNumberDetailsSupportedNetworks>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            BankConnectionsResourceAccountNumberDetailsSupportedNetworks::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    BankConnectionsResourceAccountNumberDetailsSupportedNetworks
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BankConnectionsResourceAccountNumberDetailsSupportedNetworks {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}

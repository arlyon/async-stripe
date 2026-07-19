#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BankConnectionsResourceAccountNumberDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BankConnectionsResourceAccountNumberDetails").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: BankConnectionsResourceAccountNumberDetailsBuilder {
                    expected_expiry_date: Deserialize::default(),
                    identifier_type: Deserialize::default(),
                    status: Deserialize::default(),
                    supported_networks: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "expected_expiry_date" => {
                    Deserialize::begin(&mut self.builder.expected_expiry_date)
                }
                "identifier_type" => Deserialize::begin(&mut self.builder.identifier_type),
                "status" => Deserialize::begin(&mut self.builder.status),
                "supported_networks" => Deserialize::begin(&mut self.builder.supported_networks),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(expected_expiry_date),
                Some(identifier_type),
                Some(status),
                Some(supported_networks),
            ) = (
                self.builder.expected_expiry_date,
                self.builder.identifier_type.take(),
                self.builder.status.take(),
                self.builder.supported_networks.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(BankConnectionsResourceAccountNumberDetails {
                expected_expiry_date,
                identifier_type,
                status,
                supported_networks,
            });
            Ok(())
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

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for BankConnectionsResourceAccountNumberDetailsIdentifierType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BankConnectionsResourceAccountNumberDetailsIdentifierType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(BankConnectionsResourceAccountNumberDetailsIdentifierType))
            .finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for BankConnectionsResourceAccountNumberDetailsIdentifierType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<BankConnectionsResourceAccountNumberDetailsIdentifierType>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            BankConnectionsResourceAccountNumberDetailsIdentifierType::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
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

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for BankConnectionsResourceAccountNumberDetailsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BankConnectionsResourceAccountNumberDetailsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(BankConnectionsResourceAccountNumberDetailsStatus))
            .finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for BankConnectionsResourceAccountNumberDetailsStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<BankConnectionsResourceAccountNumberDetailsStatus> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            BankConnectionsResourceAccountNumberDetailsStatus::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}
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

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for BankConnectionsResourceAccountNumberDetailsSupportedNetworks {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BankConnectionsResourceAccountNumberDetailsSupportedNetworks {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(BankConnectionsResourceAccountNumberDetailsSupportedNetworks))
            .finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for BankConnectionsResourceAccountNumberDetailsSupportedNetworks {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<BankConnectionsResourceAccountNumberDetailsSupportedNetworks>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            BankConnectionsResourceAccountNumberDetailsSupportedNetworks::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BankConnectionsResourceAccountNumberDetailsSupportedNetworks {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}

#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InboundTransfersPaymentMethodDetailsUsBankAccount {
    /// Account holder type: individual or company.
    pub account_holder_type:
        Option<InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType>,
    /// Account type: checkings or savings. Defaults to checking if omitted.
    pub account_type: Option<InboundTransfersPaymentMethodDetailsUsBankAccountAccountType>,
    /// Name of the bank associated with the bank account.
    pub bank_name: Option<String>,
    /// Uniquely identifies this particular bank account.
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    /// Last four digits of the bank account number.
    pub last4: Option<String>,
    /// ID of the mandate used to make this payment.
    pub mandate: Option<stripe_types::Expandable<stripe_shared::Mandate>>,
    /// The network rails used.
    /// See the [docs](https://docs.stripe.com/treasury/money-movement/timelines) to learn more about money movement timelines for each network type.
    pub network: InboundTransfersPaymentMethodDetailsUsBankAccountNetwork,
    /// Routing number of the bank account.
    pub routing_number: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InboundTransfersPaymentMethodDetailsUsBankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InboundTransfersPaymentMethodDetailsUsBankAccount").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InboundTransfersPaymentMethodDetailsUsBankAccountBuilder {
    account_holder_type:
        Option<Option<InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType>>,
    account_type: Option<Option<InboundTransfersPaymentMethodDetailsUsBankAccountAccountType>>,
    bank_name: Option<Option<String>>,
    fingerprint: Option<Option<String>>,
    last4: Option<Option<String>>,
    mandate: Option<Option<stripe_types::Expandable<stripe_shared::Mandate>>>,
    network: Option<InboundTransfersPaymentMethodDetailsUsBankAccountNetwork>,
    routing_number: Option<Option<String>>,
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

    impl Deserialize for InboundTransfersPaymentMethodDetailsUsBankAccount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InboundTransfersPaymentMethodDetailsUsBankAccount>,
        builder: InboundTransfersPaymentMethodDetailsUsBankAccountBuilder,
    }

    impl Visitor for Place<InboundTransfersPaymentMethodDetailsUsBankAccount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InboundTransfersPaymentMethodDetailsUsBankAccountBuilder {
                    account_holder_type: Deserialize::default(),
                    account_type: Deserialize::default(),
                    bank_name: Deserialize::default(),
                    fingerprint: Deserialize::default(),
                    last4: Deserialize::default(),
                    mandate: Deserialize::default(),
                    network: Deserialize::default(),
                    routing_number: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account_holder_type" => Deserialize::begin(&mut self.builder.account_holder_type),
                "account_type" => Deserialize::begin(&mut self.builder.account_type),
                "bank_name" => Deserialize::begin(&mut self.builder.bank_name),
                "fingerprint" => Deserialize::begin(&mut self.builder.fingerprint),
                "last4" => Deserialize::begin(&mut self.builder.last4),
                "mandate" => Deserialize::begin(&mut self.builder.mandate),
                "network" => Deserialize::begin(&mut self.builder.network),
                "routing_number" => Deserialize::begin(&mut self.builder.routing_number),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(account_holder_type),
                Some(account_type),
                Some(bank_name),
                Some(fingerprint),
                Some(last4),
                Some(mandate),
                Some(network),
                Some(routing_number),
            ) = (
                self.builder.account_holder_type.take(),
                self.builder.account_type.take(),
                self.builder.bank_name.take(),
                self.builder.fingerprint.take(),
                self.builder.last4.take(),
                self.builder.mandate.take(),
                self.builder.network.take(),
                self.builder.routing_number.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(InboundTransfersPaymentMethodDetailsUsBankAccount {
                account_holder_type,
                account_type,
                bank_name,
                fingerprint,
                last4,
                mandate,
                network,
                routing_number,
            });
            Ok(())
        }
    }
};
/// Account holder type: individual or company.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType {
    Company,
    Individual,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType {
    pub fn as_str(&self) -> &str {
        use InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType::*;
        match self {
            Company => "company",
            Individual => "individual",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType::*;
        match s {
            "company" => Ok(Company),
            "individual" => Ok(Individual),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(
            InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType
        ))
        .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Account type: checkings or savings. Defaults to checking if omitted.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum InboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    Checking,
    Savings,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl InboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    pub fn as_str(&self) -> &str {
        use InboundTransfersPaymentMethodDetailsUsBankAccountAccountType::*;
        match self {
            Checking => "checking",
            Savings => "savings",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for InboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InboundTransfersPaymentMethodDetailsUsBankAccountAccountType::*;
        match s {
            "checking" => Ok(Checking),
            "savings" => Ok(Savings),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "InboundTransfersPaymentMethodDetailsUsBankAccountAccountType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for InboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for InboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(InboundTransfersPaymentMethodDetailsUsBankAccountAccountType))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for InboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for InboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<InboundTransfersPaymentMethodDetailsUsBankAccountAccountType>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            InboundTransfersPaymentMethodDetailsUsBankAccountAccountType::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for InboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The network rails used.
/// See the [docs](https://docs.stripe.com/treasury/money-movement/timelines) to learn more about money movement timelines for each network type.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum InboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    Ach,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl InboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    pub fn as_str(&self) -> &str {
        use InboundTransfersPaymentMethodDetailsUsBankAccountNetwork::*;
        match self {
            Ach => "ach",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for InboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InboundTransfersPaymentMethodDetailsUsBankAccountNetwork::*;
        match s {
            "ach" => Ok(Ach),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "InboundTransfersPaymentMethodDetailsUsBankAccountNetwork"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for InboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for InboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(InboundTransfersPaymentMethodDetailsUsBankAccountNetwork))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for InboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for InboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<InboundTransfersPaymentMethodDetailsUsBankAccountNetwork>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            InboundTransfersPaymentMethodDetailsUsBankAccountNetwork::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for InboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}

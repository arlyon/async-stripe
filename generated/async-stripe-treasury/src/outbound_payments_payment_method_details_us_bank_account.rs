#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct OutboundPaymentsPaymentMethodDetailsUsBankAccount {
    /// Account holder type: individual or company.
    pub account_holder_type:
        Option<OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType>,
    /// Account type: checkings or savings. Defaults to checking if omitted.
    pub account_type: Option<OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType>,
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
    /// See the [docs](https://stripe.com/docs/treasury/money-movement/timelines) to learn more about money movement timelines for each network type.
    pub network: OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork,
    /// Routing number of the bank account.
    pub routing_number: Option<String>,
}
#[doc(hidden)]
pub struct OutboundPaymentsPaymentMethodDetailsUsBankAccountBuilder {
    account_holder_type:
        Option<Option<OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType>>,
    account_type: Option<Option<OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType>>,
    bank_name: Option<Option<String>>,
    fingerprint: Option<Option<String>>,
    last4: Option<Option<String>>,
    mandate: Option<Option<stripe_types::Expandable<stripe_shared::Mandate>>>,
    network: Option<OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork>,
    routing_number: Option<Option<String>>,
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

    impl Deserialize for OutboundPaymentsPaymentMethodDetailsUsBankAccount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<OutboundPaymentsPaymentMethodDetailsUsBankAccount>,
        builder: OutboundPaymentsPaymentMethodDetailsUsBankAccountBuilder,
    }

    impl Visitor for Place<OutboundPaymentsPaymentMethodDetailsUsBankAccount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: OutboundPaymentsPaymentMethodDetailsUsBankAccountBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for OutboundPaymentsPaymentMethodDetailsUsBankAccountBuilder {
        type Out = OutboundPaymentsPaymentMethodDetailsUsBankAccount;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account_holder_type" => Deserialize::begin(&mut self.account_holder_type),
                "account_type" => Deserialize::begin(&mut self.account_type),
                "bank_name" => Deserialize::begin(&mut self.bank_name),
                "fingerprint" => Deserialize::begin(&mut self.fingerprint),
                "last4" => Deserialize::begin(&mut self.last4),
                "mandate" => Deserialize::begin(&mut self.mandate),
                "network" => Deserialize::begin(&mut self.network),
                "routing_number" => Deserialize::begin(&mut self.routing_number),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                account_holder_type: Deserialize::default(),
                account_type: Deserialize::default(),
                bank_name: Deserialize::default(),
                fingerprint: Deserialize::default(),
                last4: Deserialize::default(),
                mandate: Deserialize::default(),
                network: Deserialize::default(),
                routing_number: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
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
                self.account_holder_type.take(),
                self.account_type.take(),
                self.bank_name.take(),
                self.fingerprint.take(),
                self.last4.take(),
                self.mandate.take(),
                self.network.take(),
                self.routing_number.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                account_holder_type,
                account_type,
                bank_name,
                fingerprint,
                last4,
                mandate,
                network,
                routing_number,
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

    impl ObjectDeser for OutboundPaymentsPaymentMethodDetailsUsBankAccount {
        type Builder = OutboundPaymentsPaymentMethodDetailsUsBankAccountBuilder;
    }

    impl FromValueOpt for OutboundPaymentsPaymentMethodDetailsUsBankAccount {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = OutboundPaymentsPaymentMethodDetailsUsBankAccountBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "account_holder_type" => b.account_holder_type = FromValueOpt::from_value(v),
                    "account_type" => b.account_type = FromValueOpt::from_value(v),
                    "bank_name" => b.bank_name = FromValueOpt::from_value(v),
                    "fingerprint" => b.fingerprint = FromValueOpt::from_value(v),
                    "last4" => b.last4 = FromValueOpt::from_value(v),
                    "mandate" => b.mandate = FromValueOpt::from_value(v),
                    "network" => b.network = FromValueOpt::from_value(v),
                    "routing_number" => b.routing_number = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Account holder type: individual or company.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType {
    Company,
    Individual,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType {
    pub fn as_str(&self) -> &str {
        use OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType::*;
        match self {
            Company => "company",
            Individual => "individual",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType::*;
        match s {
            "company" => Ok(Company),
            "individual" => Ok(Individual),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType
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
pub enum OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType {
    Checking,
    Savings,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType {
    pub fn as_str(&self) -> &str {
        use OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType::*;
        match self {
            Checking => "checking",
            Savings => "savings",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType::*;
        match s {
            "checking" => Ok(Checking),
            "savings" => Ok(Savings),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The network rails used.
/// See the [docs](https://stripe.com/docs/treasury/money-movement/timelines) to learn more about money movement timelines for each network type.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork {
    Ach,
    UsDomesticWire,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork {
    pub fn as_str(&self) -> &str {
        use OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork::*;
        match self {
            Ach => "ach",
            UsDomesticWire => "us_domestic_wire",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork::*;
        match s {
            "ach" => Ok(Ach),
            "us_domestic_wire" => Ok(UsDomesticWire),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}

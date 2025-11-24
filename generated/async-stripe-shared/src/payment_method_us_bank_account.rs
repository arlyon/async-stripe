#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodUsBankAccount {
    /// Account holder type: individual or company.
    pub account_holder_type: Option<PaymentMethodUsBankAccountAccountHolderType>,
    /// Account type: checkings or savings. Defaults to checking if omitted.
    pub account_type: Option<PaymentMethodUsBankAccountAccountType>,
    /// The name of the bank.
    pub bank_name: Option<String>,
    /// The ID of the Financial Connections Account used to create the payment method.
    pub financial_connections_account: Option<String>,
    /// Uniquely identifies this particular bank account.
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    /// Last four digits of the bank account number.
    pub last4: Option<String>,
    /// Contains information about US bank account networks that can be used.
    pub networks: Option<stripe_shared::UsBankAccountNetworks>,
    /// Routing number of the bank account.
    pub routing_number: Option<String>,
    /// Contains information about the future reusability of this PaymentMethod.
    pub status_details: Option<stripe_shared::PaymentMethodUsBankAccountStatusDetails>,
}
#[doc(hidden)]
pub struct PaymentMethodUsBankAccountBuilder {
    account_holder_type: Option<Option<PaymentMethodUsBankAccountAccountHolderType>>,
    account_type: Option<Option<PaymentMethodUsBankAccountAccountType>>,
    bank_name: Option<Option<String>>,
    financial_connections_account: Option<Option<String>>,
    fingerprint: Option<Option<String>>,
    last4: Option<Option<String>>,
    networks: Option<Option<stripe_shared::UsBankAccountNetworks>>,
    routing_number: Option<Option<String>>,
    status_details: Option<Option<stripe_shared::PaymentMethodUsBankAccountStatusDetails>>,
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

    impl Deserialize for PaymentMethodUsBankAccount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodUsBankAccount>,
        builder: PaymentMethodUsBankAccountBuilder,
    }

    impl Visitor for Place<PaymentMethodUsBankAccount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodUsBankAccountBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodUsBankAccountBuilder {
        type Out = PaymentMethodUsBankAccount;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account_holder_type" => Deserialize::begin(&mut self.account_holder_type),
                "account_type" => Deserialize::begin(&mut self.account_type),
                "bank_name" => Deserialize::begin(&mut self.bank_name),
                "financial_connections_account" => {
                    Deserialize::begin(&mut self.financial_connections_account)
                }
                "fingerprint" => Deserialize::begin(&mut self.fingerprint),
                "last4" => Deserialize::begin(&mut self.last4),
                "networks" => Deserialize::begin(&mut self.networks),
                "routing_number" => Deserialize::begin(&mut self.routing_number),
                "status_details" => Deserialize::begin(&mut self.status_details),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                account_holder_type: Deserialize::default(),
                account_type: Deserialize::default(),
                bank_name: Deserialize::default(),
                financial_connections_account: Deserialize::default(),
                fingerprint: Deserialize::default(),
                last4: Deserialize::default(),
                networks: Deserialize::default(),
                routing_number: Deserialize::default(),
                status_details: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(account_holder_type),
                Some(account_type),
                Some(bank_name),
                Some(financial_connections_account),
                Some(fingerprint),
                Some(last4),
                Some(networks),
                Some(routing_number),
                Some(status_details),
            ) = (
                self.account_holder_type.take(),
                self.account_type.take(),
                self.bank_name.take(),
                self.financial_connections_account.take(),
                self.fingerprint.take(),
                self.last4.take(),
                self.networks.take(),
                self.routing_number.take(),
                self.status_details.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                account_holder_type,
                account_type,
                bank_name,
                financial_connections_account,
                fingerprint,
                last4,
                networks,
                routing_number,
                status_details,
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

    impl ObjectDeser for PaymentMethodUsBankAccount {
        type Builder = PaymentMethodUsBankAccountBuilder;
    }

    impl FromValueOpt for PaymentMethodUsBankAccount {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodUsBankAccountBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "account_holder_type" => b.account_holder_type = FromValueOpt::from_value(v),
                    "account_type" => b.account_type = FromValueOpt::from_value(v),
                    "bank_name" => b.bank_name = FromValueOpt::from_value(v),
                    "financial_connections_account" => {
                        b.financial_connections_account = FromValueOpt::from_value(v)
                    }
                    "fingerprint" => b.fingerprint = FromValueOpt::from_value(v),
                    "last4" => b.last4 = FromValueOpt::from_value(v),
                    "networks" => b.networks = FromValueOpt::from_value(v),
                    "routing_number" => b.routing_number = FromValueOpt::from_value(v),
                    "status_details" => b.status_details = FromValueOpt::from_value(v),
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
pub enum PaymentMethodUsBankAccountAccountHolderType {
    Company,
    Individual,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentMethodUsBankAccountAccountHolderType {
    pub fn as_str(&self) -> &str {
        use PaymentMethodUsBankAccountAccountHolderType::*;
        match self {
            Company => "company",
            Individual => "individual",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentMethodUsBankAccountAccountHolderType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodUsBankAccountAccountHolderType::*;
        match s {
            "company" => Ok(Company),
            "individual" => Ok(Individual),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentMethodUsBankAccountAccountHolderType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentMethodUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodUsBankAccountAccountHolderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentMethodUsBankAccountAccountHolderType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentMethodUsBankAccountAccountHolderType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(PaymentMethodUsBankAccountAccountHolderType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentMethodUsBankAccountAccountHolderType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodUsBankAccountAccountHolderType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Account type: checkings or savings. Defaults to checking if omitted.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentMethodUsBankAccountAccountType {
    Checking,
    Savings,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentMethodUsBankAccountAccountType {
    pub fn as_str(&self) -> &str {
        use PaymentMethodUsBankAccountAccountType::*;
        match self {
            Checking => "checking",
            Savings => "savings",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentMethodUsBankAccountAccountType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodUsBankAccountAccountType::*;
        match s {
            "checking" => Ok(Checking),
            "savings" => Ok(Savings),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentMethodUsBankAccountAccountType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentMethodUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodUsBankAccountAccountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentMethodUsBankAccountAccountType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentMethodUsBankAccountAccountType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentMethodUsBankAccountAccountType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentMethodUsBankAccountAccountType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodUsBankAccountAccountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}

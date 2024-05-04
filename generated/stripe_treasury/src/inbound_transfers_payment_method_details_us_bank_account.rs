#[derive(Clone, Debug)]
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
    /// The US bank account network used to debit funds.
    pub network: InboundTransfersPaymentMethodDetailsUsBankAccountNetwork,
    /// Routing number of the bank account.
    pub routing_number: Option<String>,
}
#[doc(hidden)]
pub struct InboundTransfersPaymentMethodDetailsUsBankAccountBuilder {
    account_holder_type:
        Option<Option<InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType>>,
    account_type: Option<Option<InboundTransfersPaymentMethodDetailsUsBankAccountAccountType>>,
    bank_name: Option<Option<String>>,
    fingerprint: Option<Option<String>>,
    last4: Option<Option<String>>,
    network: Option<InboundTransfersPaymentMethodDetailsUsBankAccountNetwork>,
    routing_number: Option<Option<String>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: InboundTransfersPaymentMethodDetailsUsBankAccountBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InboundTransfersPaymentMethodDetailsUsBankAccountBuilder {
        type Out = InboundTransfersPaymentMethodDetailsUsBankAccount;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account_holder_type" => Deserialize::begin(&mut self.account_holder_type),
                "account_type" => Deserialize::begin(&mut self.account_type),
                "bank_name" => Deserialize::begin(&mut self.bank_name),
                "fingerprint" => Deserialize::begin(&mut self.fingerprint),
                "last4" => Deserialize::begin(&mut self.last4),
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
                network: Deserialize::default(),
                routing_number: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                account_holder_type: self.account_holder_type?,
                account_type: self.account_type?,
                bank_name: self.bank_name.take()?,
                fingerprint: self.fingerprint.take()?,
                last4: self.last4.take()?,
                network: self.network?,
                routing_number: self.routing_number.take()?,
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

    impl ObjectDeser for InboundTransfersPaymentMethodDetailsUsBankAccount {
        type Builder = InboundTransfersPaymentMethodDetailsUsBankAccountBuilder;
    }

    impl FromValueOpt for InboundTransfersPaymentMethodDetailsUsBankAccount {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InboundTransfersPaymentMethodDetailsUsBankAccountBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "account_holder_type" => {
                        b.account_holder_type = Some(FromValueOpt::from_value(v)?)
                    }
                    "account_type" => b.account_type = Some(FromValueOpt::from_value(v)?),
                    "bank_name" => b.bank_name = Some(FromValueOpt::from_value(v)?),
                    "fingerprint" => b.fingerprint = Some(FromValueOpt::from_value(v)?),
                    "last4" => b.last4 = Some(FromValueOpt::from_value(v)?),
                    "network" => b.network = Some(FromValueOpt::from_value(v)?),
                    "routing_number" => b.routing_number = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Account holder type: individual or company.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType {
    Company,
    Individual,
}
impl InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType {
    pub fn as_str(self) -> &'static str {
        use InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType::*;
        match self {
            Company => "company",
            Individual => "individual",
        }
    }
}

impl std::str::FromStr for InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType::*;
        match s {
            "company" => Ok(Company),
            "individual" => Ok(Individual),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType"))
    }
}
/// Account type: checkings or savings. Defaults to checking if omitted.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    Checking,
    Savings,
}
impl InboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    pub fn as_str(self) -> &'static str {
        use InboundTransfersPaymentMethodDetailsUsBankAccountAccountType::*;
        match self {
            Checking => "checking",
            Savings => "savings",
        }
    }
}

impl std::str::FromStr for InboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InboundTransfersPaymentMethodDetailsUsBankAccountAccountType::*;
        match s {
            "checking" => Ok(Checking),
            "savings" => Ok(Savings),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for InboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for InboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<InboundTransfersPaymentMethodDetailsUsBankAccountAccountType>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            InboundTransfersPaymentMethodDetailsUsBankAccountAccountType::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    InboundTransfersPaymentMethodDetailsUsBankAccountAccountType
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for InboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for InboundTransfersPaymentMethodDetailsUsBankAccountAccountType",
            )
        })
    }
}
/// The US bank account network used to debit funds.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    Ach,
}
impl InboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    pub fn as_str(self) -> &'static str {
        use InboundTransfersPaymentMethodDetailsUsBankAccountNetwork::*;
        match self {
            Ach => "ach",
        }
    }
}

impl std::str::FromStr for InboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InboundTransfersPaymentMethodDetailsUsBankAccountNetwork::*;
        match s {
            "ach" => Ok(Ach),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for InboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for InboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<InboundTransfersPaymentMethodDetailsUsBankAccountNetwork>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            InboundTransfersPaymentMethodDetailsUsBankAccountNetwork::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    InboundTransfersPaymentMethodDetailsUsBankAccountNetwork
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for InboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for InboundTransfersPaymentMethodDetailsUsBankAccountNetwork",
            )
        })
    }
}
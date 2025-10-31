/// Details of the US Bank Account used for this payment attempt.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetails {
    pub account_holder_type: Option<
        PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetailsAccountHolderType,
    >,
    pub account_type: Option<
        PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetailsAccountType,
    >,
    /// Name of the bank associated with the bank account.
    pub bank_name: Option<String>,
    /// Uniquely identifies this particular bank account.
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    /// Last four digits of the bank account number.
    pub last4: Option<String>,
    /// ID of the mandate used to make this payment.
    pub mandate: Option<stripe_types::Expandable<stripe_shared::Mandate>>,
    /// Reference number to locate ACH payments with customer’s bank.
    pub payment_reference: Option<String>,
    /// Routing number of the bank account.
    pub routing_number: Option<String>,
}
#[doc(hidden)]
pub struct PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetailsBuilder {
    account_holder_type: Option<Option<PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetailsAccountHolderType>>,
account_type: Option<Option<PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetailsAccountType>>,
bank_name: Option<Option<String>>,
fingerprint: Option<Option<String>>,
last4: Option<Option<String>>,
mandate: Option<Option<stripe_types::Expandable<stripe_shared::Mandate>>>,
payment_reference: Option<Option<String>>,
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

    impl Deserialize for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<
            PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetails,
        >,
        builder: PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetailsBuilder,
    }

    impl Visitor for Place<PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetailsBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder
        for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetailsBuilder
    {
        type Out = PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account_holder_type" => Deserialize::begin(&mut self.account_holder_type),
                "account_type" => Deserialize::begin(&mut self.account_type),
                "bank_name" => Deserialize::begin(&mut self.bank_name),
                "fingerprint" => Deserialize::begin(&mut self.fingerprint),
                "last4" => Deserialize::begin(&mut self.last4),
                "mandate" => Deserialize::begin(&mut self.mandate),
                "payment_reference" => Deserialize::begin(&mut self.payment_reference),
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
                payment_reference: Deserialize::default(),
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
                Some(payment_reference),
                Some(routing_number),
            ) = (
                self.account_holder_type,
                self.account_type,
                self.bank_name.take(),
                self.fingerprint.take(),
                self.last4.take(),
                self.mandate.take(),
                self.payment_reference.take(),
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
                payment_reference,
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

    impl ObjectDeser for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetails {
        type Builder =
            PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetailsBuilder;
    }

    impl FromValueOpt for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "account_holder_type" => b.account_holder_type = FromValueOpt::from_value(v),
                    "account_type" => b.account_type = FromValueOpt::from_value(v),
                    "bank_name" => b.bank_name = FromValueOpt::from_value(v),
                    "fingerprint" => b.fingerprint = FromValueOpt::from_value(v),
                    "last4" => b.last4 = FromValueOpt::from_value(v),
                    "mandate" => b.mandate = FromValueOpt::from_value(v),
                    "payment_reference" => b.payment_reference = FromValueOpt::from_value(v),
                    "routing_number" => b.routing_number = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetailsAccountHolderType
{
    Company,
    Individual,
}
impl PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetailsAccountHolderType {
    pub fn as_str(self) -> &'static str {
        use PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetailsAccountHolderType::*;
        match self {
            Company => "company",
            Individual => "individual",
        }
    }
}

impl std::str::FromStr
    for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetailsAccountHolderType
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetailsAccountHolderType::*;
        match s {
            "company" => Ok(Company),
            "individual" => Ok(Individual),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetailsAccountHolderType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetailsAccountHolderType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize
    for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetailsAccountHolderType
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize
    for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetailsAccountHolderType
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<
        PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetailsAccountHolderType,
    >
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetailsAccountHolderType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetailsAccountHolderType
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetailsAccountHolderType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetailsAccountHolderType"))
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetailsAccountType {
    Checking,
    Savings,
}
impl PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetailsAccountType {
    pub fn as_str(self) -> &'static str {
        use PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetailsAccountType::*;
        match self {
            Checking => "checking",
            Savings => "savings",
        }
    }
}

impl std::str::FromStr
    for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetailsAccountType
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetailsAccountType::*;
        match s {
            "checking" => Ok(Checking),
            "savings" => Ok(Savings),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetailsAccountType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetailsAccountType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize
    for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetailsAccountType
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize
    for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetailsAccountType
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<
        PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetailsAccountType,
    >
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetailsAccountType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetailsAccountType
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetailsAccountType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodUsBankAccountDetailsAccountType"))
    }
}

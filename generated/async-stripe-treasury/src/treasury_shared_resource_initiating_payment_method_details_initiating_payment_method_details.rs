#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails {
    /// Set when `type` is `balance`.
    pub balance: Option<
        TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance,
    >,
    pub billing_details: stripe_treasury::TreasurySharedResourceBillingDetails,
    pub financial_account: Option<stripe_treasury::ReceivedPaymentMethodDetailsFinancialAccount>,
    /// Set when `type` is `issuing_card`.
    /// This is an [Issuing Card](https://api.stripe.com#issuing_cards) ID.
    pub issuing_card: Option<String>,
    /// Polymorphic type matching the originating money movement's source.
    /// This can be an external account, a Stripe balance, or a FinancialAccount.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_:
        TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType,
    pub us_bank_account:
        Option<stripe_treasury::TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccount>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(
            "TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails",
        )
        .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBuilder {
    balance: Option<Option<TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance>>,
billing_details: Option<stripe_treasury::TreasurySharedResourceBillingDetails>,
financial_account: Option<Option<stripe_treasury::ReceivedPaymentMethodDetailsFinancialAccount>>,
issuing_card: Option<Option<String>>,
type_: Option<TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType>,
us_bank_account: Option<Option<stripe_treasury::TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccount>>,

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

    impl Deserialize
        for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails
    {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
    out: &'a mut Option<TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails>,
    builder: TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBuilder,
}

    impl Visitor
        for Place<
            TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails,
        >
    {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBuilder { balance: Deserialize::default(),
billing_details: Deserialize::default(),
financial_account: Deserialize::default(),
issuing_card: Deserialize::default(),
type_: Deserialize::default(),
us_bank_account: Deserialize::default(),
 },
        }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "balance" => Deserialize::begin(&mut self.builder.balance),
                "billing_details" => Deserialize::begin(&mut self.builder.billing_details),
                "financial_account" => Deserialize::begin(&mut self.builder.financial_account),
                "issuing_card" => Deserialize::begin(&mut self.builder.issuing_card),
                "type" => Deserialize::begin(&mut self.builder.type_),
                "us_bank_account" => Deserialize::begin(&mut self.builder.us_bank_account),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(balance),
                Some(billing_details),
                Some(financial_account),
                Some(issuing_card),
                Some(type_),
                Some(us_bank_account),
            ) = (
                self.builder.balance.take(),
                self.builder.billing_details.take(),
                self.builder.financial_account.take(),
                self.builder.issuing_card.take(),
                self.builder.type_.take(),
                self.builder.us_bank_account.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails { balance,billing_details,financial_account,issuing_card,type_,us_bank_account });
            Ok(())
        }
    }
};
/// Set when `type` is `balance`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance {
    Payments,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance {
    pub fn as_str(&self) -> &str {
        use TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance::*;
        match self {
            Payments => "payments",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance::*;
        match s {
            "payments" => Ok(Payments),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display
    for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug
    for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize
    for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize
    for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance
{
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<
        TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance,
    >
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Polymorphic type matching the originating money movement's source.
/// This can be an external account, a Stripe balance, or a FinancialAccount.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType {
    Balance,
    FinancialAccount,
    IssuingCard,
    Stripe,
    UsBankAccount,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType {
    pub fn as_str(&self) -> &str {
        use TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType::*;
        match self {
            Balance => "balance",
            FinancialAccount => "financial_account",
            IssuingCard => "issuing_card",
            Stripe => "stripe",
            UsBankAccount => "us_bank_account",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType::*;
        match s {
            "balance" => Ok(Balance),
            "financial_account" => Ok(FinancialAccount),
            "issuing_card" => Ok(IssuingCard),
            "stripe" => Ok(Stripe),
            "us_bank_account" => Ok(UsBankAccount),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display
    for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug
    for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(
            TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType
        ))
        .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize
    for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize
    for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType
{
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<
        TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType,
    >
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails {
    /// Set when `type` is `balance`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<
        TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance,
    >,
    pub billing_details: stripe_treasury::TreasurySharedResourceBillingDetails,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_account: Option<stripe_treasury::ReceivedPaymentMethodDetailsFinancialAccount>,
    /// Set when `type` is `issuing_card`.
    /// This is an [Issuing Card](https://stripe.com/docs/api#issuing_cards) ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_card: Option<String>,
    /// Polymorphic type matching the originating money movement's source.
    /// This can be an external account, a Stripe balance, or a FinancialAccount.
    #[serde(rename = "type")]
    pub type_:
        TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account:
        Option<stripe_treasury::TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccount>,
}
/// Set when `type` is `balance`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance {
    Payments,
}
impl TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance {
    pub fn as_str(self) -> &'static str {
        use TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance::*;
        match self {
            Payments => "payments",
        }
    }
}

impl std::str::FromStr
    for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance::*;
        match s {
            "payments" => Ok(Payments),
            _ => Err(()),
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

impl std::fmt::Debug
    for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
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
impl<'de> serde::Deserialize<'de>
    for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance"))
    }
}
/// Polymorphic type matching the originating money movement's source.
/// This can be an external account, a Stripe balance, or a FinancialAccount.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType {
    Balance,
    FinancialAccount,
    IssuingCard,
    Stripe,
    UsBankAccount,
}
impl TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType {
    pub fn as_str(self) -> &'static str {
        use TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType::*;
        match self {
            Balance => "balance",
            FinancialAccount => "financial_account",
            IssuingCard => "issuing_card",
            Stripe => "stripe",
            UsBankAccount => "us_bank_account",
        }
    }
}

impl std::str::FromStr
    for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType::*;
        match s {
            "balance" => Ok(Balance),
            "financial_account" => Ok(FinancialAccount),
            "issuing_card" => Ok(IssuingCard),
            "stripe" => Ok(Stripe),
            "us_bank_account" => Ok(UsBankAccount),
            _ => Err(()),
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

impl std::fmt::Debug
    for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
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
impl<'de> serde::Deserialize<'de>
    for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType"))
    }
}

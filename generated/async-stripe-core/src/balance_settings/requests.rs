use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveForMyAccountBalanceSettingsBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveForMyAccountBalanceSettingsBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves balance settings for a given connected account.
///  Related guide: <a href="/connect/authentication">Making API calls for connected accounts</a>
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveForMyAccountBalanceSettings {
    inner: RetrieveForMyAccountBalanceSettingsBuilder,
}
impl RetrieveForMyAccountBalanceSettings {
    /// Construct a new `RetrieveForMyAccountBalanceSettings`.
    pub fn new() -> Self {
        Self { inner: RetrieveForMyAccountBalanceSettingsBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl Default for RetrieveForMyAccountBalanceSettings {
    fn default() -> Self {
        Self::new()
    }
}
impl RetrieveForMyAccountBalanceSettings {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for RetrieveForMyAccountBalanceSettings {
    type Output = stripe_core::BalanceSettings;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/balance_settings").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct UpdateBalanceSettingsBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payments: Option<UpdateBalanceSettingsPayments>,
}
impl UpdateBalanceSettingsBuilder {
    fn new() -> Self {
        Self { expand: None, payments: None }
    }
}
/// Settings that apply to the [Payments Balance](https://docs.stripe.com/api/balance).
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateBalanceSettingsPayments {
    /// A Boolean indicating whether Stripe should try to reclaim negative balances from an attached bank account.
    /// For details, see [Understanding Connect Account Balances](/connect/account-balances).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debit_negative_balances: Option<bool>,
    /// Settings specific to the account's payouts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts: Option<UpdateBalanceSettingsPaymentsPayouts>,
    /// Settings related to the account's balance settlement timing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_timing: Option<UpdateBalanceSettingsPaymentsSettlementTiming>,
}
impl UpdateBalanceSettingsPayments {
    pub fn new() -> Self {
        Self { debit_negative_balances: None, payouts: None, settlement_timing: None }
    }
}
impl Default for UpdateBalanceSettingsPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// Settings specific to the account's payouts.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateBalanceSettingsPaymentsPayouts {
    /// The minimum balance amount to retain per currency after automatic payouts.
    /// Only funds that exceed these amounts are paid out.
    /// Learn more about the [minimum balances for automatic payouts](/payouts/minimum-balances-for-automatic-payouts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_balance_by_currency: Option<std::collections::HashMap<String, i64>>,
    /// Details on when funds from charges are available, and when they are paid out to an external account.
    /// For details, see our [Setting Bank and Debit Card Payouts](/connect/bank-transfers#payout-information) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<UpdateBalanceSettingsPaymentsPayoutsSchedule>,
    /// The text that appears on the bank account statement for payouts.
    /// If not set, this defaults to the platform's bank descriptor as set in the Dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
}
impl UpdateBalanceSettingsPaymentsPayouts {
    pub fn new() -> Self {
        Self { minimum_balance_by_currency: None, schedule: None, statement_descriptor: None }
    }
}
impl Default for UpdateBalanceSettingsPaymentsPayouts {
    fn default() -> Self {
        Self::new()
    }
}
/// Details on when funds from charges are available, and when they are paid out to an external account.
/// For details, see our [Setting Bank and Debit Card Payouts](/connect/bank-transfers#payout-information) documentation.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateBalanceSettingsPaymentsPayoutsSchedule {
    /// How frequently available funds are paid out.
    /// One of: `daily`, `manual`, `weekly`, or `monthly`.
    /// Default is `daily`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<UpdateBalanceSettingsPaymentsPayoutsScheduleInterval>,
    /// The days of the month when available funds are paid out, specified as an array of numbers between 1--31.
    /// Payouts nominally scheduled between the 29th and 31st of the month are instead sent on the last day of a shorter month.
    /// Required and applicable only if `interval` is `monthly`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_payout_days: Option<Vec<u32>>,
    /// The days of the week when available funds are paid out, specified as an array, e.g., [`monday`, `tuesday`].
    /// Required and applicable only if `interval` is `weekly`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_payout_days:
        Option<Vec<UpdateBalanceSettingsPaymentsPayoutsScheduleWeeklyPayoutDays>>,
}
impl UpdateBalanceSettingsPaymentsPayoutsSchedule {
    pub fn new() -> Self {
        Self { interval: None, monthly_payout_days: None, weekly_payout_days: None }
    }
}
impl Default for UpdateBalanceSettingsPaymentsPayoutsSchedule {
    fn default() -> Self {
        Self::new()
    }
}
/// How frequently available funds are paid out.
/// One of: `daily`, `manual`, `weekly`, or `monthly`.
/// Default is `daily`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateBalanceSettingsPaymentsPayoutsScheduleInterval {
    Daily,
    Manual,
    Monthly,
    Weekly,
}
impl UpdateBalanceSettingsPaymentsPayoutsScheduleInterval {
    pub fn as_str(self) -> &'static str {
        use UpdateBalanceSettingsPaymentsPayoutsScheduleInterval::*;
        match self {
            Daily => "daily",
            Manual => "manual",
            Monthly => "monthly",
            Weekly => "weekly",
        }
    }
}

impl std::str::FromStr for UpdateBalanceSettingsPaymentsPayoutsScheduleInterval {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateBalanceSettingsPaymentsPayoutsScheduleInterval::*;
        match s {
            "daily" => Ok(Daily),
            "manual" => Ok(Manual),
            "monthly" => Ok(Monthly),
            "weekly" => Ok(Weekly),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateBalanceSettingsPaymentsPayoutsScheduleInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateBalanceSettingsPaymentsPayoutsScheduleInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateBalanceSettingsPaymentsPayoutsScheduleInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateBalanceSettingsPaymentsPayoutsScheduleInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateBalanceSettingsPaymentsPayoutsScheduleInterval",
            )
        })
    }
}
/// The days of the week when available funds are paid out, specified as an array, e.g., [`monday`, `tuesday`].
/// Required and applicable only if `interval` is `weekly`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateBalanceSettingsPaymentsPayoutsScheduleWeeklyPayoutDays {
    Friday,
    Monday,
    Thursday,
    Tuesday,
    Wednesday,
}
impl UpdateBalanceSettingsPaymentsPayoutsScheduleWeeklyPayoutDays {
    pub fn as_str(self) -> &'static str {
        use UpdateBalanceSettingsPaymentsPayoutsScheduleWeeklyPayoutDays::*;
        match self {
            Friday => "friday",
            Monday => "monday",
            Thursday => "thursday",
            Tuesday => "tuesday",
            Wednesday => "wednesday",
        }
    }
}

impl std::str::FromStr for UpdateBalanceSettingsPaymentsPayoutsScheduleWeeklyPayoutDays {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateBalanceSettingsPaymentsPayoutsScheduleWeeklyPayoutDays::*;
        match s {
            "friday" => Ok(Friday),
            "monday" => Ok(Monday),
            "thursday" => Ok(Thursday),
            "tuesday" => Ok(Tuesday),
            "wednesday" => Ok(Wednesday),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateBalanceSettingsPaymentsPayoutsScheduleWeeklyPayoutDays {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateBalanceSettingsPaymentsPayoutsScheduleWeeklyPayoutDays {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateBalanceSettingsPaymentsPayoutsScheduleWeeklyPayoutDays {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateBalanceSettingsPaymentsPayoutsScheduleWeeklyPayoutDays {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateBalanceSettingsPaymentsPayoutsScheduleWeeklyPayoutDays",
            )
        })
    }
}
/// Settings related to the account's balance settlement timing.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateBalanceSettingsPaymentsSettlementTiming {
    /// Change `delay_days` for this account, which determines the number of days charge funds are held before becoming available.
    /// The maximum value is 31.
    /// Passing an empty string to `delay_days_override` will return `delay_days` to the default, which is the lowest available value for the account.
    /// [Learn more about controlling delay days](/connect/manage-payout-schedule).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay_days_override: Option<u32>,
}
impl UpdateBalanceSettingsPaymentsSettlementTiming {
    pub fn new() -> Self {
        Self { delay_days_override: None }
    }
}
impl Default for UpdateBalanceSettingsPaymentsSettlementTiming {
    fn default() -> Self {
        Self::new()
    }
}
/// Updates balance settings for a given connected account.
///  Related guide: <a href="/connect/authentication">Making API calls for connected accounts</a>
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateBalanceSettings {
    inner: UpdateBalanceSettingsBuilder,
}
impl UpdateBalanceSettings {
    /// Construct a new `UpdateBalanceSettings`.
    pub fn new() -> Self {
        Self { inner: UpdateBalanceSettingsBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Settings that apply to the [Payments Balance](https://docs.stripe.com/api/balance).
    pub fn payments(mut self, payments: impl Into<UpdateBalanceSettingsPayments>) -> Self {
        self.inner.payments = Some(payments.into());
        self
    }
}
impl Default for UpdateBalanceSettings {
    fn default() -> Self {
        Self::new()
    }
}
impl UpdateBalanceSettings {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for UpdateBalanceSettings {
    type Output = stripe_core::BalanceSettings;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/balance_settings").form(&self.inner)
    }
}

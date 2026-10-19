use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct RetrieveForMyAccountBalanceSettingsBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveForMyAccountBalanceSettingsBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveForMyAccountBalanceSettingsBuilder").finish_non_exhaustive()
    }
}
impl RetrieveForMyAccountBalanceSettingsBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves balance settings for a given connected account.
///  Related guide: <a href="/connect/authentication">Making API calls for connected accounts</a>
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RetrieveForMyAccountBalanceSettings {
    inner: RetrieveForMyAccountBalanceSettingsBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveForMyAccountBalanceSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveForMyAccountBalanceSettings").finish_non_exhaustive()
    }
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
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct UpdateBalanceSettingsBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payments: Option<UpdateBalanceSettingsPayments>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateBalanceSettingsBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateBalanceSettingsBuilder").finish_non_exhaustive()
    }
}
impl UpdateBalanceSettingsBuilder {
    fn new() -> Self {
        Self { expand: None, payments: None }
    }
}
/// Settings that apply to the [Payments Balance](https://docs.stripe.com/api/balance).
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateBalanceSettingsPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateBalanceSettingsPayments").finish_non_exhaustive()
    }
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
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateBalanceSettingsPaymentsPayouts {
    /// Configures per-currency rules for automatically transferring funds from the payments balance to a FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_transfer_rules_by_currency: Option<
        std::collections::HashMap<
            String,
            Vec<UpdateBalanceSettingsPaymentsPayoutsAutomaticTransferRulesByCurrency>,
        >,
    >,
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateBalanceSettingsPaymentsPayouts {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateBalanceSettingsPaymentsPayouts").finish_non_exhaustive()
    }
}
impl UpdateBalanceSettingsPaymentsPayouts {
    pub fn new() -> Self {
        Self {
            automatic_transfer_rules_by_currency: None,
            minimum_balance_by_currency: None,
            schedule: None,
            statement_descriptor: None,
        }
    }
}
impl Default for UpdateBalanceSettingsPaymentsPayouts {
    fn default() -> Self {
        Self::new()
    }
}
/// Configures per-currency rules for automatically transferring funds from the payments balance to a FinancialAccount.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateBalanceSettingsPaymentsPayoutsAutomaticTransferRulesByCurrency {
    /// The ID of the FinancialAccount that funds will be transferred to during automatic transfers.
    pub payout_method: String,
    /// The maximum amount in minor units to transfer to the FinancialAccount.
    /// Required and only applicable when `type` is `transfer_up_to_amount`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_up_to_amount: Option<i64>,
    /// The type of automatic transfer rule.
    #[serde(rename = "type")]
    pub type_: stripe_shared::BalanceSettingsResourceAutomaticTransferRuleType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateBalanceSettingsPaymentsPayoutsAutomaticTransferRulesByCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateBalanceSettingsPaymentsPayoutsAutomaticTransferRulesByCurrency")
            .finish_non_exhaustive()
    }
}
impl UpdateBalanceSettingsPaymentsPayoutsAutomaticTransferRulesByCurrency {
    pub fn new(
        payout_method: impl Into<String>,
        type_: impl Into<stripe_shared::BalanceSettingsResourceAutomaticTransferRuleType>,
    ) -> Self {
        Self {
            payout_method: payout_method.into(),
            transfer_up_to_amount: None,
            type_: type_.into(),
        }
    }
}
/// Details on when funds from charges are available, and when they are paid out to an external account.
/// For details, see our [Setting Bank and Debit Card Payouts](/connect/bank-transfers#payout-information) documentation.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateBalanceSettingsPaymentsPayoutsSchedule {
    /// How frequently available funds are paid out.
    /// One of: `daily`, `manual`, `weekly`, or `monthly`.
    /// Default is `daily`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<stripe_shared::Interval>,
    /// The days of the month when available funds are paid out, specified as an array of numbers between 1--31.
    /// Payouts nominally scheduled between the 29th and 31st of the month are instead sent on the last day of a shorter month.
    /// Required and applicable only if `interval` is `monthly`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_payout_days: Option<Vec<u32>>,
    /// The days of the week when available funds are paid out, specified as an array, e.g., [`monday`, `tuesday`].
    /// Required and applicable only if `interval` is `weekly`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_payout_days: Option<Vec<stripe_shared::WeeklyPayoutDays>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateBalanceSettingsPaymentsPayoutsSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateBalanceSettingsPaymentsPayoutsSchedule").finish_non_exhaustive()
    }
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
/// Settings related to the account's balance settlement timing.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateBalanceSettingsPaymentsSettlementTiming {
    /// Change `delay_days` for this account, which determines the number of days charge funds are held before becoming available.
    /// The maximum value is 31.
    /// Passing an empty string to `delay_days_override` will return `delay_days` to the default, which is the lowest available value for the account.
    /// [Learn more about controlling delay days](/connect/manage-payout-schedule).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay_days_override: Option<u32>,
    /// Customized start of day configuration for automatic payouts to group and send payments in local timezones with a customized day starting time.
    /// For details, see our [Customized start of day](/connect/customized-start-of-day) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_of_day: Option<UpdateBalanceSettingsPaymentsSettlementTimingStartOfDay>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateBalanceSettingsPaymentsSettlementTiming {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateBalanceSettingsPaymentsSettlementTiming").finish_non_exhaustive()
    }
}
impl UpdateBalanceSettingsPaymentsSettlementTiming {
    pub fn new() -> Self {
        Self { delay_days_override: None, start_of_day: None }
    }
}
impl Default for UpdateBalanceSettingsPaymentsSettlementTiming {
    fn default() -> Self {
        Self::new()
    }
}
/// Customized start of day configuration for automatic payouts to group and send payments in local timezones with a customized day starting time.
/// For details, see our [Customized start of day](/connect/customized-start-of-day) documentation.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateBalanceSettingsPaymentsSettlementTimingStartOfDay {
    /// Hour at which the customized start of day begins according to the given timezone.
    /// Must be a [supported customized start of day hour](/connect/customized-start-of-day#available-timezones-and-cutoffs).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hour: Option<i64>,
    /// Minutes at which the customized start of day begins according to the given timezone.
    /// Must be either 0 or 30.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minutes: Option<i64>,
    /// Timezone for the customized start of day.
    /// Must be a [supported customized start of day timezone](/connect/customized-start-of-day#available-timezones-and-cutoffs).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateBalanceSettingsPaymentsSettlementTimingStartOfDay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateBalanceSettingsPaymentsSettlementTimingStartOfDay")
            .finish_non_exhaustive()
    }
}
impl UpdateBalanceSettingsPaymentsSettlementTimingStartOfDay {
    pub fn new() -> Self {
        Self { hour: None, minutes: None, timezone: None }
    }
}
impl Default for UpdateBalanceSettingsPaymentsSettlementTimingStartOfDay {
    fn default() -> Self {
        Self::new()
    }
}
/// Updates balance settings for a given connected account.
///  Related guide: <a href="/connect/authentication">Making API calls for connected accounts</a>
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateBalanceSettings {
    inner: UpdateBalanceSettingsBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateBalanceSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateBalanceSettings").finish_non_exhaustive()
    }
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

/// The type of automatic transfer rule.
pub type UpdateBalanceSettingsPaymentsPayoutsAutomaticTransferRulesByCurrencyType =
    stripe_shared::BalanceSettingsResourceAutomaticTransferRuleType;
/// How frequently available funds are paid out.
/// One of: `daily`, `manual`, `weekly`, or `monthly`.
/// Default is `daily`.
pub type UpdateBalanceSettingsPaymentsPayoutsScheduleInterval = stripe_shared::Interval;
/// The days of the week when available funds are paid out, specified as an array, e.g., [`monday`, `tuesday`].
/// Required and applicable only if `interval` is `weekly`.
pub type UpdateBalanceSettingsPaymentsPayoutsScheduleWeeklyPayoutDays =
    stripe_shared::WeeklyPayoutDays;

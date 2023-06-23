use stripe::{Client, Response};

impl stripe_core::cash_balance::CashBalance {
    /// Retrieves a customer’s cash balance.
    pub fn retrieve(
        client: &Client,
        customer: &stripe_core::customer::CustomerId,
        params: RetrieveCashBalance,
    ) -> Response<stripe_core::cash_balance::CashBalance> {
        client
            .get_query(&format!("/customers/{customer}/cash_balance", customer = customer), params)
    }
    /// Changes the settings on a customer’s cash balance.
    pub fn update(
        client: &Client,
        customer: &stripe_core::customer::CustomerId,
        params: UpdateCashBalance,
    ) -> Response<stripe_core::cash_balance::CashBalance> {
        client.send_form(
            &format!("/customers/{customer}/cash_balance", customer = customer),
            params,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveCashBalance<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveCashBalance<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateCashBalance<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A hash of settings for this cash balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<UpdateCashBalanceSettings>,
}
impl<'a> UpdateCashBalance<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// A hash of settings for this cash balance.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateCashBalanceSettings {
    /// Controls how funds transferred by the customer are applied to payment intents and invoices.
    ///
    /// Valid options are `automatic` or `manual`.
    /// For more information about these reconciliation modes, see [Reconciliation](https://stripe.com/docs/payments/customer-balance/reconciliation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconciliation_mode: Option<UpdateCashBalanceSettingsReconciliationMode>,
}
impl UpdateCashBalanceSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Controls how funds transferred by the customer are applied to payment intents and invoices.
///
/// Valid options are `automatic` or `manual`.
/// For more information about these reconciliation modes, see [Reconciliation](https://stripe.com/docs/payments/customer-balance/reconciliation).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateCashBalanceSettingsReconciliationMode {
    Automatic,
    Manual,
}

impl UpdateCashBalanceSettingsReconciliationMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Manual => "manual",
        }
    }
}

impl AsRef<str> for UpdateCashBalanceSettingsReconciliationMode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateCashBalanceSettingsReconciliationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

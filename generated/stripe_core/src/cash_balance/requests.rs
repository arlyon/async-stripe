impl stripe_core::cash_balance::CashBalance {
    /// Retrieves a customer’s cash balance.
    pub fn retrieve(
        client: &stripe::Client,
        customer: &stripe_core::customer::CustomerId,
        params: RetrieveCashBalance,
    ) -> stripe::Response<stripe_core::cash_balance::CashBalance> {
        client
            .get_query(&format!("/customers/{customer}/cash_balance", customer = customer), params)
    }
    /// Changes the settings on a customer’s cash balance.
    pub fn update(
        client: &stripe::Client,
        customer: &stripe_core::customer::CustomerId,
        params: UpdateCashBalance,
    ) -> stripe::Response<stripe_core::cash_balance::CashBalance> {
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for UpdateCashBalanceSettingsReconciliationMode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "automatic" => Ok(Self::Automatic),
            "manual" => Ok(Self::Manual),

            _ => Err(()),
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
impl serde::Serialize for UpdateCashBalanceSettingsReconciliationMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

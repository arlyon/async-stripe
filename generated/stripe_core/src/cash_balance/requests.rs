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
impl<'a> RetrieveCashBalance<'a> {
    /// Retrieves a customer’s cash balance.
    pub fn send(
        &self,
        client: &stripe::Client,
        customer: &stripe_types::customer::CustomerId,
    ) -> stripe::Response<stripe_types::CashBalance> {
        client.get_query(&format!("/customers/{customer}/cash_balance", customer = customer), self)
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
    /// Valid options are `automatic`, `manual`, or `merchant_default`.
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
/// Valid options are `automatic`, `manual`, or `merchant_default`.
/// For more information about these reconciliation modes, see [Reconciliation](https://stripe.com/docs/payments/customer-balance/reconciliation).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateCashBalanceSettingsReconciliationMode {
    Automatic,
    Manual,
    MerchantDefault,
}

impl UpdateCashBalanceSettingsReconciliationMode {
    pub fn as_str(self) -> &'static str {
        use UpdateCashBalanceSettingsReconciliationMode::*;
        match self {
            Automatic => "automatic",
            Manual => "manual",
            MerchantDefault => "merchant_default",
        }
    }
}

impl std::str::FromStr for UpdateCashBalanceSettingsReconciliationMode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateCashBalanceSettingsReconciliationMode::*;
        match s {
            "automatic" => Ok(Automatic),
            "manual" => Ok(Manual),
            "merchant_default" => Ok(MerchantDefault),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateCashBalanceSettingsReconciliationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl<'a> UpdateCashBalance<'a> {
    /// Changes the settings on a customer’s cash balance.
    pub fn send(
        &self,
        client: &stripe::Client,
        customer: &stripe_types::customer::CustomerId,
    ) -> stripe::Response<stripe_types::CashBalance> {
        client.send_form(
            &format!("/customers/{customer}/cash_balance", customer = customer),
            self,
            http_types::Method::Post,
        )
    }
}

use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct RetrieveCashBalanceBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveCashBalanceBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a customer’s cash balance.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct RetrieveCashBalance {
    inner: RetrieveCashBalanceBuilder,
    customer: stripe_shared::CustomerId,
}
impl RetrieveCashBalance {
    /// Construct a new `RetrieveCashBalance`.
    pub fn new(customer: impl Into<stripe_shared::CustomerId>) -> Self {
        Self { customer: customer.into(), inner: RetrieveCashBalanceBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveCashBalance {
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

impl StripeRequest for RetrieveCashBalance {
    type Output = stripe_shared::CashBalance;

    fn build(&self) -> RequestBuilder {
        let customer = &self.customer;
        RequestBuilder::new(StripeMethod::Get, format!("/customers/{customer}/cash_balance"))
            .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct UpdateCashBalanceBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<UpdateCashBalanceSettings>,
}
impl UpdateCashBalanceBuilder {
    fn new() -> Self {
        Self { expand: None, settings: None }
    }
}
/// A hash of settings for this cash balance.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct UpdateCashBalanceSettings {
    /// Controls how funds transferred by the customer are applied to payment intents and invoices.
    /// Valid options are `automatic`, `manual`, or `merchant_default`.
    /// For more information about these reconciliation modes, see [Reconciliation](https://stripe.com/docs/payments/customer-balance/reconciliation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconciliation_mode: Option<UpdateCashBalanceSettingsReconciliationMode>,
}
impl UpdateCashBalanceSettings {
    pub fn new() -> Self {
        Self { reconciliation_mode: None }
    }
}
impl Default for UpdateCashBalanceSettings {
    fn default() -> Self {
        Self::new()
    }
}
/// Controls how funds transferred by the customer are applied to payment intents and invoices.
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
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateCashBalanceSettingsReconciliationMode::*;
        match s {
            "automatic" => Ok(Automatic),
            "manual" => Ok(Manual),
            "merchant_default" => Ok(MerchantDefault),
            _ => Err(stripe_types::StripeParseError),
        }
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
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateCashBalanceSettingsReconciliationMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateCashBalanceSettingsReconciliationMode",
            )
        })
    }
}
/// Changes the settings on a customer’s cash balance.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct UpdateCashBalance {
    inner: UpdateCashBalanceBuilder,
    customer: stripe_shared::CustomerId,
}
impl UpdateCashBalance {
    /// Construct a new `UpdateCashBalance`.
    pub fn new(customer: impl Into<stripe_shared::CustomerId>) -> Self {
        Self { customer: customer.into(), inner: UpdateCashBalanceBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// A hash of settings for this cash balance.
    pub fn settings(mut self, settings: impl Into<UpdateCashBalanceSettings>) -> Self {
        self.inner.settings = Some(settings.into());
        self
    }
}
impl UpdateCashBalance {
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

impl StripeRequest for UpdateCashBalance {
    type Output = stripe_shared::CashBalance;

    fn build(&self) -> RequestBuilder {
        let customer = &self.customer;
        RequestBuilder::new(StripeMethod::Post, format!("/customers/{customer}/cash_balance"))
            .form(&self.inner)
    }
}

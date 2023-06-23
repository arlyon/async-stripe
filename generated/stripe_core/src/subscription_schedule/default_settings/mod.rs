#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct DefaultSettings {
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the application owner's Stripe account during this phase of the schedule.
    pub application_fee_percent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax:
        Option<stripe_core::subscription_schedule::default_settings::automatic_tax::AutomaticTax>,
    /// Possible values are `phase_start` or `automatic`.
    ///
    /// If `phase_start` then billing cycle anchor of the subscription is set to the start of the phase when entering the phase.
    /// If `automatic` then the billing cycle anchor is automatically modified as needed when entering the phase.
    /// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
    pub billing_cycle_anchor: DefaultSettingsBillingCycleAnchor,
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    pub billing_thresholds:
        Option<stripe_core::subscription::billing_thresholds::BillingThresholds>,
    /// Either `charge_automatically`, or `send_invoice`.
    ///
    /// When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
    pub collection_method: Option<DefaultSettingsCollectionMethod>,
    /// ID of the default payment method for the subscription schedule.
    ///
    /// If not set, invoices will use the default payment method in the customer's invoice settings.
    pub default_payment_method:
        Option<stripe_types::Expandable<stripe_core::payment_method::PaymentMethod>>,
    /// Subscription description, meant to be displayable to the customer.
    ///
    /// Use this field to optionally store an explanation of the subscription.
    pub description: Option<String>,
    /// The subscription schedule's default invoice settings.
    pub invoice_settings:
        Option<stripe_core::subscription_schedule::invoice_settings::InvoiceSettings>,
    /// The account (if any) the charge was made on behalf of for charges associated with the schedule's subscription.
    ///
    /// See the Connect documentation for details.
    pub on_behalf_of: Option<stripe_types::Expandable<stripe_core::account::Account>>,
    /// The account (if any) the associated subscription's payments will be attributed to for tax reporting, and where funds from each payment will be transferred to for each of the subscription's invoices.
    pub transfer_data: Option<stripe_core::subscription::transfer_data::TransferData>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for DefaultSettings {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Possible values are `phase_start` or `automatic`.
///
/// If `phase_start` then billing cycle anchor of the subscription is set to the start of the phase when entering the phase.
/// If `automatic` then the billing cycle anchor is automatically modified as needed when entering the phase.
/// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum DefaultSettingsBillingCycleAnchor {
    Automatic,
    PhaseStart,
}

impl DefaultSettingsBillingCycleAnchor {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::PhaseStart => "phase_start",
        }
    }
}

impl AsRef<str> for DefaultSettingsBillingCycleAnchor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DefaultSettingsBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Either `charge_automatically`, or `send_invoice`.
///
/// When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer.
/// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum DefaultSettingsCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}

impl DefaultSettingsCollectionMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ChargeAutomatically => "charge_automatically",
            Self::SendInvoice => "send_invoice",
        }
    }
}

impl AsRef<str> for DefaultSettingsCollectionMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DefaultSettingsCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
pub mod automatic_tax;
pub use automatic_tax::AutomaticTax;

/// A phase describes the plans, coupon, and trialing status of a subscription for a predefined time period.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Phase {
    /// A list of prices and quantities that will generate invoice items appended to the next invoice for this phase.
    pub add_invoice_items:
        Vec<stripe_types::subscription_schedule::add_invoice_item::AddInvoiceItem>,
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the application owner's Stripe account during this phase of the schedule.
    pub application_fee_percent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax:
        Option<stripe_types::subscription_schedule::phase::automatic_tax::AutomaticTax>,
    /// Possible values are `phase_start` or `automatic`.
    ///
    /// If `phase_start` then billing cycle anchor of the subscription is set to the start of the phase when entering the phase.
    /// If `automatic` then the billing cycle anchor is automatically modified as needed when entering the phase.
    /// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
    pub billing_cycle_anchor: Option<PhaseBillingCycleAnchor>,
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    pub billing_thresholds:
        Option<stripe_types::subscription::billing_thresholds::BillingThresholds>,
    /// Either `charge_automatically`, or `send_invoice`.
    ///
    /// When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
    pub collection_method: Option<PhaseCollectionMethod>,
    /// ID of the coupon to use during this phase of the subscription schedule.
    pub coupon: Option<stripe_types::Expandable<stripe_types::coupon::Coupon>>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// ID of the default payment method for the subscription schedule.
    ///
    /// It must belong to the customer associated with the subscription schedule.
    /// If not set, invoices will use the default payment method in the customer's invoice settings.
    pub default_payment_method:
        Option<stripe_types::Expandable<stripe_types::payment_method::PaymentMethod>>,
    /// The default tax rates to apply to the subscription during this phase of the subscription schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<stripe_types::tax_rate::TaxRate>>,
    /// Subscription description, meant to be displayable to the customer.
    ///
    /// Use this field to optionally store an explanation of the subscription.
    pub description: Option<String>,
    /// The end of this phase of the subscription schedule.
    pub end_date: stripe_types::Timestamp,
    /// The invoice settings applicable during this phase.
    pub invoice_settings:
        Option<stripe_types::subscription_schedule::invoice_settings::InvoiceSettings>,
    /// Subscription items to configure the subscription to during this phase of the subscription schedule.
    pub items: Vec<stripe_types::subscription_schedule::phase_item::PhaseItem>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to a phase.
    ///
    /// Metadata on a schedule's phase will update the underlying subscription's `metadata` when the phase is entered.
    /// Updating the underlying subscription's `metadata` directly will not affect the current phase's `metadata`.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The account (if any) the charge was made on behalf of for charges associated with the schedule's subscription.
    ///
    /// See the Connect documentation for details.
    pub on_behalf_of: Option<stripe_types::Expandable<stripe_types::account::Account>>,
    /// If the subscription schedule will prorate when transitioning to this phase.
    ///
    /// Possible values are `create_prorations` and `none`.
    pub proration_behavior: PhaseProrationBehavior,
    /// The start of this phase of the subscription schedule.
    pub start_date: stripe_types::Timestamp,
    /// The account (if any) the associated subscription's payments will be attributed to for tax reporting, and where funds from each payment will be transferred to for each of the subscription's invoices.
    pub transfer_data: Option<stripe_types::subscription::transfer_data::TransferData>,
    /// When the trial ends within the phase.
    pub trial_end: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Phase {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Possible values are `phase_start` or `automatic`.
///
/// If `phase_start` then billing cycle anchor of the subscription is set to the start of the phase when entering the phase.
/// If `automatic` then the billing cycle anchor is automatically modified as needed when entering the phase.
/// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PhaseBillingCycleAnchor {
    Automatic,
    PhaseStart,
}

impl PhaseBillingCycleAnchor {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::PhaseStart => "phase_start",
        }
    }
}

impl std::str::FromStr for PhaseBillingCycleAnchor {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "automatic" => Ok(Self::Automatic),
            "phase_start" => Ok(Self::PhaseStart),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for PhaseBillingCycleAnchor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PhaseBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for PhaseBillingCycleAnchor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PhaseBillingCycleAnchor {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for PhaseBillingCycleAnchor"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PhaseBillingCycleAnchor {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PhaseBillingCycleAnchor> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PhaseBillingCycleAnchor::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Either `charge_automatically`, or `send_invoice`.
///
/// When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer.
/// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PhaseCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}

impl PhaseCollectionMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ChargeAutomatically => "charge_automatically",
            Self::SendInvoice => "send_invoice",
        }
    }
}

impl std::str::FromStr for PhaseCollectionMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "charge_automatically" => Ok(Self::ChargeAutomatically),
            "send_invoice" => Ok(Self::SendInvoice),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for PhaseCollectionMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PhaseCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for PhaseCollectionMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PhaseCollectionMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for PhaseCollectionMethod"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PhaseCollectionMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PhaseCollectionMethod> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PhaseCollectionMethod::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// If the subscription schedule will prorate when transitioning to this phase.
///
/// Possible values are `create_prorations` and `none`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PhaseProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}

impl PhaseProrationBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AlwaysInvoice => "always_invoice",
            Self::CreateProrations => "create_prorations",
            Self::None => "none",
        }
    }
}

impl std::str::FromStr for PhaseProrationBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "always_invoice" => Ok(Self::AlwaysInvoice),
            "create_prorations" => Ok(Self::CreateProrations),
            "none" => Ok(Self::None),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for PhaseProrationBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PhaseProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for PhaseProrationBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PhaseProrationBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for PhaseProrationBehavior"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PhaseProrationBehavior {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PhaseProrationBehavior> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PhaseProrationBehavior::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
pub mod automatic_tax;
pub use automatic_tax::AutomaticTax;

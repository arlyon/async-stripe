/// A phase describes the plans, coupon, and trialing status of a subscription for a predefined time period.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionSchedulePhaseConfiguration {
    /// A list of prices and quantities that will generate invoice items appended to the next invoice for this phase.
    pub add_invoice_items: Vec<stripe_types::SubscriptionScheduleAddInvoiceItem>,
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account during this phase of the schedule.
    pub application_fee_percent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<stripe_types::SchedulesPhaseAutomaticTax>,
    /// Possible values are `phase_start` or `automatic`.
    ///
    /// If `phase_start` then billing cycle anchor of the subscription is set to the start of the phase when entering the phase.
    /// If `automatic` then the billing cycle anchor is automatically modified as needed when entering the phase.
    /// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
    pub billing_cycle_anchor: Option<SubscriptionSchedulePhaseConfigurationBillingCycleAnchor>,
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    pub billing_thresholds: Option<stripe_types::SubscriptionBillingThresholds>,
    /// Either `charge_automatically`, or `send_invoice`.
    ///
    /// When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
    pub collection_method: Option<SubscriptionSchedulePhaseConfigurationCollectionMethod>,
    /// ID of the coupon to use during this phase of the subscription schedule.
    pub coupon: Option<stripe_types::Expandable<stripe_types::Coupon>>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// ID of the default payment method for the subscription schedule.
    ///
    /// It must belong to the customer associated with the subscription schedule.
    /// If not set, invoices will use the default payment method in the customer's invoice settings.
    pub default_payment_method: Option<stripe_types::Expandable<stripe_types::PaymentMethod>>,
    /// The default tax rates to apply to the subscription during this phase of the subscription schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<stripe_types::TaxRate>>,
    /// Subscription description, meant to be displayable to the customer.
    ///
    /// Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces and certain local payment methods UIs.
    pub description: Option<String>,
    /// The end of this phase of the subscription schedule.
    pub end_date: stripe_types::Timestamp,
    /// The invoice settings applicable during this phase.
    pub invoice_settings: Option<stripe_types::InvoiceSettingSubscriptionSchedulePhaseSetting>,
    /// Subscription items to configure the subscription to during this phase of the subscription schedule.
    pub items: Vec<stripe_types::SubscriptionScheduleConfigurationItem>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to a phase.
    ///
    /// Metadata on a schedule's phase will update the underlying subscription's `metadata` when the phase is entered.
    /// Updating the underlying subscription's `metadata` directly will not affect the current phase's `metadata`.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The account (if any) the charge was made on behalf of for charges associated with the schedule's subscription.
    ///
    /// See the Connect documentation for details.
    pub on_behalf_of: Option<stripe_types::Expandable<stripe_types::Account>>,
    /// If the subscription schedule will prorate when transitioning to this phase.
    ///
    /// Possible values are `create_prorations` and `none`.
    pub proration_behavior: SubscriptionSchedulePhaseConfigurationProrationBehavior,
    /// The start of this phase of the subscription schedule.
    pub start_date: stripe_types::Timestamp,
    /// The account (if any) the associated subscription's payments will be attributed to for tax reporting, and where funds from each payment will be transferred to for each of the subscription's invoices.
    pub transfer_data: Option<stripe_types::SubscriptionTransferData>,
    /// When the trial ends within the phase.
    pub trial_end: Option<stripe_types::Timestamp>,
}
/// Possible values are `phase_start` or `automatic`.
///
/// If `phase_start` then billing cycle anchor of the subscription is set to the start of the phase when entering the phase.
/// If `automatic` then the billing cycle anchor is automatically modified as needed when entering the phase.
/// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SubscriptionSchedulePhaseConfigurationBillingCycleAnchor {
    Automatic,
    PhaseStart,
}

impl SubscriptionSchedulePhaseConfigurationBillingCycleAnchor {
    pub fn as_str(self) -> &'static str {
        use SubscriptionSchedulePhaseConfigurationBillingCycleAnchor::*;
        match self {
            Automatic => "automatic",
            PhaseStart => "phase_start",
        }
    }
}

impl std::str::FromStr for SubscriptionSchedulePhaseConfigurationBillingCycleAnchor {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionSchedulePhaseConfigurationBillingCycleAnchor::*;
        match s {
            "automatic" => Ok(Automatic),
            "phase_start" => Ok(PhaseStart),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for SubscriptionSchedulePhaseConfigurationBillingCycleAnchor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionSchedulePhaseConfigurationBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SubscriptionSchedulePhaseConfigurationBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SubscriptionSchedulePhaseConfigurationBillingCycleAnchor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SubscriptionSchedulePhaseConfigurationBillingCycleAnchor {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for SubscriptionSchedulePhaseConfigurationBillingCycleAnchor",
            )
        })
    }
}
/// Either `charge_automatically`, or `send_invoice`.
///
/// When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer.
/// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SubscriptionSchedulePhaseConfigurationCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}

impl SubscriptionSchedulePhaseConfigurationCollectionMethod {
    pub fn as_str(self) -> &'static str {
        use SubscriptionSchedulePhaseConfigurationCollectionMethod::*;
        match self {
            ChargeAutomatically => "charge_automatically",
            SendInvoice => "send_invoice",
        }
    }
}

impl std::str::FromStr for SubscriptionSchedulePhaseConfigurationCollectionMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionSchedulePhaseConfigurationCollectionMethod::*;
        match s {
            "charge_automatically" => Ok(ChargeAutomatically),
            "send_invoice" => Ok(SendInvoice),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for SubscriptionSchedulePhaseConfigurationCollectionMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionSchedulePhaseConfigurationCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SubscriptionSchedulePhaseConfigurationCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SubscriptionSchedulePhaseConfigurationCollectionMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SubscriptionSchedulePhaseConfigurationCollectionMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for SubscriptionSchedulePhaseConfigurationCollectionMethod",
            )
        })
    }
}
/// If the subscription schedule will prorate when transitioning to this phase.
///
/// Possible values are `create_prorations` and `none`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SubscriptionSchedulePhaseConfigurationProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}

impl SubscriptionSchedulePhaseConfigurationProrationBehavior {
    pub fn as_str(self) -> &'static str {
        use SubscriptionSchedulePhaseConfigurationProrationBehavior::*;
        match self {
            AlwaysInvoice => "always_invoice",
            CreateProrations => "create_prorations",
            None => "none",
        }
    }
}

impl std::str::FromStr for SubscriptionSchedulePhaseConfigurationProrationBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionSchedulePhaseConfigurationProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for SubscriptionSchedulePhaseConfigurationProrationBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionSchedulePhaseConfigurationProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SubscriptionSchedulePhaseConfigurationProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SubscriptionSchedulePhaseConfigurationProrationBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SubscriptionSchedulePhaseConfigurationProrationBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for SubscriptionSchedulePhaseConfigurationProrationBehavior",
            )
        })
    }
}

/// A phase describes the plans, coupon, and trialing status of a subscription for a predefined time period.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionSchedulePhaseConfiguration {
    /// A list of prices and quantities that will generate invoice items appended to the next invoice for this phase.
    pub add_invoice_items: Vec<stripe_shared::SubscriptionScheduleAddInvoiceItem>,
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    /// This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account during this phase of the schedule.
    pub application_fee_percent: Option<f64>,
    pub automatic_tax: Option<stripe_shared::SchedulesPhaseAutomaticTax>,
    /// Possible values are `phase_start` or `automatic`.
    /// If `phase_start` then billing cycle anchor of the subscription is set to the start of the phase when entering the phase.
    /// If `automatic` then the billing cycle anchor is automatically modified as needed when entering the phase.
    /// For more information, see the billing cycle [documentation](https://docs.stripe.com/billing/subscriptions/billing-cycle).
    pub billing_cycle_anchor: Option<SubscriptionSchedulePhaseConfigurationBillingCycleAnchor>,
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    pub billing_thresholds: Option<stripe_shared::SubscriptionBillingThresholds>,
    /// Either `charge_automatically`, or `send_invoice`.
    /// When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
    pub collection_method: Option<SubscriptionSchedulePhaseConfigurationCollectionMethod>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// ID of the default payment method for the subscription schedule.
    /// It must belong to the customer associated with the subscription schedule.
    /// If not set, invoices will use the default payment method in the customer's invoice settings.
    pub default_payment_method: Option<stripe_types::Expandable<stripe_shared::PaymentMethod>>,
    /// The default tax rates to apply to the subscription during this phase of the subscription schedule.
    pub default_tax_rates: Option<Vec<stripe_shared::TaxRate>>,
    /// Subscription description, meant to be displayable to the customer.
    /// Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces and certain local payment methods UIs.
    pub description: Option<String>,
    /// The stackable discounts that will be applied to the subscription on this phase.
    /// Subscription item discounts are applied before subscription discounts.
    pub discounts: Vec<stripe_shared::StackableDiscountWithDiscountSettingsAndDiscountEnd>,
    /// The end of this phase of the subscription schedule.
    pub end_date: stripe_types::Timestamp,
    /// The invoice settings applicable during this phase.
    pub invoice_settings: Option<stripe_shared::InvoiceSettingSubscriptionSchedulePhaseSetting>,
    /// Subscription items to configure the subscription to during this phase of the subscription schedule.
    pub items: Vec<stripe_shared::SubscriptionScheduleConfigurationItem>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to a phase.
    /// Metadata on a schedule's phase will update the underlying subscription's `metadata` when the phase is entered.
    /// Updating the underlying subscription's `metadata` directly will not affect the current phase's `metadata`.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The account (if any) the charge was made on behalf of for charges associated with the schedule's subscription.
    /// See the Connect documentation for details.
    pub on_behalf_of: Option<stripe_types::Expandable<stripe_shared::Account>>,
    /// When transitioning phases, controls how prorations are handled (if any).
    /// Possible values are `create_prorations`, `none`, and `always_invoice`.
    pub proration_behavior: SubscriptionSchedulePhaseConfigurationProrationBehavior,
    /// The start of this phase of the subscription schedule.
    pub start_date: stripe_types::Timestamp,
    /// The account (if any) the associated subscription's payments will be attributed to for tax reporting, and where funds from each payment will be transferred to for each of the subscription's invoices.
    pub transfer_data: Option<stripe_shared::SubscriptionTransferData>,
    /// When the trial ends within the phase.
    pub trial_end: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionSchedulePhaseConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionSchedulePhaseConfiguration").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionSchedulePhaseConfigurationBuilder {
    add_invoice_items: Option<Vec<stripe_shared::SubscriptionScheduleAddInvoiceItem>>,
    application_fee_percent: Option<Option<f64>>,
    automatic_tax: Option<Option<stripe_shared::SchedulesPhaseAutomaticTax>>,
    billing_cycle_anchor: Option<Option<SubscriptionSchedulePhaseConfigurationBillingCycleAnchor>>,
    billing_thresholds: Option<Option<stripe_shared::SubscriptionBillingThresholds>>,
    collection_method: Option<Option<SubscriptionSchedulePhaseConfigurationCollectionMethod>>,
    currency: Option<stripe_types::Currency>,
    default_payment_method: Option<Option<stripe_types::Expandable<stripe_shared::PaymentMethod>>>,
    default_tax_rates: Option<Option<Vec<stripe_shared::TaxRate>>>,
    description: Option<Option<String>>,
    discounts: Option<Vec<stripe_shared::StackableDiscountWithDiscountSettingsAndDiscountEnd>>,
    end_date: Option<stripe_types::Timestamp>,
    invoice_settings: Option<Option<stripe_shared::InvoiceSettingSubscriptionSchedulePhaseSetting>>,
    items: Option<Vec<stripe_shared::SubscriptionScheduleConfigurationItem>>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    on_behalf_of: Option<Option<stripe_types::Expandable<stripe_shared::Account>>>,
    proration_behavior: Option<SubscriptionSchedulePhaseConfigurationProrationBehavior>,
    start_date: Option<stripe_types::Timestamp>,
    transfer_data: Option<Option<stripe_shared::SubscriptionTransferData>>,
    trial_end: Option<Option<stripe_types::Timestamp>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

    make_place!(Place);

    impl Deserialize for SubscriptionSchedulePhaseConfiguration {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionSchedulePhaseConfiguration>,
        builder: SubscriptionSchedulePhaseConfigurationBuilder,
    }

    impl Visitor for Place<SubscriptionSchedulePhaseConfiguration> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionSchedulePhaseConfigurationBuilder {
                    add_invoice_items: Deserialize::default(),
                    application_fee_percent: Deserialize::default(),
                    automatic_tax: Deserialize::default(),
                    billing_cycle_anchor: Deserialize::default(),
                    billing_thresholds: Deserialize::default(),
                    collection_method: Deserialize::default(),
                    currency: Deserialize::default(),
                    default_payment_method: Deserialize::default(),
                    default_tax_rates: Deserialize::default(),
                    description: Deserialize::default(),
                    discounts: Deserialize::default(),
                    end_date: Deserialize::default(),
                    invoice_settings: Deserialize::default(),
                    items: Deserialize::default(),
                    metadata: Deserialize::default(),
                    on_behalf_of: Deserialize::default(),
                    proration_behavior: Deserialize::default(),
                    start_date: Deserialize::default(),
                    transfer_data: Deserialize::default(),
                    trial_end: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "add_invoice_items" => Deserialize::begin(&mut self.builder.add_invoice_items),
                "application_fee_percent" => {
                    Deserialize::begin(&mut self.builder.application_fee_percent)
                }
                "automatic_tax" => Deserialize::begin(&mut self.builder.automatic_tax),
                "billing_cycle_anchor" => {
                    Deserialize::begin(&mut self.builder.billing_cycle_anchor)
                }
                "billing_thresholds" => Deserialize::begin(&mut self.builder.billing_thresholds),
                "collection_method" => Deserialize::begin(&mut self.builder.collection_method),
                "currency" => Deserialize::begin(&mut self.builder.currency),
                "default_payment_method" => {
                    Deserialize::begin(&mut self.builder.default_payment_method)
                }
                "default_tax_rates" => Deserialize::begin(&mut self.builder.default_tax_rates),
                "description" => Deserialize::begin(&mut self.builder.description),
                "discounts" => Deserialize::begin(&mut self.builder.discounts),
                "end_date" => Deserialize::begin(&mut self.builder.end_date),
                "invoice_settings" => Deserialize::begin(&mut self.builder.invoice_settings),
                "items" => Deserialize::begin(&mut self.builder.items),
                "metadata" => Deserialize::begin(&mut self.builder.metadata),
                "on_behalf_of" => Deserialize::begin(&mut self.builder.on_behalf_of),
                "proration_behavior" => Deserialize::begin(&mut self.builder.proration_behavior),
                "start_date" => Deserialize::begin(&mut self.builder.start_date),
                "transfer_data" => Deserialize::begin(&mut self.builder.transfer_data),
                "trial_end" => Deserialize::begin(&mut self.builder.trial_end),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(add_invoice_items),
                Some(application_fee_percent),
                Some(automatic_tax),
                Some(billing_cycle_anchor),
                Some(billing_thresholds),
                Some(collection_method),
                Some(currency),
                Some(default_payment_method),
                Some(default_tax_rates),
                Some(description),
                Some(discounts),
                Some(end_date),
                Some(invoice_settings),
                Some(items),
                Some(metadata),
                Some(on_behalf_of),
                Some(proration_behavior),
                Some(start_date),
                Some(transfer_data),
                Some(trial_end),
            ) = (
                self.builder.add_invoice_items.take(),
                self.builder.application_fee_percent,
                self.builder.automatic_tax.take(),
                self.builder.billing_cycle_anchor.take(),
                self.builder.billing_thresholds,
                self.builder.collection_method.take(),
                self.builder.currency.take(),
                self.builder.default_payment_method.take(),
                self.builder.default_tax_rates.take(),
                self.builder.description.take(),
                self.builder.discounts.take(),
                self.builder.end_date,
                self.builder.invoice_settings.take(),
                self.builder.items.take(),
                self.builder.metadata.take(),
                self.builder.on_behalf_of.take(),
                self.builder.proration_behavior.take(),
                self.builder.start_date,
                self.builder.transfer_data.take(),
                self.builder.trial_end,
            )
            else {
                return Ok(());
            };
            *self.out = Some(SubscriptionSchedulePhaseConfiguration {
                add_invoice_items,
                application_fee_percent,
                automatic_tax,
                billing_cycle_anchor,
                billing_thresholds,
                collection_method,
                currency,
                default_payment_method,
                default_tax_rates,
                description,
                discounts,
                end_date,
                invoice_settings,
                items,
                metadata,
                on_behalf_of,
                proration_behavior,
                start_date,
                transfer_data,
                trial_end,
            });
            Ok(())
        }
    }
};
/// Possible values are `phase_start` or `automatic`.
/// If `phase_start` then billing cycle anchor of the subscription is set to the start of the phase when entering the phase.
/// If `automatic` then the billing cycle anchor is automatically modified as needed when entering the phase.
/// For more information, see the billing cycle [documentation](https://docs.stripe.com/billing/subscriptions/billing-cycle).
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum SubscriptionSchedulePhaseConfigurationBillingCycleAnchor {
    Automatic,
    PhaseStart,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl SubscriptionSchedulePhaseConfigurationBillingCycleAnchor {
    pub fn as_str(&self) -> &str {
        use SubscriptionSchedulePhaseConfigurationBillingCycleAnchor::*;
        match self {
            Automatic => "automatic",
            PhaseStart => "phase_start",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for SubscriptionSchedulePhaseConfigurationBillingCycleAnchor {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionSchedulePhaseConfigurationBillingCycleAnchor::*;
        match s {
            "automatic" => Ok(Automatic),
            "phase_start" => Ok(PhaseStart),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "SubscriptionSchedulePhaseConfigurationBillingCycleAnchor"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for SubscriptionSchedulePhaseConfigurationBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for SubscriptionSchedulePhaseConfigurationBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionSchedulePhaseConfigurationBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(SubscriptionSchedulePhaseConfigurationBillingCycleAnchor))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SubscriptionSchedulePhaseConfigurationBillingCycleAnchor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for SubscriptionSchedulePhaseConfigurationBillingCycleAnchor {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<SubscriptionSchedulePhaseConfigurationBillingCycleAnchor>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            SubscriptionSchedulePhaseConfigurationBillingCycleAnchor::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for SubscriptionSchedulePhaseConfigurationBillingCycleAnchor {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Either `charge_automatically`, or `send_invoice`.
/// When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer.
/// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum SubscriptionSchedulePhaseConfigurationCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl SubscriptionSchedulePhaseConfigurationCollectionMethod {
    pub fn as_str(&self) -> &str {
        use SubscriptionSchedulePhaseConfigurationCollectionMethod::*;
        match self {
            ChargeAutomatically => "charge_automatically",
            SendInvoice => "send_invoice",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for SubscriptionSchedulePhaseConfigurationCollectionMethod {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionSchedulePhaseConfigurationCollectionMethod::*;
        match s {
            "charge_automatically" => Ok(ChargeAutomatically),
            "send_invoice" => Ok(SendInvoice),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "SubscriptionSchedulePhaseConfigurationCollectionMethod"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for SubscriptionSchedulePhaseConfigurationCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for SubscriptionSchedulePhaseConfigurationCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionSchedulePhaseConfigurationCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(SubscriptionSchedulePhaseConfigurationCollectionMethod))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SubscriptionSchedulePhaseConfigurationCollectionMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for SubscriptionSchedulePhaseConfigurationCollectionMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<SubscriptionSchedulePhaseConfigurationCollectionMethod>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            SubscriptionSchedulePhaseConfigurationCollectionMethod::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for SubscriptionSchedulePhaseConfigurationCollectionMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// When transitioning phases, controls how prorations are handled (if any).
/// Possible values are `create_prorations`, `none`, and `always_invoice`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum SubscriptionSchedulePhaseConfigurationProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl SubscriptionSchedulePhaseConfigurationProrationBehavior {
    pub fn as_str(&self) -> &str {
        use SubscriptionSchedulePhaseConfigurationProrationBehavior::*;
        match self {
            AlwaysInvoice => "always_invoice",
            CreateProrations => "create_prorations",
            None => "none",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for SubscriptionSchedulePhaseConfigurationProrationBehavior {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionSchedulePhaseConfigurationProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "SubscriptionSchedulePhaseConfigurationProrationBehavior"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for SubscriptionSchedulePhaseConfigurationProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for SubscriptionSchedulePhaseConfigurationProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionSchedulePhaseConfigurationProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(SubscriptionSchedulePhaseConfigurationProrationBehavior))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SubscriptionSchedulePhaseConfigurationProrationBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for SubscriptionSchedulePhaseConfigurationProrationBehavior {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<SubscriptionSchedulePhaseConfigurationProrationBehavior>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            SubscriptionSchedulePhaseConfigurationProrationBehavior::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for SubscriptionSchedulePhaseConfigurationProrationBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}

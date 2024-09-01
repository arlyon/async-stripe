/// A phase describes the plans, coupon, and trialing status of a subscription for a predefined time period.
#[derive(Clone, Debug)]
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
    /// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
    pub billing_cycle_anchor: Option<SubscriptionSchedulePhaseConfigurationBillingCycleAnchor>,
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    pub billing_thresholds: Option<stripe_shared::SubscriptionBillingThresholds>,
    /// Either `charge_automatically`, or `send_invoice`.
    /// When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
    pub collection_method: Option<SubscriptionSchedulePhaseConfigurationCollectionMethod>,
    /// ID of the coupon to use during this phase of the subscription schedule.
    pub coupon: Option<stripe_types::Expandable<stripe_shared::Coupon>>,
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
    pub discounts: Vec<stripe_shared::DiscountsResourceStackableDiscount>,
    /// The end of this phase of the subscription schedule.
    pub end_date: stripe_types::Timestamp,
    /// The invoice settings applicable during this phase.
    pub invoice_settings: Option<stripe_shared::InvoiceSettingSubscriptionSchedulePhaseSetting>,
    /// Subscription items to configure the subscription to during this phase of the subscription schedule.
    pub items: Vec<stripe_shared::SubscriptionScheduleConfigurationItem>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to a phase.
    /// Metadata on a schedule's phase will update the underlying subscription's `metadata` when the phase is entered.
    /// Updating the underlying subscription's `metadata` directly will not affect the current phase's `metadata`.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The account (if any) the charge was made on behalf of for charges associated with the schedule's subscription.
    /// See the Connect documentation for details.
    pub on_behalf_of: Option<stripe_types::Expandable<stripe_shared::Account>>,
    /// If the subscription schedule will prorate when transitioning to this phase.
    /// Possible values are `create_prorations` and `none`.
    pub proration_behavior: SubscriptionSchedulePhaseConfigurationProrationBehavior,
    /// The start of this phase of the subscription schedule.
    pub start_date: stripe_types::Timestamp,
    /// The account (if any) the associated subscription's payments will be attributed to for tax reporting, and where funds from each payment will be transferred to for each of the subscription's invoices.
    pub transfer_data: Option<stripe_shared::SubscriptionTransferData>,
    /// When the trial ends within the phase.
    pub trial_end: Option<stripe_types::Timestamp>,
}
#[doc(hidden)]
pub struct SubscriptionSchedulePhaseConfigurationBuilder {
    add_invoice_items: Option<Vec<stripe_shared::SubscriptionScheduleAddInvoiceItem>>,
    application_fee_percent: Option<Option<f64>>,
    automatic_tax: Option<Option<stripe_shared::SchedulesPhaseAutomaticTax>>,
    billing_cycle_anchor: Option<Option<SubscriptionSchedulePhaseConfigurationBillingCycleAnchor>>,
    billing_thresholds: Option<Option<stripe_shared::SubscriptionBillingThresholds>>,
    collection_method: Option<Option<SubscriptionSchedulePhaseConfigurationCollectionMethod>>,
    coupon: Option<Option<stripe_types::Expandable<stripe_shared::Coupon>>>,
    currency: Option<stripe_types::Currency>,
    default_payment_method: Option<Option<stripe_types::Expandable<stripe_shared::PaymentMethod>>>,
    default_tax_rates: Option<Option<Vec<stripe_shared::TaxRate>>>,
    description: Option<Option<String>>,
    discounts: Option<Vec<stripe_shared::DiscountsResourceStackableDiscount>>,
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
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: SubscriptionSchedulePhaseConfigurationBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SubscriptionSchedulePhaseConfigurationBuilder {
        type Out = SubscriptionSchedulePhaseConfiguration;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "add_invoice_items" => Deserialize::begin(&mut self.add_invoice_items),
                "application_fee_percent" => Deserialize::begin(&mut self.application_fee_percent),
                "automatic_tax" => Deserialize::begin(&mut self.automatic_tax),
                "billing_cycle_anchor" => Deserialize::begin(&mut self.billing_cycle_anchor),
                "billing_thresholds" => Deserialize::begin(&mut self.billing_thresholds),
                "collection_method" => Deserialize::begin(&mut self.collection_method),
                "coupon" => Deserialize::begin(&mut self.coupon),
                "currency" => Deserialize::begin(&mut self.currency),
                "default_payment_method" => Deserialize::begin(&mut self.default_payment_method),
                "default_tax_rates" => Deserialize::begin(&mut self.default_tax_rates),
                "description" => Deserialize::begin(&mut self.description),
                "discounts" => Deserialize::begin(&mut self.discounts),
                "end_date" => Deserialize::begin(&mut self.end_date),
                "invoice_settings" => Deserialize::begin(&mut self.invoice_settings),
                "items" => Deserialize::begin(&mut self.items),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "on_behalf_of" => Deserialize::begin(&mut self.on_behalf_of),
                "proration_behavior" => Deserialize::begin(&mut self.proration_behavior),
                "start_date" => Deserialize::begin(&mut self.start_date),
                "transfer_data" => Deserialize::begin(&mut self.transfer_data),
                "trial_end" => Deserialize::begin(&mut self.trial_end),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                add_invoice_items: Deserialize::default(),
                application_fee_percent: Deserialize::default(),
                automatic_tax: Deserialize::default(),
                billing_cycle_anchor: Deserialize::default(),
                billing_thresholds: Deserialize::default(),
                collection_method: Deserialize::default(),
                coupon: Deserialize::default(),
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
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(add_invoice_items),
                Some(application_fee_percent),
                Some(automatic_tax),
                Some(billing_cycle_anchor),
                Some(billing_thresholds),
                Some(collection_method),
                Some(coupon),
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
                self.add_invoice_items.take(),
                self.application_fee_percent,
                self.automatic_tax.take(),
                self.billing_cycle_anchor,
                self.billing_thresholds,
                self.collection_method,
                self.coupon.take(),
                self.currency,
                self.default_payment_method.take(),
                self.default_tax_rates.take(),
                self.description.take(),
                self.discounts.take(),
                self.end_date,
                self.invoice_settings.take(),
                self.items.take(),
                self.metadata.take(),
                self.on_behalf_of.take(),
                self.proration_behavior,
                self.start_date,
                self.transfer_data.take(),
                self.trial_end,
            )
            else {
                return None;
            };
            Some(Self::Out {
                add_invoice_items,
                application_fee_percent,
                automatic_tax,
                billing_cycle_anchor,
                billing_thresholds,
                collection_method,
                coupon,
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
            })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for SubscriptionSchedulePhaseConfiguration {
        type Builder = SubscriptionSchedulePhaseConfigurationBuilder;
    }

    impl FromValueOpt for SubscriptionSchedulePhaseConfiguration {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SubscriptionSchedulePhaseConfigurationBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "add_invoice_items" => b.add_invoice_items = FromValueOpt::from_value(v),
                    "application_fee_percent" => {
                        b.application_fee_percent = FromValueOpt::from_value(v)
                    }
                    "automatic_tax" => b.automatic_tax = FromValueOpt::from_value(v),
                    "billing_cycle_anchor" => b.billing_cycle_anchor = FromValueOpt::from_value(v),
                    "billing_thresholds" => b.billing_thresholds = FromValueOpt::from_value(v),
                    "collection_method" => b.collection_method = FromValueOpt::from_value(v),
                    "coupon" => b.coupon = FromValueOpt::from_value(v),
                    "currency" => b.currency = FromValueOpt::from_value(v),
                    "default_payment_method" => {
                        b.default_payment_method = FromValueOpt::from_value(v)
                    }
                    "default_tax_rates" => b.default_tax_rates = FromValueOpt::from_value(v),
                    "description" => b.description = FromValueOpt::from_value(v),
                    "discounts" => b.discounts = FromValueOpt::from_value(v),
                    "end_date" => b.end_date = FromValueOpt::from_value(v),
                    "invoice_settings" => b.invoice_settings = FromValueOpt::from_value(v),
                    "items" => b.items = FromValueOpt::from_value(v),
                    "metadata" => b.metadata = FromValueOpt::from_value(v),
                    "on_behalf_of" => b.on_behalf_of = FromValueOpt::from_value(v),
                    "proration_behavior" => b.proration_behavior = FromValueOpt::from_value(v),
                    "start_date" => b.start_date = FromValueOpt::from_value(v),
                    "transfer_data" => b.transfer_data = FromValueOpt::from_value(v),
                    "trial_end" => b.trial_end = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Possible values are `phase_start` or `automatic`.
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
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionSchedulePhaseConfigurationBillingCycleAnchor::*;
        match s {
            "automatic" => Ok(Automatic),
            "phase_start" => Ok(PhaseStart),
            _ => Err(stripe_types::StripeParseError),
        }
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
#[cfg(feature = "serialize")]
impl serde::Serialize for SubscriptionSchedulePhaseConfigurationBillingCycleAnchor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for SubscriptionSchedulePhaseConfigurationBillingCycleAnchor {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<SubscriptionSchedulePhaseConfigurationBillingCycleAnchor>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            SubscriptionSchedulePhaseConfigurationBillingCycleAnchor::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    SubscriptionSchedulePhaseConfigurationBillingCycleAnchor
);
#[cfg(feature = "deserialize")]
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
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionSchedulePhaseConfigurationCollectionMethod::*;
        match s {
            "charge_automatically" => Ok(ChargeAutomatically),
            "send_invoice" => Ok(SendInvoice),
            _ => Err(stripe_types::StripeParseError),
        }
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
#[cfg(feature = "serialize")]
impl serde::Serialize for SubscriptionSchedulePhaseConfigurationCollectionMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for SubscriptionSchedulePhaseConfigurationCollectionMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<SubscriptionSchedulePhaseConfigurationCollectionMethod>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            SubscriptionSchedulePhaseConfigurationCollectionMethod::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(SubscriptionSchedulePhaseConfigurationCollectionMethod);
#[cfg(feature = "deserialize")]
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
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionSchedulePhaseConfigurationProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
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
#[cfg(feature = "serialize")]
impl serde::Serialize for SubscriptionSchedulePhaseConfigurationProrationBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for SubscriptionSchedulePhaseConfigurationProrationBehavior {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<SubscriptionSchedulePhaseConfigurationProrationBehavior>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            SubscriptionSchedulePhaseConfigurationProrationBehavior::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(SubscriptionSchedulePhaseConfigurationProrationBehavior);
#[cfg(feature = "deserialize")]
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

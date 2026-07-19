/// You can now model subscriptions more flexibly using the [Prices API](https://api.stripe.com#prices).
/// It replaces the Plans API and is backwards compatible to simplify your migration.
///
/// Plans define the base price, currency, and billing cycle for recurring purchases of products.
/// [Products](https://api.stripe.com#products) help you track inventory or provisioning, and plans help you track pricing.
/// Different physical goods or levels of service should be represented by products, and pricing options should be represented by plans.
/// This approach lets you change prices without having to change your provisioning scheme.
///
/// For example, you might have a single "gold" product that has plans for $10/month, $100/year, €9/month, and €90/year.
///
/// Related guides: [Set up a subscription](https://docs.stripe.com/billing/subscriptions/set-up-subscription) and more about [products and prices](https://docs.stripe.com/products-prices/overview).
///
/// For more details see <<https://stripe.com/docs/api/plans/object>>.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Plan {
    /// Whether the plan can be used for new purchases.
    pub active: bool,
    /// The unit amount in cents (or local equivalent) to be charged, represented as a whole integer if possible.
    /// Only set if `billing_scheme=per_unit`.
    pub amount: Option<i64>,
    /// The unit amount in cents (or local equivalent) to be charged, represented as a decimal string with at most 12 decimal places.
    /// Only set if `billing_scheme=per_unit`.
    pub amount_decimal: Option<String>,
    /// Describes how to compute the price per period.
    /// Either `per_unit` or `tiered`.
    /// `per_unit` indicates that the fixed amount (specified in `amount`) will be charged per unit in `quantity` (for plans with `usage_type=licensed`), or per unit of total usage (for plans with `usage_type=metered`).
    /// `tiered` indicates that the unit pricing will be computed using a tiering strategy as defined using the `tiers` and `tiers_mode` attributes.
    pub billing_scheme: stripe_shared::PlanBillingScheme,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Unique identifier for the object.
    pub id: stripe_shared::PlanId,
    /// The frequency at which a subscription is billed. One of `day`, `week`, `month` or `year`.
    pub interval: stripe_shared::PlanInterval,
    /// The number of intervals (specified in the `interval` attribute) between subscription billings.
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    pub interval_count: u64,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The meter tracking the usage of a metered price
    pub meter: Option<String>,
    /// A brief description of the plan, hidden from customers.
    pub nickname: Option<String>,
    /// The product whose pricing this plan determines.
    pub product: Option<stripe_types::Expandable<stripe_shared::Product>>,
    /// Each element represents a pricing tier.
    /// This parameter requires `billing_scheme` to be set to `tiered`.
    /// See also the documentation for `billing_scheme`.
    pub tiers: Option<Vec<stripe_shared::PlanTier>>,
    /// Defines if the tiering price should be `graduated` or `volume` based.
    /// In `volume`-based tiering, the maximum quantity within a period determines the per unit price.
    /// In `graduated` tiering, pricing can change as the quantity grows.
    pub tiers_mode: Option<stripe_shared::PlanTiersMode>,
    /// Apply a transformation to the reported usage or set quantity before computing the amount billed.
    /// Cannot be combined with `tiers`.
    pub transform_usage: Option<stripe_shared::TransformUsage>,
    /// Default number of trial days when subscribing a customer to this plan using [`trial_from_plan=true`](https://docs.stripe.com/api#create_subscription-trial_from_plan).
    pub trial_period_days: Option<u32>,
    /// Configures how the quantity per period should be determined.
    /// Can be either `metered` or `licensed`.
    /// `licensed` automatically bills the `quantity` set when adding it to a subscription.
    /// `metered` aggregates the total usage based on usage records.
    /// Defaults to `licensed`.
    pub usage_type: stripe_shared::PlanUsageType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for Plan {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Plan").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PlanBuilder {
    active: Option<bool>,
    amount: Option<Option<i64>>,
    amount_decimal: Option<Option<String>>,
    billing_scheme: Option<stripe_shared::PlanBillingScheme>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    id: Option<stripe_shared::PlanId>,
    interval: Option<stripe_shared::PlanInterval>,
    interval_count: Option<u64>,
    livemode: Option<bool>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    meter: Option<Option<String>>,
    nickname: Option<Option<String>>,
    product: Option<Option<stripe_types::Expandable<stripe_shared::Product>>>,
    tiers: Option<Option<Vec<stripe_shared::PlanTier>>>,
    tiers_mode: Option<Option<stripe_shared::PlanTiersMode>>,
    transform_usage: Option<Option<stripe_shared::TransformUsage>>,
    trial_period_days: Option<Option<u32>>,
    usage_type: Option<stripe_shared::PlanUsageType>,
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

    impl Deserialize for Plan {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Plan>,
        builder: PlanBuilder,
    }

    impl Visitor for Place<Plan> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PlanBuilder {
                    active: Deserialize::default(),
                    amount: Deserialize::default(),
                    amount_decimal: Deserialize::default(),
                    billing_scheme: Deserialize::default(),
                    created: Deserialize::default(),
                    currency: Deserialize::default(),
                    id: Deserialize::default(),
                    interval: Deserialize::default(),
                    interval_count: Deserialize::default(),
                    livemode: Deserialize::default(),
                    metadata: Deserialize::default(),
                    meter: Deserialize::default(),
                    nickname: Deserialize::default(),
                    product: Deserialize::default(),
                    tiers: Deserialize::default(),
                    tiers_mode: Deserialize::default(),
                    transform_usage: Deserialize::default(),
                    trial_period_days: Deserialize::default(),
                    usage_type: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "active" => Deserialize::begin(&mut self.builder.active),
                "amount" => Deserialize::begin(&mut self.builder.amount),
                "amount_decimal" => Deserialize::begin(&mut self.builder.amount_decimal),
                "billing_scheme" => Deserialize::begin(&mut self.builder.billing_scheme),
                "created" => Deserialize::begin(&mut self.builder.created),
                "currency" => Deserialize::begin(&mut self.builder.currency),
                "id" => Deserialize::begin(&mut self.builder.id),
                "interval" => Deserialize::begin(&mut self.builder.interval),
                "interval_count" => Deserialize::begin(&mut self.builder.interval_count),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "metadata" => Deserialize::begin(&mut self.builder.metadata),
                "meter" => Deserialize::begin(&mut self.builder.meter),
                "nickname" => Deserialize::begin(&mut self.builder.nickname),
                "product" => Deserialize::begin(&mut self.builder.product),
                "tiers" => Deserialize::begin(&mut self.builder.tiers),
                "tiers_mode" => Deserialize::begin(&mut self.builder.tiers_mode),
                "transform_usage" => Deserialize::begin(&mut self.builder.transform_usage),
                "trial_period_days" => Deserialize::begin(&mut self.builder.trial_period_days),
                "usage_type" => Deserialize::begin(&mut self.builder.usage_type),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(active),
                Some(amount),
                Some(amount_decimal),
                Some(billing_scheme),
                Some(created),
                Some(currency),
                Some(id),
                Some(interval),
                Some(interval_count),
                Some(livemode),
                Some(metadata),
                Some(meter),
                Some(nickname),
                Some(product),
                Some(tiers),
                Some(tiers_mode),
                Some(transform_usage),
                Some(trial_period_days),
                Some(usage_type),
            ) = (
                self.builder.active,
                self.builder.amount,
                self.builder.amount_decimal.take(),
                self.builder.billing_scheme.take(),
                self.builder.created,
                self.builder.currency.take(),
                self.builder.id.take(),
                self.builder.interval.take(),
                self.builder.interval_count,
                self.builder.livemode,
                self.builder.metadata.take(),
                self.builder.meter.take(),
                self.builder.nickname.take(),
                self.builder.product.take(),
                self.builder.tiers.take(),
                self.builder.tiers_mode.take(),
                self.builder.transform_usage.take(),
                self.builder.trial_period_days,
                self.builder.usage_type.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(Plan {
                active,
                amount,
                amount_decimal,
                billing_scheme,
                created,
                currency,
                id,
                interval,
                interval_count,
                livemode,
                metadata,
                meter,
                nickname,
                product,
                tiers,
                tiers_mode,
                transform_usage,
                trial_period_days,
                usage_type,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for Plan {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("Plan", 20)?;
        s.serialize_field("active", &self.active)?;
        s.serialize_field("amount", &self.amount)?;
        s.serialize_field("amount_decimal", &self.amount_decimal)?;
        s.serialize_field("billing_scheme", &self.billing_scheme)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("interval", &self.interval)?;
        s.serialize_field("interval_count", &self.interval_count)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("meter", &self.meter)?;
        s.serialize_field("nickname", &self.nickname)?;
        s.serialize_field("product", &self.product)?;
        s.serialize_field("tiers", &self.tiers)?;
        s.serialize_field("tiers_mode", &self.tiers_mode)?;
        s.serialize_field("transform_usage", &self.transform_usage)?;
        s.serialize_field("trial_period_days", &self.trial_period_days)?;
        s.serialize_field("usage_type", &self.usage_type)?;

        s.serialize_field("object", "plan")?;
        s.end()
    }
}
impl stripe_types::Object for Plan {
    type Id = stripe_shared::PlanId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(PlanId);
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PlanBillingScheme {
    PerUnit,
    Tiered,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PlanBillingScheme {
    pub fn as_str(&self) -> &str {
        use PlanBillingScheme::*;
        match self {
            PerUnit => "per_unit",
            Tiered => "tiered",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PlanBillingScheme {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PlanBillingScheme::*;
        match s {
            "per_unit" => Ok(PerUnit),
            "tiered" => Ok(Tiered),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "PlanBillingScheme");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PlanBillingScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PlanBillingScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PlanBillingScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PlanBillingScheme)).finish_non_exhaustive()
    }
}
impl serde::Serialize for PlanBillingScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PlanBillingScheme {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<PlanBillingScheme> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PlanBillingScheme::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PlanBillingScheme {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PlanInterval {
    Day,
    Month,
    Week,
    Year,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PlanInterval {
    pub fn as_str(&self) -> &str {
        use PlanInterval::*;
        match self {
            Day => "day",
            Month => "month",
            Week => "week",
            Year => "year",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PlanInterval {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PlanInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "week" => Ok(Week),
            "year" => Ok(Year),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "PlanInterval");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PlanInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PlanInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PlanInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PlanInterval)).finish_non_exhaustive()
    }
}
impl serde::Serialize for PlanInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PlanInterval {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<PlanInterval> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PlanInterval::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PlanInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PlanTiersMode {
    Graduated,
    Volume,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PlanTiersMode {
    pub fn as_str(&self) -> &str {
        use PlanTiersMode::*;
        match self {
            Graduated => "graduated",
            Volume => "volume",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PlanTiersMode {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PlanTiersMode::*;
        match s {
            "graduated" => Ok(Graduated),
            "volume" => Ok(Volume),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "PlanTiersMode");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PlanTiersMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PlanTiersMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PlanTiersMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PlanTiersMode)).finish_non_exhaustive()
    }
}
impl serde::Serialize for PlanTiersMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PlanTiersMode {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<PlanTiersMode> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PlanTiersMode::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PlanTiersMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PlanUsageType {
    Licensed,
    Metered,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PlanUsageType {
    pub fn as_str(&self) -> &str {
        use PlanUsageType::*;
        match self {
            Licensed => "licensed",
            Metered => "metered",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PlanUsageType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PlanUsageType::*;
        match s {
            "licensed" => Ok(Licensed),
            "metered" => Ok(Metered),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "PlanUsageType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PlanUsageType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PlanUsageType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PlanUsageType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PlanUsageType)).finish_non_exhaustive()
    }
}
impl serde::Serialize for PlanUsageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PlanUsageType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<PlanUsageType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PlanUsageType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PlanUsageType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}

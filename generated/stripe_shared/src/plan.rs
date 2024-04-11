/// You can now model subscriptions more flexibly using the [Prices API](https://stripe.com/docs/api#prices).
/// It replaces the Plans API and is backwards compatible to simplify your migration.
///
/// Plans define the base price, currency, and billing cycle for recurring purchases of products.
/// [Products](https://stripe.com/docs/api#products) help you track inventory or provisioning, and plans help you track pricing.
/// Different physical goods or levels of service should be represented by products, and pricing options should be represented by plans.
/// This approach lets you change prices without having to change your provisioning scheme.
///
/// For example, you might have a single "gold" product that has plans for $10/month, $100/year, €9/month, and €90/year.
///
/// Related guides: [Set up a subscription](https://stripe.com/docs/billing/subscriptions/set-up-subscription) and more about [products and prices](https://stripe.com/docs/products-prices/overview).
///
/// For more details see <<https://stripe.com/docs/api/plans/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Plan {
    /// Whether the plan can be used for new purchases.
    pub active: bool,
    /// Specifies a usage aggregation strategy for plans of `usage_type=metered`.
    /// Allowed values are `sum` for summing up all usage during a period, `last_during_period` for using the last usage record reported within a period, `last_ever` for using the last usage record ever (across period bounds) or `max` which uses the usage record with the maximum reported usage during a period.
    /// Defaults to `sum`.
    pub aggregate_usage: Option<stripe_shared::PlanAggregateUsage>,
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
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
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
    /// Default number of trial days when subscribing a customer to this plan using [`trial_from_plan=true`](https://stripe.com/docs/api#create_subscription-trial_from_plan).
    pub trial_period_days: Option<u32>,
    /// Configures how the quantity per period should be determined.
    /// Can be either `metered` or `licensed`.
    /// `licensed` automatically bills the `quantity` set when adding it to a subscription.
    /// `metered` aggregates the total usage based on usage records.
    /// Defaults to `licensed`.
    pub usage_type: stripe_shared::PlanUsageType,
}
#[doc(hidden)]
pub struct PlanBuilder {
    active: Option<bool>,
    aggregate_usage: Option<Option<stripe_shared::PlanAggregateUsage>>,
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
    nickname: Option<Option<String>>,
    product: Option<Option<stripe_types::Expandable<stripe_shared::Product>>>,
    tiers: Option<Option<Vec<stripe_shared::PlanTier>>>,
    tiers_mode: Option<Option<stripe_shared::PlanTiersMode>>,
    transform_usage: Option<Option<stripe_shared::TransformUsage>>,
    trial_period_days: Option<Option<u32>>,
    usage_type: Option<stripe_shared::PlanUsageType>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
            Ok(Box::new(Builder { out: &mut self.out, builder: PlanBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PlanBuilder {
        type Out = Plan;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "active" => Deserialize::begin(&mut self.active),
                "aggregate_usage" => Deserialize::begin(&mut self.aggregate_usage),
                "amount" => Deserialize::begin(&mut self.amount),
                "amount_decimal" => Deserialize::begin(&mut self.amount_decimal),
                "billing_scheme" => Deserialize::begin(&mut self.billing_scheme),
                "created" => Deserialize::begin(&mut self.created),
                "currency" => Deserialize::begin(&mut self.currency),
                "id" => Deserialize::begin(&mut self.id),
                "interval" => Deserialize::begin(&mut self.interval),
                "interval_count" => Deserialize::begin(&mut self.interval_count),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "nickname" => Deserialize::begin(&mut self.nickname),
                "product" => Deserialize::begin(&mut self.product),
                "tiers" => Deserialize::begin(&mut self.tiers),
                "tiers_mode" => Deserialize::begin(&mut self.tiers_mode),
                "transform_usage" => Deserialize::begin(&mut self.transform_usage),
                "trial_period_days" => Deserialize::begin(&mut self.trial_period_days),
                "usage_type" => Deserialize::begin(&mut self.usage_type),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                active: Deserialize::default(),
                aggregate_usage: Deserialize::default(),
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
                nickname: Deserialize::default(),
                product: Deserialize::default(),
                tiers: Deserialize::default(),
                tiers_mode: Deserialize::default(),
                transform_usage: Deserialize::default(),
                trial_period_days: Deserialize::default(),
                usage_type: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                active: self.active?,
                aggregate_usage: self.aggregate_usage?,
                amount: self.amount?,
                amount_decimal: self.amount_decimal.take()?,
                billing_scheme: self.billing_scheme?,
                created: self.created?,
                currency: self.currency?,
                id: self.id.take()?,
                interval: self.interval?,
                interval_count: self.interval_count?,
                livemode: self.livemode?,
                metadata: self.metadata.take()?,
                nickname: self.nickname.take()?,
                product: self.product.take()?,
                tiers: self.tiers.take()?,
                tiers_mode: self.tiers_mode?,
                transform_usage: self.transform_usage?,
                trial_period_days: self.trial_period_days?,
                usage_type: self.usage_type?,
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

    impl ObjectDeser for Plan {
        type Builder = PlanBuilder;
    }

    impl FromValueOpt for Plan {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PlanBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "active" => b.active = Some(FromValueOpt::from_value(v)?),
                    "aggregate_usage" => b.aggregate_usage = Some(FromValueOpt::from_value(v)?),
                    "amount" => b.amount = Some(FromValueOpt::from_value(v)?),
                    "amount_decimal" => b.amount_decimal = Some(FromValueOpt::from_value(v)?),
                    "billing_scheme" => b.billing_scheme = Some(FromValueOpt::from_value(v)?),
                    "created" => b.created = Some(FromValueOpt::from_value(v)?),
                    "currency" => b.currency = Some(FromValueOpt::from_value(v)?),
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "interval" => b.interval = Some(FromValueOpt::from_value(v)?),
                    "interval_count" => b.interval_count = Some(FromValueOpt::from_value(v)?),
                    "livemode" => b.livemode = Some(FromValueOpt::from_value(v)?),
                    "metadata" => b.metadata = Some(FromValueOpt::from_value(v)?),
                    "nickname" => b.nickname = Some(FromValueOpt::from_value(v)?),
                    "product" => b.product = Some(FromValueOpt::from_value(v)?),
                    "tiers" => b.tiers = Some(FromValueOpt::from_value(v)?),
                    "tiers_mode" => b.tiers_mode = Some(FromValueOpt::from_value(v)?),
                    "transform_usage" => b.transform_usage = Some(FromValueOpt::from_value(v)?),
                    "trial_period_days" => b.trial_period_days = Some(FromValueOpt::from_value(v)?),
                    "usage_type" => b.usage_type = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for Plan {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("Plan", 20)?;
        s.serialize_field("active", &self.active)?;
        s.serialize_field("aggregate_usage", &self.aggregate_usage)?;
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
}
stripe_types::def_id!(PlanId);
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PlanAggregateUsage {
    LastDuringPeriod,
    LastEver,
    Max,
    Sum,
}
impl PlanAggregateUsage {
    pub fn as_str(self) -> &'static str {
        use PlanAggregateUsage::*;
        match self {
            LastDuringPeriod => "last_during_period",
            LastEver => "last_ever",
            Max => "max",
            Sum => "sum",
        }
    }
}

impl std::str::FromStr for PlanAggregateUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PlanAggregateUsage::*;
        match s {
            "last_during_period" => Ok(LastDuringPeriod),
            "last_ever" => Ok(LastEver),
            "max" => Ok(Max),
            "sum" => Ok(Sum),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PlanAggregateUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PlanAggregateUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PlanAggregateUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PlanAggregateUsage {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PlanAggregateUsage> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PlanAggregateUsage::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PlanAggregateUsage);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PlanAggregateUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for PlanAggregateUsage"))
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PlanBillingScheme {
    PerUnit,
    Tiered,
}
impl PlanBillingScheme {
    pub fn as_str(self) -> &'static str {
        use PlanBillingScheme::*;
        match self {
            PerUnit => "per_unit",
            Tiered => "tiered",
        }
    }
}

impl std::str::FromStr for PlanBillingScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PlanBillingScheme::*;
        match s {
            "per_unit" => Ok(PerUnit),
            "tiered" => Ok(Tiered),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PlanBillingScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PlanBillingScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for PlanBillingScheme {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PlanBillingScheme> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PlanBillingScheme::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PlanBillingScheme);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PlanBillingScheme {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for PlanBillingScheme"))
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PlanInterval {
    Day,
    Month,
    Week,
    Year,
}
impl PlanInterval {
    pub fn as_str(self) -> &'static str {
        use PlanInterval::*;
        match self {
            Day => "day",
            Month => "month",
            Week => "week",
            Year => "year",
        }
    }
}

impl std::str::FromStr for PlanInterval {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PlanInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "week" => Ok(Week),
            "year" => Ok(Year),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PlanInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PlanInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for PlanInterval {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PlanInterval> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PlanInterval::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PlanInterval);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PlanInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PlanInterval"))
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PlanTiersMode {
    Graduated,
    Volume,
}
impl PlanTiersMode {
    pub fn as_str(self) -> &'static str {
        use PlanTiersMode::*;
        match self {
            Graduated => "graduated",
            Volume => "volume",
        }
    }
}

impl std::str::FromStr for PlanTiersMode {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PlanTiersMode::*;
        match s {
            "graduated" => Ok(Graduated),
            "volume" => Ok(Volume),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PlanTiersMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PlanTiersMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for PlanTiersMode {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PlanTiersMode> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PlanTiersMode::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PlanTiersMode);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PlanTiersMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PlanTiersMode"))
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PlanUsageType {
    Licensed,
    Metered,
}
impl PlanUsageType {
    pub fn as_str(self) -> &'static str {
        use PlanUsageType::*;
        match self {
            Licensed => "licensed",
            Metered => "metered",
        }
    }
}

impl std::str::FromStr for PlanUsageType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PlanUsageType::*;
        match s {
            "licensed" => Ok(Licensed),
            "metered" => Ok(Metered),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PlanUsageType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PlanUsageType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for PlanUsageType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PlanUsageType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PlanUsageType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PlanUsageType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PlanUsageType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PlanUsageType"))
    }
}

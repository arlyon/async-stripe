/// You can now model subscriptions more flexibly using the [Prices API](https://stripe.com/docs/api#prices).
///
/// It replaces the Plans API and is backwards compatible to simplify your migration.  Plans define the base price, currency, and billing cycle for recurring purchases of products. [Products](https://stripe.com/docs/api#products) help you track inventory or provisioning, and plans help you track pricing.
/// Different physical goods or levels of service should be represented by products, and pricing options should be represented by plans.
/// This approach lets you change prices without having to change your provisioning scheme.  For example, you might have a single "gold" product that has plans for $10/month, $100/year, €9/month, and €90/year.  Related guides: [Set up a subscription](https://stripe.com/docs/billing/subscriptions/set-up-subscription) and more about [products and prices](https://stripe.com/docs/products-prices/overview).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Plan {
    /// Whether the plan can be used for new purchases.
    pub active: bool,
    /// Specifies a usage aggregation strategy for plans of `usage_type=metered`.
    ///
    /// Allowed values are `sum` for summing up all usage during a period, `last_during_period` for using the last usage record reported within a period, `last_ever` for using the last usage record ever (across period bounds) or `max` which uses the usage record with the maximum reported usage during a period.
    /// Defaults to `sum`.
    pub aggregate_usage: Option<PlanAggregateUsage>,
    /// The unit amount in %s to be charged, represented as a whole integer if possible.
    ///
    /// Only set if `billing_scheme=per_unit`.
    pub amount: Option<i64>,
    /// The unit amount in %s to be charged, represented as a decimal string with at most 12 decimal places.
    ///
    /// Only set if `billing_scheme=per_unit`.
    pub amount_decimal: Option<String>,
    /// Describes how to compute the price per period.
    ///
    /// Either `per_unit` or `tiered`.
    /// `per_unit` indicates that the fixed amount (specified in `amount`) will be charged per unit in `quantity` (for plans with `usage_type=licensed`), or per unit of total usage (for plans with `usage_type=metered`).
    /// `tiered` indicates that the unit pricing will be computed using a tiering strategy as defined using the `tiers` and `tiers_mode` attributes.
    pub billing_scheme: PlanBillingScheme,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Unique identifier for the object.
    pub id: stripe_types::plan::PlanId,
    /// The frequency at which a subscription is billed.
    ///
    /// One of `day`, `week`, `month` or `year`.
    pub interval: PlanInterval,
    /// The number of intervals (specified in the `interval` attribute) between subscription billings.
    ///
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    pub interval_count: u64,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// A brief description of the plan, hidden from customers.
    pub nickname: Option<String>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: PlanObject,
    /// The product whose pricing this plan determines.
    pub product: Option<stripe_types::Expandable<stripe_types::product::Product>>,
    /// Each element represents a pricing tier.
    ///
    /// This parameter requires `billing_scheme` to be set to `tiered`.
    /// See also the documentation for `billing_scheme`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<stripe_types::plan::tier::Tier>>,
    /// Defines if the tiering price should be `graduated` or `volume` based.
    ///
    /// In `volume`-based tiering, the maximum quantity within a period determines the per unit price.
    /// In `graduated` tiering, pricing can change as the quantity grows.
    pub tiers_mode: Option<PlanTiersMode>,
    /// Apply a transformation to the reported usage or set quantity before computing the amount billed.
    ///
    /// Cannot be combined with `tiers`.
    pub transform_usage: Option<stripe_types::plan::transform_usage::TransformUsage>,
    /// Default number of trial days when subscribing a customer to this plan using [`trial_from_plan=true`](https://stripe.com/docs/api#create_subscription-trial_from_plan).
    pub trial_period_days: Option<u32>,
    /// Configures how the quantity per period should be determined.
    ///
    /// Can be either `metered` or `licensed`.
    /// `licensed` automatically bills the `quantity` set when adding it to a subscription.
    /// `metered` aggregates the total usage based on usage records.
    /// Defaults to `licensed`.
    pub usage_type: PlanUsageType,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Plan {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Specifies a usage aggregation strategy for plans of `usage_type=metered`.
///
/// Allowed values are `sum` for summing up all usage during a period, `last_during_period` for using the last usage record reported within a period, `last_ever` for using the last usage record ever (across period bounds) or `max` which uses the usage record with the maximum reported usage during a period.
/// Defaults to `sum`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PlanAggregateUsage {
    LastDuringPeriod,
    LastEver,
    Max,
    Sum,
}

impl PlanAggregateUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LastDuringPeriod => "last_during_period",
            Self::LastEver => "last_ever",
            Self::Max => "max",
            Self::Sum => "sum",
        }
    }
}

impl std::str::FromStr for PlanAggregateUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "last_during_period" => Ok(Self::LastDuringPeriod),
            "last_ever" => Ok(Self::LastEver),
            "max" => Ok(Self::Max),
            "sum" => Ok(Self::Sum),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for PlanAggregateUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PlanAggregateUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
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
impl<'de> serde::Deserialize<'de> for PlanAggregateUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for PlanAggregateUsage"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PlanAggregateUsage {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PlanAggregateUsage> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PlanAggregateUsage::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Describes how to compute the price per period.
///
/// Either `per_unit` or `tiered`.
/// `per_unit` indicates that the fixed amount (specified in `amount`) will be charged per unit in `quantity` (for plans with `usage_type=licensed`), or per unit of total usage (for plans with `usage_type=metered`).
/// `tiered` indicates that the unit pricing will be computed using a tiering strategy as defined using the `tiers` and `tiers_mode` attributes.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PlanBillingScheme {
    PerUnit,
    Tiered,
}

impl PlanBillingScheme {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PerUnit => "per_unit",
            Self::Tiered => "tiered",
        }
    }
}

impl std::str::FromStr for PlanBillingScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "per_unit" => Ok(Self::PerUnit),
            "tiered" => Ok(Self::Tiered),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for PlanBillingScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PlanBillingScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
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
impl<'de> serde::Deserialize<'de> for PlanBillingScheme {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for PlanBillingScheme"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PlanBillingScheme {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PlanBillingScheme> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PlanBillingScheme::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// The frequency at which a subscription is billed.
///
/// One of `day`, `week`, `month` or `year`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PlanInterval {
    Day,
    Month,
    Week,
    Year,
}

impl PlanInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Month => "month",
            Self::Week => "week",
            Self::Year => "year",
        }
    }
}

impl std::str::FromStr for PlanInterval {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "day" => Ok(Self::Day),
            "month" => Ok(Self::Month),
            "week" => Ok(Self::Week),
            "year" => Ok(Self::Year),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for PlanInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PlanInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
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
impl<'de> serde::Deserialize<'de> for PlanInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PlanInterval"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PlanInterval {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PlanInterval> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PlanInterval::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PlanObject {
    Plan,
}

impl PlanObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Plan => "plan",
        }
    }
}

impl std::str::FromStr for PlanObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "plan" => Ok(Self::Plan),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for PlanObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PlanObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for PlanObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PlanObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PlanObject"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PlanObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PlanObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PlanObject::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Defines if the tiering price should be `graduated` or `volume` based.
///
/// In `volume`-based tiering, the maximum quantity within a period determines the per unit price.
/// In `graduated` tiering, pricing can change as the quantity grows.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PlanTiersMode {
    Graduated,
    Volume,
}

impl PlanTiersMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Graduated => "graduated",
            Self::Volume => "volume",
        }
    }
}

impl std::str::FromStr for PlanTiersMode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "graduated" => Ok(Self::Graduated),
            "volume" => Ok(Self::Volume),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for PlanTiersMode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PlanTiersMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
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
impl<'de> serde::Deserialize<'de> for PlanTiersMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PlanTiersMode"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PlanTiersMode {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PlanTiersMode> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PlanTiersMode::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Configures how the quantity per period should be determined.
///
/// Can be either `metered` or `licensed`.
/// `licensed` automatically bills the `quantity` set when adding it to a subscription.
/// `metered` aggregates the total usage based on usage records.
/// Defaults to `licensed`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PlanUsageType {
    Licensed,
    Metered,
}

impl PlanUsageType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Licensed => "licensed",
            Self::Metered => "metered",
        }
    }
}

impl std::str::FromStr for PlanUsageType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "licensed" => Ok(Self::Licensed),
            "metered" => Ok(Self::Metered),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for PlanUsageType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PlanUsageType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
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
impl<'de> serde::Deserialize<'de> for PlanUsageType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PlanUsageType"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PlanUsageType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PlanUsageType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PlanUsageType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
impl stripe_types::Object for Plan {
    type Id = stripe_types::plan::PlanId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(PlanId);
pub mod deleted;
pub use deleted::DeletedPlan;
pub mod tier;
pub use tier::Tier;
pub mod transform_usage;
pub use transform_usage::TransformUsage;

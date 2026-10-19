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
    pub billing_scheme: stripe_shared::BillingScheme,
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
    pub tiers_mode: Option<stripe_shared::TiersMode>,
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
    pub usage_type: stripe_shared::UsageType,
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
    billing_scheme: Option<stripe_shared::BillingScheme>,
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
    tiers_mode: Option<Option<stripe_shared::TiersMode>>,
    transform_usage: Option<Option<stripe_shared::TransformUsage>>,
    trial_period_days: Option<Option<u32>>,
    usage_type: Option<stripe_shared::UsageType>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
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
                "meter" => Deserialize::begin(&mut self.meter),
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
                active: None,
                amount: Some(None),
                amount_decimal: Some(None),
                billing_scheme: None,
                created: None,
                currency: None,
                id: None,
                interval: None,
                interval_count: None,
                livemode: None,
                metadata: Some(None),
                meter: Some(None),
                nickname: Some(None),
                product: Some(None),
                tiers: Some(None),
                tiers_mode: Some(None),
                transform_usage: Some(None),
                trial_period_days: Some(None),
                usage_type: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
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
                self.active,
                self.amount,
                self.amount_decimal.take(),
                self.billing_scheme.take(),
                self.created,
                self.currency.take(),
                self.id.take(),
                self.interval.take(),
                self.interval_count,
                self.livemode,
                self.metadata.take(),
                self.meter.take(),
                self.nickname.take(),
                self.product.take(),
                self.tiers.take(),
                self.tiers_mode.take(),
                self.transform_usage.take(),
                self.trial_period_days,
                self.usage_type.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
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
            })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
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
                    "active" => b.active = FromValueOpt::from_value(v),
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "amount_decimal" => b.amount_decimal = FromValueOpt::from_value(v),
                    "billing_scheme" => b.billing_scheme = FromValueOpt::from_value(v),
                    "created" => b.created = FromValueOpt::from_value(v),
                    "currency" => b.currency = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "interval" => b.interval = FromValueOpt::from_value(v),
                    "interval_count" => b.interval_count = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "metadata" => b.metadata = FromValueOpt::from_value(v),
                    "meter" => b.meter = FromValueOpt::from_value(v),
                    "nickname" => b.nickname = FromValueOpt::from_value(v),
                    "product" => b.product = FromValueOpt::from_value(v),
                    "tiers" => b.tiers = FromValueOpt::from_value(v),
                    "tiers_mode" => b.tiers_mode = FromValueOpt::from_value(v),
                    "transform_usage" => b.transform_usage = FromValueOpt::from_value(v),
                    "trial_period_days" => b.trial_period_days = FromValueOpt::from_value(v),
                    "usage_type" => b.usage_type = FromValueOpt::from_value(v),
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
/// Describes how to compute the price per period.
/// Either `per_unit` or `tiered`.
/// `per_unit` indicates that the fixed amount (specified in `amount`) will be charged per unit in `quantity` (for plans with `usage_type=licensed`), or per unit of total usage (for plans with `usage_type=metered`).
/// `tiered` indicates that the unit pricing will be computed using a tiering strategy as defined using the `tiers` and `tiers_mode` attributes.
pub type PlanBillingScheme = stripe_shared::BillingScheme;
/// The frequency at which a subscription is billed. One of `day`, `week`, `month` or `year`.
pub type PlanInterval = stripe_shared::PlanInterval;
/// Defines if the tiering price should be `graduated` or `volume` based.
/// In `volume`-based tiering, the maximum quantity within a period determines the per unit price.
/// In `graduated` tiering, pricing can change as the quantity grows.
pub type PlanTiersMode = stripe_shared::TiersMode;
/// Configures how the quantity per period should be determined.
/// Can be either `metered` or `licensed`.
/// `licensed` automatically bills the `quantity` set when adding it to a subscription.
/// `metered` aggregates the total usage based on usage records.
/// Defaults to `licensed`.
pub type PlanUsageType = stripe_shared::UsageType;

/// Prices define the unit cost, currency, and (optional) billing cycle for both recurring and one-time purchases of products.
/// [Products](https://stripe.com/docs/api#products) help you track inventory or provisioning, and prices help you track payment terms.
/// Different physical goods or levels of service should be represented by products, and pricing options should be represented by prices.
/// This approach lets you change prices without having to change your provisioning scheme.
///
/// For example, you might have a single "gold" product that has prices for $10/month, $100/year, and €9 once.
///
/// Related guides: [Set up a subscription](https://stripe.com/docs/billing/subscriptions/set-up-subscription), [create an invoice](https://stripe.com/docs/billing/invoices/create), and more about [products and prices](https://stripe.com/docs/products-prices/overview).
///
/// For more details see <<https://stripe.com/docs/api/prices/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Price {
    /// Whether the price can be used for new purchases.
    pub active: bool,
    /// Describes how to compute the price per period.
    /// Either `per_unit` or `tiered`.
    /// `per_unit` indicates that the fixed amount (specified in `unit_amount` or `unit_amount_decimal`) will be charged per unit in `quantity` (for prices with `usage_type=licensed`), or per unit of total usage (for prices with `usage_type=metered`).
    /// `tiered` indicates that the unit pricing will be computed using a tiering strategy as defined using the `tiers` and `tiers_mode` attributes.
    pub billing_scheme: stripe_shared::PriceBillingScheme,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Prices defined in each available currency option.
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    pub currency_options:
        Option<std::collections::HashMap<stripe_types::Currency, stripe_shared::CurrencyOption>>,
    /// When set, provides configuration for the amount to be adjusted by the customer during Checkout Sessions and Payment Links.
    pub custom_unit_amount: Option<stripe_shared::CustomUnitAmount>,
    /// Unique identifier for the object.
    pub id: stripe_shared::PriceId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// A lookup key used to retrieve prices dynamically from a static string.
    /// This may be up to 200 characters.
    pub lookup_key: Option<String>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// A brief description of the price, hidden from customers.
    pub nickname: Option<String>,
    /// The ID of the product this price is associated with.
    pub product: stripe_types::Expandable<stripe_shared::Product>,
    /// The recurring components of a price such as `interval` and `usage_type`.
    pub recurring: Option<stripe_shared::Recurring>,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    pub tax_behavior: Option<stripe_shared::PriceTaxBehavior>,
    /// Each element represents a pricing tier.
    /// This parameter requires `billing_scheme` to be set to `tiered`.
    /// See also the documentation for `billing_scheme`.
    pub tiers: Option<Vec<stripe_shared::PriceTier>>,
    /// Defines if the tiering price should be `graduated` or `volume` based.
    /// In `volume`-based tiering, the maximum quantity within a period determines the per unit price.
    /// In `graduated` tiering, pricing can change as the quantity grows.
    pub tiers_mode: Option<stripe_shared::PriceTiersMode>,
    /// Apply a transformation to the reported usage or set quantity before computing the amount billed.
    /// Cannot be combined with `tiers`.
    pub transform_quantity: Option<stripe_shared::TransformQuantity>,
    /// One of `one_time` or `recurring` depending on whether the price is for a one-time purchase or a recurring (subscription) purchase.
    #[cfg_attr(feature = "deserialize", serde(rename = "type"))]
    pub type_: stripe_shared::PriceType,
    /// The unit amount in cents (or local equivalent) to be charged, represented as a whole integer if possible.
    /// Only set if `billing_scheme=per_unit`.
    pub unit_amount: Option<i64>,
    /// The unit amount in cents (or local equivalent) to be charged, represented as a decimal string with at most 12 decimal places.
    /// Only set if `billing_scheme=per_unit`.
    pub unit_amount_decimal: Option<String>,
}
#[doc(hidden)]
pub struct PriceBuilder {
    active: Option<bool>,
    billing_scheme: Option<stripe_shared::PriceBillingScheme>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    currency_options: Option<
        Option<std::collections::HashMap<stripe_types::Currency, stripe_shared::CurrencyOption>>,
    >,
    custom_unit_amount: Option<Option<stripe_shared::CustomUnitAmount>>,
    id: Option<stripe_shared::PriceId>,
    livemode: Option<bool>,
    lookup_key: Option<Option<String>>,
    metadata: Option<std::collections::HashMap<String, String>>,
    nickname: Option<Option<String>>,
    product: Option<stripe_types::Expandable<stripe_shared::Product>>,
    recurring: Option<Option<stripe_shared::Recurring>>,
    tax_behavior: Option<Option<stripe_shared::PriceTaxBehavior>>,
    tiers: Option<Option<Vec<stripe_shared::PriceTier>>>,
    tiers_mode: Option<Option<stripe_shared::PriceTiersMode>>,
    transform_quantity: Option<Option<stripe_shared::TransformQuantity>>,
    type_: Option<stripe_shared::PriceType>,
    unit_amount: Option<Option<i64>>,
    unit_amount_decimal: Option<Option<String>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for Price {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Price>,
        builder: PriceBuilder,
    }

    impl Visitor for Place<Price> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PriceBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PriceBuilder {
        type Out = Price;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "active" => Deserialize::begin(&mut self.active),
                "billing_scheme" => Deserialize::begin(&mut self.billing_scheme),
                "created" => Deserialize::begin(&mut self.created),
                "currency" => Deserialize::begin(&mut self.currency),
                "currency_options" => Deserialize::begin(&mut self.currency_options),
                "custom_unit_amount" => Deserialize::begin(&mut self.custom_unit_amount),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "lookup_key" => Deserialize::begin(&mut self.lookup_key),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "nickname" => Deserialize::begin(&mut self.nickname),
                "product" => Deserialize::begin(&mut self.product),
                "recurring" => Deserialize::begin(&mut self.recurring),
                "tax_behavior" => Deserialize::begin(&mut self.tax_behavior),
                "tiers" => Deserialize::begin(&mut self.tiers),
                "tiers_mode" => Deserialize::begin(&mut self.tiers_mode),
                "transform_quantity" => Deserialize::begin(&mut self.transform_quantity),
                "type" => Deserialize::begin(&mut self.type_),
                "unit_amount" => Deserialize::begin(&mut self.unit_amount),
                "unit_amount_decimal" => Deserialize::begin(&mut self.unit_amount_decimal),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                active: Deserialize::default(),
                billing_scheme: Deserialize::default(),
                created: Deserialize::default(),
                currency: Deserialize::default(),
                currency_options: Deserialize::default(),
                custom_unit_amount: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                lookup_key: Deserialize::default(),
                metadata: Deserialize::default(),
                nickname: Deserialize::default(),
                product: Deserialize::default(),
                recurring: Deserialize::default(),
                tax_behavior: Deserialize::default(),
                tiers: Deserialize::default(),
                tiers_mode: Deserialize::default(),
                transform_quantity: Deserialize::default(),
                type_: Deserialize::default(),
                unit_amount: Deserialize::default(),
                unit_amount_decimal: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                active: self.active?,
                billing_scheme: self.billing_scheme?,
                created: self.created?,
                currency: self.currency?,
                currency_options: self.currency_options.take()?,
                custom_unit_amount: self.custom_unit_amount?,
                id: self.id.take()?,
                livemode: self.livemode?,
                lookup_key: self.lookup_key.take()?,
                metadata: self.metadata.take()?,
                nickname: self.nickname.take()?,
                product: self.product.take()?,
                recurring: self.recurring?,
                tax_behavior: self.tax_behavior?,
                tiers: self.tiers.take()?,
                tiers_mode: self.tiers_mode?,
                transform_quantity: self.transform_quantity?,
                type_: self.type_?,
                unit_amount: self.unit_amount?,
                unit_amount_decimal: self.unit_amount_decimal.take()?,
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

    impl ObjectDeser for Price {
        type Builder = PriceBuilder;
    }

    impl FromValueOpt for Price {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PriceBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "active" => b.active = Some(FromValueOpt::from_value(v)?),
                    "billing_scheme" => b.billing_scheme = Some(FromValueOpt::from_value(v)?),
                    "created" => b.created = Some(FromValueOpt::from_value(v)?),
                    "currency" => b.currency = Some(FromValueOpt::from_value(v)?),
                    "currency_options" => b.currency_options = Some(FromValueOpt::from_value(v)?),
                    "custom_unit_amount" => {
                        b.custom_unit_amount = Some(FromValueOpt::from_value(v)?)
                    }
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "livemode" => b.livemode = Some(FromValueOpt::from_value(v)?),
                    "lookup_key" => b.lookup_key = Some(FromValueOpt::from_value(v)?),
                    "metadata" => b.metadata = Some(FromValueOpt::from_value(v)?),
                    "nickname" => b.nickname = Some(FromValueOpt::from_value(v)?),
                    "product" => b.product = Some(FromValueOpt::from_value(v)?),
                    "recurring" => b.recurring = Some(FromValueOpt::from_value(v)?),
                    "tax_behavior" => b.tax_behavior = Some(FromValueOpt::from_value(v)?),
                    "tiers" => b.tiers = Some(FromValueOpt::from_value(v)?),
                    "tiers_mode" => b.tiers_mode = Some(FromValueOpt::from_value(v)?),
                    "transform_quantity" => {
                        b.transform_quantity = Some(FromValueOpt::from_value(v)?)
                    }
                    "type" => b.type_ = Some(FromValueOpt::from_value(v)?),
                    "unit_amount" => b.unit_amount = Some(FromValueOpt::from_value(v)?),
                    "unit_amount_decimal" => {
                        b.unit_amount_decimal = Some(FromValueOpt::from_value(v)?)
                    }

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for Price {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("Price", 21)?;
        s.serialize_field("active", &self.active)?;
        s.serialize_field("billing_scheme", &self.billing_scheme)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("currency_options", &self.currency_options)?;
        s.serialize_field("custom_unit_amount", &self.custom_unit_amount)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("lookup_key", &self.lookup_key)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("nickname", &self.nickname)?;
        s.serialize_field("product", &self.product)?;
        s.serialize_field("recurring", &self.recurring)?;
        s.serialize_field("tax_behavior", &self.tax_behavior)?;
        s.serialize_field("tiers", &self.tiers)?;
        s.serialize_field("tiers_mode", &self.tiers_mode)?;
        s.serialize_field("transform_quantity", &self.transform_quantity)?;
        s.serialize_field("type", &self.type_)?;
        s.serialize_field("unit_amount", &self.unit_amount)?;
        s.serialize_field("unit_amount_decimal", &self.unit_amount_decimal)?;

        s.serialize_field("object", "price")?;
        s.end()
    }
}
impl stripe_types::Object for Price {
    type Id = stripe_shared::PriceId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(PriceId);
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PriceBillingScheme {
    PerUnit,
    Tiered,
}
impl PriceBillingScheme {
    pub fn as_str(self) -> &'static str {
        use PriceBillingScheme::*;
        match self {
            PerUnit => "per_unit",
            Tiered => "tiered",
        }
    }
}

impl std::str::FromStr for PriceBillingScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PriceBillingScheme::*;
        match s {
            "per_unit" => Ok(PerUnit),
            "tiered" => Ok(Tiered),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PriceBillingScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PriceBillingScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PriceBillingScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PriceBillingScheme {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PriceBillingScheme> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PriceBillingScheme::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PriceBillingScheme);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PriceBillingScheme {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for PriceBillingScheme"))
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PriceTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl PriceTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use PriceTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for PriceTaxBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PriceTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PriceTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PriceTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PriceTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PriceTaxBehavior {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PriceTaxBehavior> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PriceTaxBehavior::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PriceTaxBehavior);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PriceTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for PriceTaxBehavior"))
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PriceTiersMode {
    Graduated,
    Volume,
}
impl PriceTiersMode {
    pub fn as_str(self) -> &'static str {
        use PriceTiersMode::*;
        match self {
            Graduated => "graduated",
            Volume => "volume",
        }
    }
}

impl std::str::FromStr for PriceTiersMode {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PriceTiersMode::*;
        match s {
            "graduated" => Ok(Graduated),
            "volume" => Ok(Volume),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PriceTiersMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PriceTiersMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PriceTiersMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PriceTiersMode {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PriceTiersMode> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PriceTiersMode::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PriceTiersMode);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PriceTiersMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PriceTiersMode"))
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PriceType {
    OneTime,
    Recurring,
}
impl PriceType {
    pub fn as_str(self) -> &'static str {
        use PriceType::*;
        match self {
            OneTime => "one_time",
            Recurring => "recurring",
        }
    }
}

impl std::str::FromStr for PriceType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PriceType::*;
        match s {
            "one_time" => Ok(OneTime),
            "recurring" => Ok(Recurring),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PriceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PriceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PriceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PriceType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PriceType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PriceType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PriceType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PriceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PriceType"))
    }
}

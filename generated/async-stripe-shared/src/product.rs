/// Products describe the specific goods or services you offer to your customers.
/// For example, you might offer a Standard and Premium version of your goods or service; each version would be a separate Product.
/// They can be used in conjunction with [Prices](https://stripe.com/docs/api#prices) to configure pricing in Payment Links, Checkout, and Subscriptions.
///
/// Related guides: [Set up a subscription](https://stripe.com/docs/billing/subscriptions/set-up-subscription),.
/// [share a Payment Link](https://stripe.com/docs/payment-links),
/// [accept payments with Checkout](https://stripe.com/docs/payments/accept-a-payment#create-product-prices-upfront),.
/// and more about [Products and Prices](https://stripe.com/docs/products-prices/overview)
///
/// For more details see <<https://stripe.com/docs/api/products/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Product {
    /// Whether the product is currently available for purchase.
    pub active: bool,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The ID of the [Price](https://stripe.com/docs/api/prices) object that is the default price for this product.
    pub default_price: Option<stripe_types::Expandable<stripe_shared::Price>>,
    /// The product's description, meant to be displayable to the customer.
    /// Use this field to optionally store a long form explanation of the product being sold for your own rendering purposes.
    pub description: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_shared::ProductId,
    /// A list of up to 8 URLs of images for this product, meant to be displayable to the customer.
    pub images: Vec<String>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// A list of up to 15 marketing features for this product.
    /// These are displayed in [pricing tables](https://stripe.com/docs/payments/checkout/pricing-table).
    pub marketing_features: Vec<stripe_shared::ProductMarketingFeature>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// The product's name, meant to be displayable to the customer.
    pub name: String,
    /// The dimensions of this product for shipping purposes.
    pub package_dimensions: Option<stripe_shared::PackageDimensions>,
    /// Whether this product is shipped (i.e., physical goods).
    pub shippable: Option<bool>,
    /// Extra information about a product which will appear on your customer's credit card statement.
    /// In the case that multiple products are billed at once, the first statement descriptor will be used.
    /// Only used for subscription payments.
    pub statement_descriptor: Option<String>,
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    pub tax_code: Option<stripe_types::Expandable<stripe_shared::TaxCode>>,
    /// The type of the product.
    /// The product is either of type `good`, which is eligible for use with Orders and SKUs, or `service`, which is eligible for use with Subscriptions and Plans.
    #[cfg_attr(feature = "deserialize", serde(rename = "type"))]
    pub type_: stripe_shared::ProductType,
    /// A label that represents units of this product.
    /// When set, this will be included in customers' receipts, invoices, Checkout, and the customer portal.
    pub unit_label: Option<String>,
    /// Time at which the object was last updated. Measured in seconds since the Unix epoch.
    pub updated: stripe_types::Timestamp,
    /// A URL of a publicly-accessible webpage for this product.
    pub url: Option<String>,
}
#[doc(hidden)]
pub struct ProductBuilder {
    active: Option<bool>,
    created: Option<stripe_types::Timestamp>,
    default_price: Option<Option<stripe_types::Expandable<stripe_shared::Price>>>,
    description: Option<Option<String>>,
    id: Option<stripe_shared::ProductId>,
    images: Option<Vec<String>>,
    livemode: Option<bool>,
    marketing_features: Option<Vec<stripe_shared::ProductMarketingFeature>>,
    metadata: Option<std::collections::HashMap<String, String>>,
    name: Option<String>,
    package_dimensions: Option<Option<stripe_shared::PackageDimensions>>,
    shippable: Option<Option<bool>>,
    statement_descriptor: Option<Option<String>>,
    tax_code: Option<Option<stripe_types::Expandable<stripe_shared::TaxCode>>>,
    type_: Option<stripe_shared::ProductType>,
    unit_label: Option<Option<String>>,
    updated: Option<stripe_types::Timestamp>,
    url: Option<Option<String>>,
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

    impl Deserialize for Product {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Product>,
        builder: ProductBuilder,
    }

    impl Visitor for Place<Product> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: ProductBuilder::deser_default() }))
        }
    }

    impl MapBuilder for ProductBuilder {
        type Out = Product;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "active" => Deserialize::begin(&mut self.active),
                "created" => Deserialize::begin(&mut self.created),
                "default_price" => Deserialize::begin(&mut self.default_price),
                "description" => Deserialize::begin(&mut self.description),
                "id" => Deserialize::begin(&mut self.id),
                "images" => Deserialize::begin(&mut self.images),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "marketing_features" => Deserialize::begin(&mut self.marketing_features),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "name" => Deserialize::begin(&mut self.name),
                "package_dimensions" => Deserialize::begin(&mut self.package_dimensions),
                "shippable" => Deserialize::begin(&mut self.shippable),
                "statement_descriptor" => Deserialize::begin(&mut self.statement_descriptor),
                "tax_code" => Deserialize::begin(&mut self.tax_code),
                "type" => Deserialize::begin(&mut self.type_),
                "unit_label" => Deserialize::begin(&mut self.unit_label),
                "updated" => Deserialize::begin(&mut self.updated),
                "url" => Deserialize::begin(&mut self.url),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                active: Deserialize::default(),
                created: Deserialize::default(),
                default_price: Deserialize::default(),
                description: Deserialize::default(),
                id: Deserialize::default(),
                images: Deserialize::default(),
                livemode: Deserialize::default(),
                marketing_features: Deserialize::default(),
                metadata: Deserialize::default(),
                name: Deserialize::default(),
                package_dimensions: Deserialize::default(),
                shippable: Deserialize::default(),
                statement_descriptor: Deserialize::default(),
                tax_code: Deserialize::default(),
                type_: Deserialize::default(),
                unit_label: Deserialize::default(),
                updated: Deserialize::default(),
                url: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(active),
                Some(created),
                Some(default_price),
                Some(description),
                Some(id),
                Some(images),
                Some(livemode),
                Some(marketing_features),
                Some(metadata),
                Some(name),
                Some(package_dimensions),
                Some(shippable),
                Some(statement_descriptor),
                Some(tax_code),
                Some(type_),
                Some(unit_label),
                Some(updated),
                Some(url),
            ) = (
                self.active,
                self.created,
                self.default_price.take(),
                self.description.take(),
                self.id.take(),
                self.images.take(),
                self.livemode,
                self.marketing_features.take(),
                self.metadata.take(),
                self.name.take(),
                self.package_dimensions,
                self.shippable,
                self.statement_descriptor.take(),
                self.tax_code.take(),
                self.type_,
                self.unit_label.take(),
                self.updated,
                self.url.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                active,
                created,
                default_price,
                description,
                id,
                images,
                livemode,
                marketing_features,
                metadata,
                name,
                package_dimensions,
                shippable,
                statement_descriptor,
                tax_code,
                type_,
                unit_label,
                updated,
                url,
            })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for Product {
        type Builder = ProductBuilder;
    }

    impl FromValueOpt for Product {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ProductBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "active" => b.active = FromValueOpt::from_value(v),
                    "created" => b.created = FromValueOpt::from_value(v),
                    "default_price" => b.default_price = FromValueOpt::from_value(v),
                    "description" => b.description = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "images" => b.images = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "marketing_features" => b.marketing_features = FromValueOpt::from_value(v),
                    "metadata" => b.metadata = FromValueOpt::from_value(v),
                    "name" => b.name = FromValueOpt::from_value(v),
                    "package_dimensions" => b.package_dimensions = FromValueOpt::from_value(v),
                    "shippable" => b.shippable = FromValueOpt::from_value(v),
                    "statement_descriptor" => b.statement_descriptor = FromValueOpt::from_value(v),
                    "tax_code" => b.tax_code = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    "unit_label" => b.unit_label = FromValueOpt::from_value(v),
                    "updated" => b.updated = FromValueOpt::from_value(v),
                    "url" => b.url = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for Product {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("Product", 19)?;
        s.serialize_field("active", &self.active)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("default_price", &self.default_price)?;
        s.serialize_field("description", &self.description)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("images", &self.images)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("marketing_features", &self.marketing_features)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("package_dimensions", &self.package_dimensions)?;
        s.serialize_field("shippable", &self.shippable)?;
        s.serialize_field("statement_descriptor", &self.statement_descriptor)?;
        s.serialize_field("tax_code", &self.tax_code)?;
        s.serialize_field("type", &self.type_)?;
        s.serialize_field("unit_label", &self.unit_label)?;
        s.serialize_field("updated", &self.updated)?;
        s.serialize_field("url", &self.url)?;

        s.serialize_field("object", "product")?;
        s.end()
    }
}
impl stripe_types::Object for Product {
    type Id = stripe_shared::ProductId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(ProductId);
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ProductType {
    Good,
    Service,
}
impl ProductType {
    pub fn as_str(self) -> &'static str {
        use ProductType::*;
        match self {
            Good => "good",
            Service => "service",
        }
    }
}

impl std::str::FromStr for ProductType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ProductType::*;
        match s {
            "good" => Ok(Good),
            "service" => Ok(Service),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ProductType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ProductType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ProductType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ProductType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(ProductType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ProductType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ProductType"))
    }
}

/// A Tax Transaction records the tax collected from or refunded to your customer.
///
/// Related guide: [Calculate tax in your custom payment flow](https://stripe.com/docs/tax/custom#tax-transaction).
///
/// For more details see <<https://stripe.com/docs/api/tax/transactions/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxTransaction {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of an existing [Customer](https://stripe.com/docs/api/customers/object) used for the resource.
    pub customer: Option<String>,
    pub customer_details: stripe_misc::TaxProductResourceCustomerDetails,
    /// Unique identifier for the transaction.
    pub id: stripe_misc::TaxTransactionId,
    /// The tax collected or refunded, by line item.
    pub line_items: Option<stripe_types::List<stripe_misc::TaxTransactionLineItem>>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// A custom unique identifier, such as 'myOrder_123'.
    pub reference: String,
    /// If `type=reversal`, contains information about what was reversed.
    pub reversal: Option<stripe_misc::TaxProductResourceTaxTransactionResourceReversal>,
    /// The shipping cost details for the transaction.
    pub shipping_cost: Option<stripe_misc::TaxProductResourceTaxTransactionShippingCost>,
    /// Timestamp of date at which the tax rules and rates in effect applies for the calculation.
    pub tax_date: stripe_types::Timestamp,
    /// If `reversal`, this transaction reverses an earlier transaction.
    #[cfg_attr(feature = "deserialize", serde(rename = "type"))]
    pub type_: TaxTransactionType,
}
#[doc(hidden)]
pub struct TaxTransactionBuilder {
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    customer: Option<Option<String>>,
    customer_details: Option<stripe_misc::TaxProductResourceCustomerDetails>,
    id: Option<stripe_misc::TaxTransactionId>,
    line_items: Option<Option<stripe_types::List<stripe_misc::TaxTransactionLineItem>>>,
    livemode: Option<bool>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    reference: Option<String>,
    reversal: Option<Option<stripe_misc::TaxProductResourceTaxTransactionResourceReversal>>,
    shipping_cost: Option<Option<stripe_misc::TaxProductResourceTaxTransactionShippingCost>>,
    tax_date: Option<stripe_types::Timestamp>,
    type_: Option<TaxTransactionType>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TaxTransaction {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxTransaction>,
        builder: TaxTransactionBuilder,
    }

    impl Visitor for Place<TaxTransaction> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TaxTransactionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TaxTransactionBuilder {
        type Out = TaxTransaction;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "created" => Deserialize::begin(&mut self.created),
                "currency" => Deserialize::begin(&mut self.currency),
                "customer" => Deserialize::begin(&mut self.customer),
                "customer_details" => Deserialize::begin(&mut self.customer_details),
                "id" => Deserialize::begin(&mut self.id),
                "line_items" => Deserialize::begin(&mut self.line_items),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "reference" => Deserialize::begin(&mut self.reference),
                "reversal" => Deserialize::begin(&mut self.reversal),
                "shipping_cost" => Deserialize::begin(&mut self.shipping_cost),
                "tax_date" => Deserialize::begin(&mut self.tax_date),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                created: Deserialize::default(),
                currency: Deserialize::default(),
                customer: Deserialize::default(),
                customer_details: Deserialize::default(),
                id: Deserialize::default(),
                line_items: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                reference: Deserialize::default(),
                reversal: Deserialize::default(),
                shipping_cost: Deserialize::default(),
                tax_date: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                created: self.created?,
                currency: self.currency?,
                customer: self.customer.take()?,
                customer_details: self.customer_details.take()?,
                id: self.id.take()?,
                line_items: self.line_items.take()?,
                livemode: self.livemode?,
                metadata: self.metadata.take()?,
                reference: self.reference.take()?,
                reversal: self.reversal.take()?,
                shipping_cost: self.shipping_cost.take()?,
                tax_date: self.tax_date?,
                type_: self.type_?,
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

    impl ObjectDeser for TaxTransaction {
        type Builder = TaxTransactionBuilder;
    }

    impl FromValueOpt for TaxTransaction {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TaxTransactionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "created" => b.created = Some(FromValueOpt::from_value(v)?),
                    "currency" => b.currency = Some(FromValueOpt::from_value(v)?),
                    "customer" => b.customer = Some(FromValueOpt::from_value(v)?),
                    "customer_details" => b.customer_details = Some(FromValueOpt::from_value(v)?),
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "line_items" => b.line_items = Some(FromValueOpt::from_value(v)?),
                    "livemode" => b.livemode = Some(FromValueOpt::from_value(v)?),
                    "metadata" => b.metadata = Some(FromValueOpt::from_value(v)?),
                    "reference" => b.reference = Some(FromValueOpt::from_value(v)?),
                    "reversal" => b.reversal = Some(FromValueOpt::from_value(v)?),
                    "shipping_cost" => b.shipping_cost = Some(FromValueOpt::from_value(v)?),
                    "tax_date" => b.tax_date = Some(FromValueOpt::from_value(v)?),
                    "type" => b.type_ = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxTransaction {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("TaxTransaction", 14)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("customer", &self.customer)?;
        s.serialize_field("customer_details", &self.customer_details)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("line_items", &self.line_items)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("reference", &self.reference)?;
        s.serialize_field("reversal", &self.reversal)?;
        s.serialize_field("shipping_cost", &self.shipping_cost)?;
        s.serialize_field("tax_date", &self.tax_date)?;
        s.serialize_field("type", &self.type_)?;

        s.serialize_field("object", "tax.transaction")?;
        s.end()
    }
}
/// If `reversal`, this transaction reverses an earlier transaction.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxTransactionType {
    Reversal,
    Transaction,
}
impl TaxTransactionType {
    pub fn as_str(self) -> &'static str {
        use TaxTransactionType::*;
        match self {
            Reversal => "reversal",
            Transaction => "transaction",
        }
    }
}

impl std::str::FromStr for TaxTransactionType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxTransactionType::*;
        match s {
            "reversal" => Ok(Reversal),
            "transaction" => Ok(Transaction),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for TaxTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxTransactionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TaxTransactionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TaxTransactionType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TaxTransactionType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TaxTransactionType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TaxTransactionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TaxTransactionType"))
    }
}
impl stripe_types::Object for TaxTransaction {
    type Id = stripe_misc::TaxTransactionId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(TaxTransactionId);

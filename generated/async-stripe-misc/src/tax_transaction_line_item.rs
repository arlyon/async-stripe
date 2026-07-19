#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxTransactionLineItem {
    /// The line item amount in the [smallest currency unit](https://docs.stripe.com/currencies#minor-units).
    /// If `tax_behavior=inclusive`, then this amount includes taxes.
    /// Otherwise, taxes were calculated on top of this amount.
    pub amount: i64,
    /// The amount of tax calculated for this line item, in the [smallest currency unit](https://docs.stripe.com/currencies#minor-units).
    pub amount_tax: i64,
    /// Unique identifier for the object.
    pub id: stripe_misc::TaxTransactionLineItemId,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The ID of an existing [Product](https://docs.stripe.com/api/products/object).
    pub product: Option<String>,
    /// The number of units of the item being purchased. For reversals, this is the quantity reversed.
    pub quantity: u64,
    /// A custom identifier for this line item in the transaction.
    pub reference: String,
    /// If `type=reversal`, contains information about what was reversed.
    pub reversal: Option<stripe_misc::TaxProductResourceTaxTransactionLineItemResourceReversal>,
    /// Specifies whether the `amount` includes taxes.
    /// If `tax_behavior=inclusive`, then the amount includes taxes.
    pub tax_behavior: TaxTransactionLineItemTaxBehavior,
    /// The [tax code](https://docs.stripe.com/tax/tax-categories) ID used for this resource.
    pub tax_code: String,
    /// If `reversal`, this line item reverses an earlier transaction.
    #[cfg_attr(feature = "deserialize", serde(rename = "type"))]
    pub type_: TaxTransactionLineItemType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxTransactionLineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TaxTransactionLineItem").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TaxTransactionLineItemBuilder {
    amount: Option<i64>,
    amount_tax: Option<i64>,
    id: Option<stripe_misc::TaxTransactionLineItemId>,
    livemode: Option<bool>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    product: Option<Option<String>>,
    quantity: Option<u64>,
    reference: Option<String>,
    reversal: Option<Option<stripe_misc::TaxProductResourceTaxTransactionLineItemResourceReversal>>,
    tax_behavior: Option<TaxTransactionLineItemTaxBehavior>,
    tax_code: Option<String>,
    type_: Option<TaxTransactionLineItemType>,
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

    impl Deserialize for TaxTransactionLineItem {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxTransactionLineItem>,
        builder: TaxTransactionLineItemBuilder,
    }

    impl Visitor for Place<TaxTransactionLineItem> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TaxTransactionLineItemBuilder {
                    amount: Deserialize::default(),
                    amount_tax: Deserialize::default(),
                    id: Deserialize::default(),
                    livemode: Deserialize::default(),
                    metadata: Deserialize::default(),
                    product: Deserialize::default(),
                    quantity: Deserialize::default(),
                    reference: Deserialize::default(),
                    reversal: Deserialize::default(),
                    tax_behavior: Deserialize::default(),
                    tax_code: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.builder.amount),
                "amount_tax" => Deserialize::begin(&mut self.builder.amount_tax),
                "id" => Deserialize::begin(&mut self.builder.id),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "metadata" => Deserialize::begin(&mut self.builder.metadata),
                "product" => Deserialize::begin(&mut self.builder.product),
                "quantity" => Deserialize::begin(&mut self.builder.quantity),
                "reference" => Deserialize::begin(&mut self.builder.reference),
                "reversal" => Deserialize::begin(&mut self.builder.reversal),
                "tax_behavior" => Deserialize::begin(&mut self.builder.tax_behavior),
                "tax_code" => Deserialize::begin(&mut self.builder.tax_code),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(amount),
                Some(amount_tax),
                Some(id),
                Some(livemode),
                Some(metadata),
                Some(product),
                Some(quantity),
                Some(reference),
                Some(reversal),
                Some(tax_behavior),
                Some(tax_code),
                Some(type_),
            ) = (
                self.builder.amount,
                self.builder.amount_tax,
                self.builder.id.take(),
                self.builder.livemode,
                self.builder.metadata.take(),
                self.builder.product.take(),
                self.builder.quantity,
                self.builder.reference.take(),
                self.builder.reversal.take(),
                self.builder.tax_behavior.take(),
                self.builder.tax_code.take(),
                self.builder.type_.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(TaxTransactionLineItem {
                amount,
                amount_tax,
                id,
                livemode,
                metadata,
                product,
                quantity,
                reference,
                reversal,
                tax_behavior,
                tax_code,
                type_,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxTransactionLineItem {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("TaxTransactionLineItem", 13)?;
        s.serialize_field("amount", &self.amount)?;
        s.serialize_field("amount_tax", &self.amount_tax)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("product", &self.product)?;
        s.serialize_field("quantity", &self.quantity)?;
        s.serialize_field("reference", &self.reference)?;
        s.serialize_field("reversal", &self.reversal)?;
        s.serialize_field("tax_behavior", &self.tax_behavior)?;
        s.serialize_field("tax_code", &self.tax_code)?;
        s.serialize_field("type", &self.type_)?;

        s.serialize_field("object", "tax.transaction_line_item")?;
        s.end()
    }
}
/// Specifies whether the `amount` includes taxes.
/// If `tax_behavior=inclusive`, then the amount includes taxes.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TaxTransactionLineItemTaxBehavior {
    Exclusive,
    Inclusive,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TaxTransactionLineItemTaxBehavior {
    pub fn as_str(&self) -> &str {
        use TaxTransactionLineItemTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TaxTransactionLineItemTaxBehavior {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxTransactionLineItemTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TaxTransactionLineItemTaxBehavior"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TaxTransactionLineItemTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TaxTransactionLineItemTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxTransactionLineItemTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TaxTransactionLineItemTaxBehavior)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxTransactionLineItemTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for TaxTransactionLineItemTaxBehavior {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<TaxTransactionLineItemTaxBehavior> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TaxTransactionLineItemTaxBehavior::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TaxTransactionLineItemTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// If `reversal`, this line item reverses an earlier transaction.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TaxTransactionLineItemType {
    Reversal,
    Transaction,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TaxTransactionLineItemType {
    pub fn as_str(&self) -> &str {
        use TaxTransactionLineItemType::*;
        match self {
            Reversal => "reversal",
            Transaction => "transaction",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TaxTransactionLineItemType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxTransactionLineItemType::*;
        match s {
            "reversal" => Ok(Reversal),
            "transaction" => Ok(Transaction),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "TaxTransactionLineItemType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TaxTransactionLineItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TaxTransactionLineItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxTransactionLineItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TaxTransactionLineItemType)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxTransactionLineItemType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for TaxTransactionLineItemType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<TaxTransactionLineItemType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TaxTransactionLineItemType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TaxTransactionLineItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
impl stripe_types::Object for TaxTransactionLineItem {
    type Id = stripe_misc::TaxTransactionLineItemId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(TaxTransactionLineItemId);

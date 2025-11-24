/// The credit note line item object
///
/// For more details see <<https://stripe.com/docs/api/credit_notes/line_item>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CreditNoteLineItem {
    /// The integer amount in cents (or local equivalent) representing the gross amount being credited for this line item, excluding (exclusive) tax and discounts.
    pub amount: i64,
    /// Description of the item being credited.
    pub description: Option<String>,
    /// The integer amount in cents (or local equivalent) representing the discount being credited for this line item.
    pub discount_amount: i64,
    /// The amount of discount calculated per discount for this line item
    pub discount_amounts: Vec<stripe_shared::DiscountsResourceDiscountAmount>,
    /// Unique identifier for the object.
    pub id: stripe_shared::CreditNoteLineItemId,
    /// ID of the invoice line item being credited
    pub invoice_line_item: Option<String>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The pretax credit amounts (ex: discount, credit grants, etc) for this line item.
    pub pretax_credit_amounts: Vec<stripe_shared::CreditNotesPretaxCreditAmount>,
    /// The number of units of product being credited.
    pub quantity: Option<u64>,
    /// The tax rates which apply to the line item.
    pub tax_rates: Vec<stripe_shared::TaxRate>,
    /// The tax information of the line item.
    pub taxes: Option<Vec<stripe_shared::BillingBillResourceInvoicingTaxesTax>>,
    /// The type of the credit note line item, one of `invoice_line_item` or `custom_line_item`.
    /// When the type is `invoice_line_item` there is an additional `invoice_line_item` property on the resource the value of which is the id of the credited line item on the invoice.
    #[cfg_attr(feature = "deserialize", serde(rename = "type"))]
    pub type_: CreditNoteLineItemType,
    /// The cost of each unit of product being credited.
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but contains a decimal value with at most 12 decimal places.
    pub unit_amount_decimal: Option<String>,
}
#[doc(hidden)]
pub struct CreditNoteLineItemBuilder {
    amount: Option<i64>,
    description: Option<Option<String>>,
    discount_amount: Option<i64>,
    discount_amounts: Option<Vec<stripe_shared::DiscountsResourceDiscountAmount>>,
    id: Option<stripe_shared::CreditNoteLineItemId>,
    invoice_line_item: Option<Option<String>>,
    livemode: Option<bool>,
    pretax_credit_amounts: Option<Vec<stripe_shared::CreditNotesPretaxCreditAmount>>,
    quantity: Option<Option<u64>>,
    tax_rates: Option<Vec<stripe_shared::TaxRate>>,
    taxes: Option<Option<Vec<stripe_shared::BillingBillResourceInvoicingTaxesTax>>>,
    type_: Option<CreditNoteLineItemType>,
    unit_amount: Option<Option<i64>>,
    unit_amount_decimal: Option<Option<String>>,
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

    impl Deserialize for CreditNoteLineItem {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CreditNoteLineItem>,
        builder: CreditNoteLineItemBuilder,
    }

    impl Visitor for Place<CreditNoteLineItem> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CreditNoteLineItemBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for CreditNoteLineItemBuilder {
        type Out = CreditNoteLineItem;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "description" => Deserialize::begin(&mut self.description),
                "discount_amount" => Deserialize::begin(&mut self.discount_amount),
                "discount_amounts" => Deserialize::begin(&mut self.discount_amounts),
                "id" => Deserialize::begin(&mut self.id),
                "invoice_line_item" => Deserialize::begin(&mut self.invoice_line_item),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "pretax_credit_amounts" => Deserialize::begin(&mut self.pretax_credit_amounts),
                "quantity" => Deserialize::begin(&mut self.quantity),
                "tax_rates" => Deserialize::begin(&mut self.tax_rates),
                "taxes" => Deserialize::begin(&mut self.taxes),
                "type" => Deserialize::begin(&mut self.type_),
                "unit_amount" => Deserialize::begin(&mut self.unit_amount),
                "unit_amount_decimal" => Deserialize::begin(&mut self.unit_amount_decimal),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                description: Deserialize::default(),
                discount_amount: Deserialize::default(),
                discount_amounts: Deserialize::default(),
                id: Deserialize::default(),
                invoice_line_item: Deserialize::default(),
                livemode: Deserialize::default(),
                pretax_credit_amounts: Deserialize::default(),
                quantity: Deserialize::default(),
                tax_rates: Deserialize::default(),
                taxes: Deserialize::default(),
                type_: Deserialize::default(),
                unit_amount: Deserialize::default(),
                unit_amount_decimal: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(amount),
                Some(description),
                Some(discount_amount),
                Some(discount_amounts),
                Some(id),
                Some(invoice_line_item),
                Some(livemode),
                Some(pretax_credit_amounts),
                Some(quantity),
                Some(tax_rates),
                Some(taxes),
                Some(type_),
                Some(unit_amount),
                Some(unit_amount_decimal),
            ) = (
                self.amount,
                self.description.take(),
                self.discount_amount,
                self.discount_amounts.take(),
                self.id.take(),
                self.invoice_line_item.take(),
                self.livemode,
                self.pretax_credit_amounts.take(),
                self.quantity,
                self.tax_rates.take(),
                self.taxes.take(),
                self.type_.take(),
                self.unit_amount,
                self.unit_amount_decimal.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                amount,
                description,
                discount_amount,
                discount_amounts,
                id,
                invoice_line_item,
                livemode,
                pretax_credit_amounts,
                quantity,
                tax_rates,
                taxes,
                type_,
                unit_amount,
                unit_amount_decimal,
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

    impl ObjectDeser for CreditNoteLineItem {
        type Builder = CreditNoteLineItemBuilder;
    }

    impl FromValueOpt for CreditNoteLineItem {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CreditNoteLineItemBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "description" => b.description = FromValueOpt::from_value(v),
                    "discount_amount" => b.discount_amount = FromValueOpt::from_value(v),
                    "discount_amounts" => b.discount_amounts = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "invoice_line_item" => b.invoice_line_item = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "pretax_credit_amounts" => {
                        b.pretax_credit_amounts = FromValueOpt::from_value(v)
                    }
                    "quantity" => b.quantity = FromValueOpt::from_value(v),
                    "tax_rates" => b.tax_rates = FromValueOpt::from_value(v),
                    "taxes" => b.taxes = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    "unit_amount" => b.unit_amount = FromValueOpt::from_value(v),
                    "unit_amount_decimal" => b.unit_amount_decimal = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for CreditNoteLineItem {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("CreditNoteLineItem", 15)?;
        s.serialize_field("amount", &self.amount)?;
        s.serialize_field("description", &self.description)?;
        s.serialize_field("discount_amount", &self.discount_amount)?;
        s.serialize_field("discount_amounts", &self.discount_amounts)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("invoice_line_item", &self.invoice_line_item)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("pretax_credit_amounts", &self.pretax_credit_amounts)?;
        s.serialize_field("quantity", &self.quantity)?;
        s.serialize_field("tax_rates", &self.tax_rates)?;
        s.serialize_field("taxes", &self.taxes)?;
        s.serialize_field("type", &self.type_)?;
        s.serialize_field("unit_amount", &self.unit_amount)?;
        s.serialize_field("unit_amount_decimal", &self.unit_amount_decimal)?;

        s.serialize_field("object", "credit_note_line_item")?;
        s.end()
    }
}
/// The type of the credit note line item, one of `invoice_line_item` or `custom_line_item`.
/// When the type is `invoice_line_item` there is an additional `invoice_line_item` property on the resource the value of which is the id of the credited line item on the invoice.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreditNoteLineItemType {
    CustomLineItem,
    InvoiceLineItem,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreditNoteLineItemType {
    pub fn as_str(&self) -> &str {
        use CreditNoteLineItemType::*;
        match self {
            CustomLineItem => "custom_line_item",
            InvoiceLineItem => "invoice_line_item",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreditNoteLineItemType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreditNoteLineItemType::*;
        match s {
            "custom_line_item" => Ok(CustomLineItem),
            "invoice_line_item" => Ok(InvoiceLineItem),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "CreditNoteLineItemType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreditNoteLineItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreditNoteLineItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CreditNoteLineItemType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CreditNoteLineItemType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CreditNoteLineItemType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CreditNoteLineItemType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CreditNoteLineItemType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreditNoteLineItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
impl stripe_types::Object for CreditNoteLineItem {
    type Id = stripe_shared::CreditNoteLineItemId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(CreditNoteLineItemId);

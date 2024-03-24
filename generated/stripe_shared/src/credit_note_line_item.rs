/// The credit note line item object
///
/// For more details see <<https://stripe.com/docs/api/credit_notes/line_item>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CreditNoteLineItem {
    /// The integer amount in cents (or local equivalent) representing the gross amount being credited for this line item, excluding (exclusive) tax and discounts.
    pub amount: i64,
    /// The integer amount in cents (or local equivalent) representing the amount being credited for this line item, excluding all tax and discounts.
    pub amount_excluding_tax: Option<i64>,
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
    /// The number of units of product being credited.
    pub quantity: Option<u64>,
    /// The amount of tax calculated per tax rate for this line item
    pub tax_amounts: Vec<stripe_shared::CreditNoteTaxAmount>,
    /// The tax rates which apply to the line item.
    pub tax_rates: Vec<stripe_shared::TaxRate>,
    /// The type of the credit note line item, one of `invoice_line_item` or `custom_line_item`.
    /// When the type is `invoice_line_item` there is an additional `invoice_line_item` property on the resource the value of which is the id of the credited line item on the invoice.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: CreditNoteLineItemType,
    /// The cost of each unit of product being credited.
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but contains a decimal value with at most 12 decimal places.
    pub unit_amount_decimal: Option<String>,
    /// The amount in cents (or local equivalent) representing the unit amount being credited for this line item, excluding all tax and discounts.
    pub unit_amount_excluding_tax: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct CreditNoteLineItemBuilder {
    amount: Option<i64>,
    amount_excluding_tax: Option<Option<i64>>,
    description: Option<Option<String>>,
    discount_amount: Option<i64>,
    discount_amounts: Option<Vec<stripe_shared::DiscountsResourceDiscountAmount>>,
    id: Option<stripe_shared::CreditNoteLineItemId>,
    invoice_line_item: Option<Option<String>>,
    livemode: Option<bool>,
    quantity: Option<Option<u64>>,
    tax_amounts: Option<Vec<stripe_shared::CreditNoteTaxAmount>>,
    tax_rates: Option<Vec<stripe_shared::TaxRate>>,
    type_: Option<CreditNoteLineItemType>,
    unit_amount: Option<Option<i64>>,
    unit_amount_decimal: Option<Option<String>>,
    unit_amount_excluding_tax: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
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
            Ok(Box::new(Builder { out: &mut self.out, builder: CreditNoteLineItemBuilder::deser_default() }))
        }
    }

    impl MapBuilder for CreditNoteLineItemBuilder {
        type Out = CreditNoteLineItem;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "amount_excluding_tax" => Deserialize::begin(&mut self.amount_excluding_tax),
                "description" => Deserialize::begin(&mut self.description),
                "discount_amount" => Deserialize::begin(&mut self.discount_amount),
                "discount_amounts" => Deserialize::begin(&mut self.discount_amounts),
                "id" => Deserialize::begin(&mut self.id),
                "invoice_line_item" => Deserialize::begin(&mut self.invoice_line_item),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "quantity" => Deserialize::begin(&mut self.quantity),
                "tax_amounts" => Deserialize::begin(&mut self.tax_amounts),
                "tax_rates" => Deserialize::begin(&mut self.tax_rates),
                "type" => Deserialize::begin(&mut self.type_),
                "unit_amount" => Deserialize::begin(&mut self.unit_amount),
                "unit_amount_decimal" => Deserialize::begin(&mut self.unit_amount_decimal),
                "unit_amount_excluding_tax" => Deserialize::begin(&mut self.unit_amount_excluding_tax),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                amount_excluding_tax: Deserialize::default(),
                description: Deserialize::default(),
                discount_amount: Deserialize::default(),
                discount_amounts: Deserialize::default(),
                id: Deserialize::default(),
                invoice_line_item: Deserialize::default(),
                livemode: Deserialize::default(),
                quantity: Deserialize::default(),
                tax_amounts: Deserialize::default(),
                tax_rates: Deserialize::default(),
                type_: Deserialize::default(),
                unit_amount: Deserialize::default(),
                unit_amount_decimal: Deserialize::default(),
                unit_amount_excluding_tax: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount = self.amount.take()?;
            let amount_excluding_tax = self.amount_excluding_tax.take()?;
            let description = self.description.take()?;
            let discount_amount = self.discount_amount.take()?;
            let discount_amounts = self.discount_amounts.take()?;
            let id = self.id.take()?;
            let invoice_line_item = self.invoice_line_item.take()?;
            let livemode = self.livemode.take()?;
            let quantity = self.quantity.take()?;
            let tax_amounts = self.tax_amounts.take()?;
            let tax_rates = self.tax_rates.take()?;
            let type_ = self.type_.take()?;
            let unit_amount = self.unit_amount.take()?;
            let unit_amount_decimal = self.unit_amount_decimal.take()?;
            let unit_amount_excluding_tax = self.unit_amount_excluding_tax.take()?;

            Some(Self::Out {
                amount,
                amount_excluding_tax,
                description,
                discount_amount,
                discount_amounts,
                id,
                invoice_line_item,
                livemode,
                quantity,
                tax_amounts,
                tax_rates,
                type_,
                unit_amount,
                unit_amount_decimal,
                unit_amount_excluding_tax,
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
                    "amount" => b.amount = Some(FromValueOpt::from_value(v)?),
                    "amount_excluding_tax" => b.amount_excluding_tax = Some(FromValueOpt::from_value(v)?),
                    "description" => b.description = Some(FromValueOpt::from_value(v)?),
                    "discount_amount" => b.discount_amount = Some(FromValueOpt::from_value(v)?),
                    "discount_amounts" => b.discount_amounts = Some(FromValueOpt::from_value(v)?),
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "invoice_line_item" => b.invoice_line_item = Some(FromValueOpt::from_value(v)?),
                    "livemode" => b.livemode = Some(FromValueOpt::from_value(v)?),
                    "quantity" => b.quantity = Some(FromValueOpt::from_value(v)?),
                    "tax_amounts" => b.tax_amounts = Some(FromValueOpt::from_value(v)?),
                    "tax_rates" => b.tax_rates = Some(FromValueOpt::from_value(v)?),
                    "type" => b.type_ = Some(FromValueOpt::from_value(v)?),
                    "unit_amount" => b.unit_amount = Some(FromValueOpt::from_value(v)?),
                    "unit_amount_decimal" => b.unit_amount_decimal = Some(FromValueOpt::from_value(v)?),
                    "unit_amount_excluding_tax" => b.unit_amount_excluding_tax = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The type of the credit note line item, one of `invoice_line_item` or `custom_line_item`.
/// When the type is `invoice_line_item` there is an additional `invoice_line_item` property on the resource the value of which is the id of the credited line item on the invoice.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreditNoteLineItemType {
    CustomLineItem,
    InvoiceLineItem,
}
impl CreditNoteLineItemType {
    pub fn as_str(self) -> &'static str {
        use CreditNoteLineItemType::*;
        match self {
            CustomLineItem => "custom_line_item",
            InvoiceLineItem => "invoice_line_item",
        }
    }
}

impl std::str::FromStr for CreditNoteLineItemType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreditNoteLineItemType::*;
        match s {
            "custom_line_item" => Ok(CustomLineItem),
            "invoice_line_item" => Ok(InvoiceLineItem),
            _ => Err(()),
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
impl serde::Serialize for CreditNoteLineItemType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CreditNoteLineItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreditNoteLineItemType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CreditNoteLineItemType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<CreditNoteLineItemType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CreditNoteLineItemType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

#[cfg(feature = "min-ser")]
stripe_types::impl_from_val_with_from_str!(CreditNoteLineItemType);
impl stripe_types::Object for CreditNoteLineItem {
    type Id = stripe_shared::CreditNoteLineItemId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(CreditNoteLineItemId, "cnli_");

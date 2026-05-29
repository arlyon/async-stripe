#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoiceItemProrationCreditedItems {
    /// When `type` is `invoice_item`, the invoice item id for the debited invoice item corresponding to this credit proration.
    pub invoice_item: Option<String>,
    pub invoice_line_item_details: Option<stripe_billing::CreditedItemsInvoiceLineItems>,
    /// Whether the credit references a pending invoice item or one or more invoice line items on an invoice.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: InvoiceItemProrationCreditedItemsType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoiceItemProrationCreditedItems {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InvoiceItemProrationCreditedItems").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InvoiceItemProrationCreditedItemsBuilder {
    invoice_item: Option<Option<String>>,
    invoice_line_item_details: Option<Option<stripe_billing::CreditedItemsInvoiceLineItems>>,
    type_: Option<InvoiceItemProrationCreditedItemsType>,
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

    impl Deserialize for InvoiceItemProrationCreditedItems {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoiceItemProrationCreditedItems>,
        builder: InvoiceItemProrationCreditedItemsBuilder,
    }

    impl Visitor for Place<InvoiceItemProrationCreditedItems> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InvoiceItemProrationCreditedItemsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InvoiceItemProrationCreditedItemsBuilder {
        type Out = InvoiceItemProrationCreditedItems;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "invoice_item" => Deserialize::begin(&mut self.invoice_item),
                "invoice_line_item_details" => {
                    Deserialize::begin(&mut self.invoice_line_item_details)
                }
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { invoice_item: Some(None), invoice_line_item_details: Some(None), type_: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(invoice_item), Some(invoice_line_item_details), Some(type_)) = (
                self.invoice_item.take(),
                self.invoice_line_item_details.take(),
                self.type_.take(),
            ) else {
                return None;
            };
            Some(Self::Out { invoice_item, invoice_line_item_details, type_ })
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

    impl ObjectDeser for InvoiceItemProrationCreditedItems {
        type Builder = InvoiceItemProrationCreditedItemsBuilder;
    }

    impl FromValueOpt for InvoiceItemProrationCreditedItems {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InvoiceItemProrationCreditedItemsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "invoice_item" => b.invoice_item = FromValueOpt::from_value(v),
                    "invoice_line_item_details" => {
                        b.invoice_line_item_details = FromValueOpt::from_value(v)
                    }
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Whether the credit references a pending invoice item or one or more invoice line items on an invoice.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum InvoiceItemProrationCreditedItemsType {
    InvoiceItem,
    InvoiceLineItems,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl InvoiceItemProrationCreditedItemsType {
    pub fn as_str(&self) -> &str {
        use InvoiceItemProrationCreditedItemsType::*;
        match self {
            InvoiceItem => "invoice_item",
            InvoiceLineItems => "invoice_line_items",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for InvoiceItemProrationCreditedItemsType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InvoiceItemProrationCreditedItemsType::*;
        match s {
            "invoice_item" => Ok(InvoiceItem),
            "invoice_line_items" => Ok(InvoiceLineItems),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "InvoiceItemProrationCreditedItemsType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for InvoiceItemProrationCreditedItemsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for InvoiceItemProrationCreditedItemsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoiceItemProrationCreditedItemsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(InvoiceItemProrationCreditedItemsType)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for InvoiceItemProrationCreditedItemsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for InvoiceItemProrationCreditedItemsType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<InvoiceItemProrationCreditedItemsType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(InvoiceItemProrationCreditedItemsType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(InvoiceItemProrationCreditedItemsType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for InvoiceItemProrationCreditedItemsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}

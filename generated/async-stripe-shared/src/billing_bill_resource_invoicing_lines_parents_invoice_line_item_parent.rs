#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingBillResourceInvoicingLinesParentsInvoiceLineItemParent {
    /// Details about the invoice item that generated this line item
pub invoice_item_details: Option<stripe_shared::BillingBillResourceInvoicingLinesParentsInvoiceLineItemInvoiceItemParent>,
    /// Details about the subscription item that generated this line item
pub subscription_item_details: Option<stripe_shared::BillingBillResourceInvoicingLinesParentsInvoiceLineItemSubscriptionItemParent>,
    /// The type of parent that generated this line item
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
pub type_: BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType,

}
#[doc(hidden)]
pub struct BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentBuilder {
    invoice_item_details: Option<Option<stripe_shared::BillingBillResourceInvoicingLinesParentsInvoiceLineItemInvoiceItemParent>>,
subscription_item_details: Option<Option<stripe_shared::BillingBillResourceInvoicingLinesParentsInvoiceLineItemSubscriptionItemParent>>,
type_: Option<BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType>,

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

    impl Deserialize for BillingBillResourceInvoicingLinesParentsInvoiceLineItemParent {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BillingBillResourceInvoicingLinesParentsInvoiceLineItemParent>,
        builder: BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentBuilder,
    }

    impl Visitor for Place<BillingBillResourceInvoicingLinesParentsInvoiceLineItemParent> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentBuilder {
        type Out = BillingBillResourceInvoicingLinesParentsInvoiceLineItemParent;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "invoice_item_details" => Deserialize::begin(&mut self.invoice_item_details),
                "subscription_item_details" => {
                    Deserialize::begin(&mut self.subscription_item_details)
                }
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                invoice_item_details: Deserialize::default(),
                subscription_item_details: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(invoice_item_details), Some(subscription_item_details), Some(type_)) = (
                self.invoice_item_details.take(),
                self.subscription_item_details.take(),
                self.type_,
            ) else {
                return None;
            };
            Some(Self::Out { invoice_item_details, subscription_item_details, type_ })
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

    impl ObjectDeser for BillingBillResourceInvoicingLinesParentsInvoiceLineItemParent {
        type Builder = BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentBuilder;
    }

    impl FromValueOpt for BillingBillResourceInvoicingLinesParentsInvoiceLineItemParent {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentBuilder::deser_default(
                );
            for (k, v) in obj {
                match k.as_str() {
                    "invoice_item_details" => b.invoice_item_details = FromValueOpt::from_value(v),
                    "subscription_item_details" => {
                        b.subscription_item_details = FromValueOpt::from_value(v)
                    }
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The type of parent that generated this line item
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType {
    InvoiceItemDetails,
    SubscriptionItemDetails,
}
impl BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType {
    pub fn as_str(self) -> &'static str {
        use BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType::*;
        match self {
            InvoiceItemDetails => "invoice_item_details",
            SubscriptionItemDetails => "subscription_item_details",
        }
    }
}

impl std::str::FromStr for BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType::*;
        match s {
            "invoice_item_details" => Ok(InvoiceItemDetails),
            "subscription_item_details" => Ok(SubscriptionItemDetails),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType"))
    }
}

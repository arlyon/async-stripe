#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingBillResourceInvoicingLinesParentsInvoiceLineItemParent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BillingBillResourceInvoicingLinesParentsInvoiceLineItemParent")
            .finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentBuilder {
                    invoice_item_details: Deserialize::default(),
                    subscription_item_details: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "invoice_item_details" => {
                    Deserialize::begin(&mut self.builder.invoice_item_details)
                }
                "subscription_item_details" => {
                    Deserialize::begin(&mut self.builder.subscription_item_details)
                }
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(invoice_item_details), Some(subscription_item_details), Some(type_)) = (
                self.builder.invoice_item_details.take(),
                self.builder.subscription_item_details.take(),
                self.builder.type_.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(BillingBillResourceInvoicingLinesParentsInvoiceLineItemParent {
                invoice_item_details,
                subscription_item_details,
                type_,
            });
            Ok(())
        }
    }
};
/// The type of parent that generated this line item
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType {
    InvoiceItemDetails,
    SubscriptionItemDetails,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType {
    pub fn as_str(&self) -> &str {
        use BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType::*;
        match self {
            InvoiceItemDetails => "invoice_item_details",
            SubscriptionItemDetails => "subscription_item_details",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType::*;
        match s {
            "invoice_item_details" => Ok(InvoiceItemDetails),
            "subscription_item_details" => Ok(SubscriptionItemDetails),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(
            BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType
        ))
        .finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}

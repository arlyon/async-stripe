#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingBillResourceInvoicingParentsInvoiceParent {
    /// Details about the quote that generated this invoice
    pub quote_details: Option<stripe_shared::BillingBillResourceInvoicingParentsInvoiceQuoteParent>,
    /// Details about the subscription that generated this invoice
    pub subscription_details:
        Option<stripe_shared::BillingBillResourceInvoicingParentsInvoiceSubscriptionParent>,
    /// The type of parent that generated this invoice
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: BillingBillResourceInvoicingParentsInvoiceParentType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingBillResourceInvoicingParentsInvoiceParent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BillingBillResourceInvoicingParentsInvoiceParent").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BillingBillResourceInvoicingParentsInvoiceParentBuilder {
    quote_details:
        Option<Option<stripe_shared::BillingBillResourceInvoicingParentsInvoiceQuoteParent>>,
    subscription_details:
        Option<Option<stripe_shared::BillingBillResourceInvoicingParentsInvoiceSubscriptionParent>>,
    type_: Option<BillingBillResourceInvoicingParentsInvoiceParentType>,
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

    impl Deserialize for BillingBillResourceInvoicingParentsInvoiceParent {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BillingBillResourceInvoicingParentsInvoiceParent>,
        builder: BillingBillResourceInvoicingParentsInvoiceParentBuilder,
    }

    impl Visitor for Place<BillingBillResourceInvoicingParentsInvoiceParent> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BillingBillResourceInvoicingParentsInvoiceParentBuilder {
                    quote_details: Deserialize::default(),
                    subscription_details: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "quote_details" => Deserialize::begin(&mut self.builder.quote_details),
                "subscription_details" => {
                    Deserialize::begin(&mut self.builder.subscription_details)
                }
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(quote_details), Some(subscription_details), Some(type_)) = (
                self.builder.quote_details.take(),
                self.builder.subscription_details.take(),
                self.builder.type_.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(BillingBillResourceInvoicingParentsInvoiceParent {
                quote_details,
                subscription_details,
                type_,
            });
            Ok(())
        }
    }
};
/// The type of parent that generated this invoice
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum BillingBillResourceInvoicingParentsInvoiceParentType {
    QuoteDetails,
    SubscriptionDetails,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl BillingBillResourceInvoicingParentsInvoiceParentType {
    pub fn as_str(&self) -> &str {
        use BillingBillResourceInvoicingParentsInvoiceParentType::*;
        match self {
            QuoteDetails => "quote_details",
            SubscriptionDetails => "subscription_details",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for BillingBillResourceInvoicingParentsInvoiceParentType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BillingBillResourceInvoicingParentsInvoiceParentType::*;
        match s {
            "quote_details" => Ok(QuoteDetails),
            "subscription_details" => Ok(SubscriptionDetails),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "BillingBillResourceInvoicingParentsInvoiceParentType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for BillingBillResourceInvoicingParentsInvoiceParentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for BillingBillResourceInvoicingParentsInvoiceParentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingBillResourceInvoicingParentsInvoiceParentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(BillingBillResourceInvoicingParentsInvoiceParentType))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for BillingBillResourceInvoicingParentsInvoiceParentType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for BillingBillResourceInvoicingParentsInvoiceParentType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<BillingBillResourceInvoicingParentsInvoiceParentType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            BillingBillResourceInvoicingParentsInvoiceParentType::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BillingBillResourceInvoicingParentsInvoiceParentType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}

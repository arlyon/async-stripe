#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionCheckoutAddressDetails {
    pub address: stripe_shared::Address,
    /// Customer name.
    pub name: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentPagesCheckoutSessionCheckoutAddressDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentPagesCheckoutSessionCheckoutAddressDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentPagesCheckoutSessionCheckoutAddressDetailsBuilder {
    address: Option<stripe_shared::Address>,
    name: Option<String>,
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

    impl Deserialize for PaymentPagesCheckoutSessionCheckoutAddressDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesCheckoutSessionCheckoutAddressDetails>,
        builder: PaymentPagesCheckoutSessionCheckoutAddressDetailsBuilder,
    }

    impl Visitor for Place<PaymentPagesCheckoutSessionCheckoutAddressDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentPagesCheckoutSessionCheckoutAddressDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentPagesCheckoutSessionCheckoutAddressDetailsBuilder {
        type Out = PaymentPagesCheckoutSessionCheckoutAddressDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "address" => Deserialize::begin(&mut self.address),
                "name" => Deserialize::begin(&mut self.name),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { address: None, name: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self { address: Some(address), name: Some(name) } => {
                    Some(Self::Out { address, name })
                }
                builder => {
                    *self = builder;
                    None
                }
            }
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for PaymentPagesCheckoutSessionCheckoutAddressDetails {
        type Builder = PaymentPagesCheckoutSessionCheckoutAddressDetailsBuilder;
    }

    impl FromValueOpt for PaymentPagesCheckoutSessionCheckoutAddressDetails {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionCheckoutAddressDetails {
    pub address: stripe_shared::Address,
    /// Customer name.
    pub name: String,
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
            Self { address: Deserialize::default(), name: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(address), Some(name)) = (self.address.take(), self.name.take()) else {
                return None;
            };
            Some(Self::Out { address, name })
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

    impl ObjectDeser for PaymentPagesCheckoutSessionCheckoutAddressDetails {
        type Builder = PaymentPagesCheckoutSessionCheckoutAddressDetailsBuilder;
    }

    impl FromValueOpt for PaymentPagesCheckoutSessionCheckoutAddressDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentPagesCheckoutSessionCheckoutAddressDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "address" => b.address = FromValueOpt::from_value(v),
                    "name" => b.name = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};

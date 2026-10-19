#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentLinksResourceShippingAddressCollection {
    /// An array of two-letter ISO country codes representing which countries Checkout should provide as options for shipping locations.
    /// Unsupported country codes: `AS, CX, CC, CU, HM, IR, KP, MH, FM, NF, MP, PW, SD, SY, UM, VI`.
    pub allowed_countries: Vec<stripe_shared::AllowedCountries>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentLinksResourceShippingAddressCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentLinksResourceShippingAddressCollection").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentLinksResourceShippingAddressCollectionBuilder {
    allowed_countries: Option<Vec<stripe_shared::AllowedCountries>>,
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

    impl Deserialize for PaymentLinksResourceShippingAddressCollection {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLinksResourceShippingAddressCollection>,
        builder: PaymentLinksResourceShippingAddressCollectionBuilder,
    }

    impl Visitor for Place<PaymentLinksResourceShippingAddressCollection> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentLinksResourceShippingAddressCollectionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentLinksResourceShippingAddressCollectionBuilder {
        type Out = PaymentLinksResourceShippingAddressCollection;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "allowed_countries" => Deserialize::begin(&mut self.allowed_countries),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { allowed_countries: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(allowed_countries),) = (self.allowed_countries.take(),) else {
                return None;
            };
            Some(Self::Out { allowed_countries })
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

    impl ObjectDeser for PaymentLinksResourceShippingAddressCollection {
        type Builder = PaymentLinksResourceShippingAddressCollectionBuilder;
    }

    impl FromValueOpt for PaymentLinksResourceShippingAddressCollection {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentLinksResourceShippingAddressCollectionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "allowed_countries" => b.allowed_countries = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// An array of two-letter ISO country codes representing which countries Checkout should provide as options for shipping locations.
/// Unsupported country codes: `AS, CX, CC, CU, HM, IR, KP, MH, FM, NF, MP, PW, SD, SY, UM, VI`.
pub type PaymentLinksResourceShippingAddressCollectionAllowedCountries =
    stripe_shared::AllowedCountries;

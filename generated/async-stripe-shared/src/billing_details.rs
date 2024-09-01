#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingDetails {
    /// Billing address.
    pub address: Option<stripe_shared::Address>,
    /// Email address.
    pub email: Option<String>,
    /// Full name.
    pub name: Option<String>,
    /// Billing phone number (including extension).
    pub phone: Option<String>,
}
#[doc(hidden)]
pub struct BillingDetailsBuilder {
    address: Option<Option<stripe_shared::Address>>,
    email: Option<Option<String>>,
    name: Option<Option<String>>,
    phone: Option<Option<String>>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for BillingDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BillingDetails>,
        builder: BillingDetailsBuilder,
    }

    impl Visitor for Place<BillingDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BillingDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BillingDetailsBuilder {
        type Out = BillingDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "address" => Deserialize::begin(&mut self.address),
                "email" => Deserialize::begin(&mut self.email),
                "name" => Deserialize::begin(&mut self.name),
                "phone" => Deserialize::begin(&mut self.phone),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                address: Deserialize::default(),
                email: Deserialize::default(),
                name: Deserialize::default(),
                phone: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(address), Some(email), Some(name), Some(phone)) =
                (self.address.take(), self.email.take(), self.name.take(), self.phone.take())
            else {
                return None;
            };
            Some(Self::Out { address, email, name, phone })
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

    impl ObjectDeser for BillingDetails {
        type Builder = BillingDetailsBuilder;
    }

    impl FromValueOpt for BillingDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BillingDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "address" => b.address = FromValueOpt::from_value(v),
                    "email" => b.email = FromValueOpt::from_value(v),
                    "name" => b.name = FromValueOpt::from_value(v),
                    "phone" => b.phone = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};

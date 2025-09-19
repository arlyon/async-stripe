#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodLink {
    /// Account owner's email address.
    pub email: Option<String>,
    /// \[Deprecated\] This is a legacy parameter that no longer has any function.
    pub persistent_token: Option<String>,
}
#[doc(hidden)]
pub struct PaymentMethodLinkBuilder {
    email: Option<Option<String>>,
    persistent_token: Option<Option<String>>,
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

    impl Deserialize for PaymentMethodLink {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodLink>,
        builder: PaymentMethodLinkBuilder,
    }

    impl Visitor for Place<PaymentMethodLink> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodLinkBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodLinkBuilder {
        type Out = PaymentMethodLink;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "email" => Deserialize::begin(&mut self.email),
                "persistent_token" => Deserialize::begin(&mut self.persistent_token),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { email: Deserialize::default(), persistent_token: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(email), Some(persistent_token)) =
                (self.email.take(), self.persistent_token.take())
            else {
                return None;
            };
            Some(Self::Out { email, persistent_token })
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

    impl ObjectDeser for PaymentMethodLink {
        type Builder = PaymentMethodLinkBuilder;
    }

    impl FromValueOpt for PaymentMethodLink {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodLinkBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "email" => b.email = FromValueOpt::from_value(v),
                    "persistent_token" => b.persistent_token = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};

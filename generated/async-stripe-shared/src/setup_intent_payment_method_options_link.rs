#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SetupIntentPaymentMethodOptionsLink {
    /// \[Deprecated\] This is a legacy parameter that no longer has any function.
    pub persistent_token: Option<String>,
}
#[doc(hidden)]
pub struct SetupIntentPaymentMethodOptionsLinkBuilder {
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

    impl Deserialize for SetupIntentPaymentMethodOptionsLink {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SetupIntentPaymentMethodOptionsLink>,
        builder: SetupIntentPaymentMethodOptionsLinkBuilder,
    }

    impl Visitor for Place<SetupIntentPaymentMethodOptionsLink> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SetupIntentPaymentMethodOptionsLinkBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SetupIntentPaymentMethodOptionsLinkBuilder {
        type Out = SetupIntentPaymentMethodOptionsLink;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "persistent_token" => Deserialize::begin(&mut self.persistent_token),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { persistent_token: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(persistent_token),) = (self.persistent_token.take(),) else {
                return None;
            };
            Some(Self::Out { persistent_token })
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

    impl ObjectDeser for SetupIntentPaymentMethodOptionsLink {
        type Builder = SetupIntentPaymentMethodOptionsLinkBuilder;
    }

    impl FromValueOpt for SetupIntentPaymentMethodOptionsLink {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SetupIntentPaymentMethodOptionsLinkBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "persistent_token" => b.persistent_token = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};

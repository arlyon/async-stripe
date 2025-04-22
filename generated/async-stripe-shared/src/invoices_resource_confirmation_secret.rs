#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoicesResourceConfirmationSecret {
    /// The client_secret of the payment that Stripe creates for the invoice after finalization.
    pub client_secret: String,
    /// The type of client_secret.
    /// Currently this is always payment_intent, referencing the default payment_intent that Stripe creates during invoice finalization.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: String,
}
#[doc(hidden)]
pub struct InvoicesResourceConfirmationSecretBuilder {
    client_secret: Option<String>,
    type_: Option<String>,
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

    impl Deserialize for InvoicesResourceConfirmationSecret {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoicesResourceConfirmationSecret>,
        builder: InvoicesResourceConfirmationSecretBuilder,
    }

    impl Visitor for Place<InvoicesResourceConfirmationSecret> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InvoicesResourceConfirmationSecretBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InvoicesResourceConfirmationSecretBuilder {
        type Out = InvoicesResourceConfirmationSecret;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "client_secret" => Deserialize::begin(&mut self.client_secret),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { client_secret: Deserialize::default(), type_: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(client_secret), Some(type_)) = (self.client_secret.take(), self.type_.take())
            else {
                return None;
            };
            Some(Self::Out { client_secret, type_ })
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

    impl ObjectDeser for InvoicesResourceConfirmationSecret {
        type Builder = InvoicesResourceConfirmationSecretBuilder;
    }

    impl FromValueOpt for InvoicesResourceConfirmationSecret {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InvoicesResourceConfirmationSecretBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "client_secret" => b.client_secret = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoicesResourceConfirmationSecret {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InvoicesResourceConfirmationSecret").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InvoicesResourceConfirmationSecretBuilder {
    client_secret: Option<String>,
    type_: Option<String>,
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
                builder: InvoicesResourceConfirmationSecretBuilder {
                    client_secret: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "client_secret" => Deserialize::begin(&mut self.builder.client_secret),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(client_secret), Some(type_)) =
                (self.builder.client_secret.take(), self.builder.type_.take())
            else {
                return Ok(());
            };
            *self.out = Some(InvoicesResourceConfirmationSecret { client_secret, type_ });
            Ok(())
        }
    }
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct RefundNextActionDisplayDetails {
    pub email_sent: stripe_shared::EmailSent,
    /// The expiry timestamp.
    pub expires_at: stripe_types::Timestamp,
}
#[doc(hidden)]
pub struct RefundNextActionDisplayDetailsBuilder {
    email_sent: Option<stripe_shared::EmailSent>,
    expires_at: Option<stripe_types::Timestamp>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
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

    impl Deserialize for RefundNextActionDisplayDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<RefundNextActionDisplayDetails>,
        builder: RefundNextActionDisplayDetailsBuilder,
    }

    impl Visitor for Place<RefundNextActionDisplayDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: RefundNextActionDisplayDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for RefundNextActionDisplayDetailsBuilder {
        type Out = RefundNextActionDisplayDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "email_sent" => Deserialize::begin(&mut self.email_sent),
                "expires_at" => Deserialize::begin(&mut self.expires_at),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { email_sent: Deserialize::default(), expires_at: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(email_sent), Some(expires_at)) = (self.email_sent.take(), self.expires_at)
            else {
                return None;
            };
            Some(Self::Out { email_sent, expires_at })
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

    impl ObjectDeser for RefundNextActionDisplayDetails {
        type Builder = RefundNextActionDisplayDetailsBuilder;
    }

    impl FromValueOpt for RefundNextActionDisplayDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = RefundNextActionDisplayDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "email_sent" => b.email_sent = FromValueOpt::from_value(v),
                    "expires_at" => b.expires_at = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};

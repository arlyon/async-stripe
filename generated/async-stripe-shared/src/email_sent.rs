#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct EmailSent {
    /// The timestamp when the email was sent.
    pub email_sent_at: stripe_types::Timestamp,
    /// The recipient's email address.
    pub email_sent_to: String,
}
#[doc(hidden)]
pub struct EmailSentBuilder {
    email_sent_at: Option<stripe_types::Timestamp>,
    email_sent_to: Option<String>,
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

    impl Deserialize for EmailSent {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<EmailSent>,
        builder: EmailSentBuilder,
    }

    impl Visitor for Place<EmailSent> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: EmailSentBuilder::deser_default() }))
        }
    }

    impl MapBuilder for EmailSentBuilder {
        type Out = EmailSent;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "email_sent_at" => Deserialize::begin(&mut self.email_sent_at),
                "email_sent_to" => Deserialize::begin(&mut self.email_sent_to),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { email_sent_at: Deserialize::default(), email_sent_to: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(email_sent_at), Some(email_sent_to)) =
                (self.email_sent_at, self.email_sent_to.take())
            else {
                return None;
            };
            Some(Self::Out { email_sent_at, email_sent_to })
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

    impl ObjectDeser for EmailSent {
        type Builder = EmailSentBuilder;
    }

    impl FromValueOpt for EmailSent {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = EmailSentBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "email_sent_at" => b.email_sent_at = FromValueOpt::from_value(v),
                    "email_sent_to" => b.email_sent_to = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};

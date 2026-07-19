#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct EmailSent {
    /// The timestamp when the email was sent.
    pub email_sent_at: stripe_types::Timestamp,
    /// The recipient's email address.
    pub email_sent_to: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for EmailSent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("EmailSent").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct EmailSentBuilder {
    email_sent_at: Option<stripe_types::Timestamp>,
    email_sent_to: Option<String>,
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
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: EmailSentBuilder {
                    email_sent_at: Deserialize::default(),
                    email_sent_to: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "email_sent_at" => Deserialize::begin(&mut self.builder.email_sent_at),
                "email_sent_to" => Deserialize::begin(&mut self.builder.email_sent_to),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(email_sent_at), Some(email_sent_to)) =
                (self.builder.email_sent_at, self.builder.email_sent_to.take())
            else {
                return Ok(());
            };
            *self.out = Some(EmailSent { email_sent_at, email_sent_to });
            Ok(())
        }
    }
};

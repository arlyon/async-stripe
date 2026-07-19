#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct RefundNextActionDisplayDetails {
    pub email_sent: stripe_shared::EmailSent,
    /// The expiry timestamp.
    pub expires_at: stripe_types::Timestamp,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RefundNextActionDisplayDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RefundNextActionDisplayDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct RefundNextActionDisplayDetailsBuilder {
    email_sent: Option<stripe_shared::EmailSent>,
    expires_at: Option<stripe_types::Timestamp>,
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
                builder: RefundNextActionDisplayDetailsBuilder {
                    email_sent: Deserialize::default(),
                    expires_at: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "email_sent" => Deserialize::begin(&mut self.builder.email_sent),
                "expires_at" => Deserialize::begin(&mut self.builder.expires_at),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(email_sent), Some(expires_at)) =
                (self.builder.email_sent.take(), self.builder.expires_at)
            else {
                return Ok(());
            };
            *self.out = Some(RefundNextActionDisplayDetails { email_sent, expires_at });
            Ok(())
        }
    }
};

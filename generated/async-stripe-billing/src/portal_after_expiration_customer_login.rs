#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PortalAfterExpirationCustomerLogin {
    /// The time after which the customer can no longer recover this session.
    pub expires_at: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PortalAfterExpirationCustomerLogin {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PortalAfterExpirationCustomerLogin").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PortalAfterExpirationCustomerLoginBuilder {
    expires_at: Option<Option<stripe_types::Timestamp>>,
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

    impl Deserialize for PortalAfterExpirationCustomerLogin {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalAfterExpirationCustomerLogin>,
        builder: PortalAfterExpirationCustomerLoginBuilder,
    }

    impl Visitor for Place<PortalAfterExpirationCustomerLogin> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PortalAfterExpirationCustomerLoginBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PortalAfterExpirationCustomerLoginBuilder {
        type Out = PortalAfterExpirationCustomerLogin;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "expires_at" => Deserialize::begin(&mut self.expires_at),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { expires_at: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(expires_at),) = (self.expires_at,) else {
                return None;
            };
            Some(Self::Out { expires_at })
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

    impl ObjectDeser for PortalAfterExpirationCustomerLogin {
        type Builder = PortalAfterExpirationCustomerLoginBuilder;
    }

    impl FromValueOpt for PortalAfterExpirationCustomerLogin {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PortalAfterExpirationCustomerLoginBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "expires_at" => b.expires_at = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};

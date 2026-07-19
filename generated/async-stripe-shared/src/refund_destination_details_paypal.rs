#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct RefundDestinationDetailsPaypal {
    /// For refunds declined by the network, a decline code provided by the network which indicates the reason the refund failed.
    pub network_decline_code: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RefundDestinationDetailsPaypal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RefundDestinationDetailsPaypal").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct RefundDestinationDetailsPaypalBuilder {
    network_decline_code: Option<Option<String>>,
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

    impl Deserialize for RefundDestinationDetailsPaypal {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<RefundDestinationDetailsPaypal>,
        builder: RefundDestinationDetailsPaypalBuilder,
    }

    impl Visitor for Place<RefundDestinationDetailsPaypal> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: RefundDestinationDetailsPaypalBuilder {
                    network_decline_code: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "network_decline_code" => {
                    Deserialize::begin(&mut self.builder.network_decline_code)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(network_decline_code),) = (self.builder.network_decline_code.take(),) else {
                return Ok(());
            };
            *self.out = Some(RefundDestinationDetailsPaypal { network_decline_code });
            Ok(())
        }
    }
};

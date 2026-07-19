#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct RefundDestinationDetailsBlik {
    /// For refunds declined by the network, a decline code provided by the network which indicates the reason the refund failed.
    pub network_decline_code: Option<String>,
    /// The reference assigned to the refund.
    pub reference: Option<String>,
    /// Status of the reference on the refund. This can be `pending`, `available` or `unavailable`.
    pub reference_status: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RefundDestinationDetailsBlik {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RefundDestinationDetailsBlik").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct RefundDestinationDetailsBlikBuilder {
    network_decline_code: Option<Option<String>>,
    reference: Option<Option<String>>,
    reference_status: Option<Option<String>>,
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

    impl Deserialize for RefundDestinationDetailsBlik {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<RefundDestinationDetailsBlik>,
        builder: RefundDestinationDetailsBlikBuilder,
    }

    impl Visitor for Place<RefundDestinationDetailsBlik> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: RefundDestinationDetailsBlikBuilder {
                    network_decline_code: Deserialize::default(),
                    reference: Deserialize::default(),
                    reference_status: Deserialize::default(),
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
                "reference" => Deserialize::begin(&mut self.builder.reference),
                "reference_status" => Deserialize::begin(&mut self.builder.reference_status),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(network_decline_code), Some(reference), Some(reference_status)) = (
                self.builder.network_decline_code.take(),
                self.builder.reference.take(),
                self.builder.reference_status.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(RefundDestinationDetailsBlik {
                network_decline_code,
                reference,
                reference_status,
            });
            Ok(())
        }
    }
};

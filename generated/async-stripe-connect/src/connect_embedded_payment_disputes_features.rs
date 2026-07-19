#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ConnectEmbeddedPaymentDisputesFeatures {
    /// Whether connected accounts can manage destination charges that are created on behalf of them.
    /// This is `false` by default.
    pub destination_on_behalf_of_charge_management: bool,
    /// Whether responding to disputes is enabled, including submitting evidence and accepting disputes.
    /// This is `true` by default.
    pub dispute_management: bool,
    /// Whether sending refunds is enabled. This is `true` by default.
    pub refund_management: bool,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConnectEmbeddedPaymentDisputesFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConnectEmbeddedPaymentDisputesFeatures").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ConnectEmbeddedPaymentDisputesFeaturesBuilder {
    destination_on_behalf_of_charge_management: Option<bool>,
    dispute_management: Option<bool>,
    refund_management: Option<bool>,
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

    impl Deserialize for ConnectEmbeddedPaymentDisputesFeatures {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ConnectEmbeddedPaymentDisputesFeatures>,
        builder: ConnectEmbeddedPaymentDisputesFeaturesBuilder,
    }

    impl Visitor for Place<ConnectEmbeddedPaymentDisputesFeatures> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ConnectEmbeddedPaymentDisputesFeaturesBuilder {
                    destination_on_behalf_of_charge_management: Deserialize::default(),
                    dispute_management: Deserialize::default(),
                    refund_management: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "destination_on_behalf_of_charge_management" => {
                    Deserialize::begin(&mut self.builder.destination_on_behalf_of_charge_management)
                }
                "dispute_management" => Deserialize::begin(&mut self.builder.dispute_management),
                "refund_management" => Deserialize::begin(&mut self.builder.refund_management),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(destination_on_behalf_of_charge_management),
                Some(dispute_management),
                Some(refund_management),
            ) = (
                self.builder.destination_on_behalf_of_charge_management,
                self.builder.dispute_management,
                self.builder.refund_management,
            )
            else {
                return Ok(());
            };
            *self.out = Some(ConnectEmbeddedPaymentDisputesFeatures {
                destination_on_behalf_of_charge_management,
                dispute_management,
                refund_management,
            });
            Ok(())
        }
    }
};

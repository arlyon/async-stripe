#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryOutboundPaymentsResourceUsDomesticWireTrackingDetails {
    /// CHIPS System Sequence Number (SSN) of the OutboundPayment for payments sent over the `us_domestic_wire` network.
    pub chips: Option<String>,
    /// IMAD of the OutboundPayment for payments sent over the `us_domestic_wire` network.
    pub imad: Option<String>,
    /// OMAD of the OutboundPayment for payments sent over the `us_domestic_wire` network.
    pub omad: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryOutboundPaymentsResourceUsDomesticWireTrackingDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TreasuryOutboundPaymentsResourceUsDomesticWireTrackingDetails")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TreasuryOutboundPaymentsResourceUsDomesticWireTrackingDetailsBuilder {
    chips: Option<Option<String>>,
    imad: Option<Option<String>>,
    omad: Option<Option<String>>,
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

    impl Deserialize for TreasuryOutboundPaymentsResourceUsDomesticWireTrackingDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryOutboundPaymentsResourceUsDomesticWireTrackingDetails>,
        builder: TreasuryOutboundPaymentsResourceUsDomesticWireTrackingDetailsBuilder,
    }

    impl Visitor for Place<TreasuryOutboundPaymentsResourceUsDomesticWireTrackingDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasuryOutboundPaymentsResourceUsDomesticWireTrackingDetailsBuilder {
                    chips: Deserialize::default(),
                    imad: Deserialize::default(),
                    omad: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "chips" => Deserialize::begin(&mut self.builder.chips),
                "imad" => Deserialize::begin(&mut self.builder.imad),
                "omad" => Deserialize::begin(&mut self.builder.omad),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(chips), Some(imad), Some(omad)) =
                (self.builder.chips.take(), self.builder.imad.take(), self.builder.omad.take())
            else {
                return Ok(());
            };
            *self.out = Some(TreasuryOutboundPaymentsResourceUsDomesticWireTrackingDetails {
                chips,
                imad,
                omad,
            });
            Ok(())
        }
    }
};

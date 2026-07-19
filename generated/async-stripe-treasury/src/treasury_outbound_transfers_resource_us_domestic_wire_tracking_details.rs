#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryOutboundTransfersResourceUsDomesticWireTrackingDetails {
    /// CHIPS System Sequence Number (SSN) of the OutboundTransfer for transfers sent over the `us_domestic_wire` network.
    pub chips: Option<String>,
    /// IMAD of the OutboundTransfer for transfers sent over the `us_domestic_wire` network.
    pub imad: Option<String>,
    /// OMAD of the OutboundTransfer for transfers sent over the `us_domestic_wire` network.
    pub omad: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryOutboundTransfersResourceUsDomesticWireTrackingDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TreasuryOutboundTransfersResourceUsDomesticWireTrackingDetails")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TreasuryOutboundTransfersResourceUsDomesticWireTrackingDetailsBuilder {
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

    impl Deserialize for TreasuryOutboundTransfersResourceUsDomesticWireTrackingDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryOutboundTransfersResourceUsDomesticWireTrackingDetails>,
        builder: TreasuryOutboundTransfersResourceUsDomesticWireTrackingDetailsBuilder,
    }

    impl Visitor for Place<TreasuryOutboundTransfersResourceUsDomesticWireTrackingDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasuryOutboundTransfersResourceUsDomesticWireTrackingDetailsBuilder {
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
            *self.out = Some(TreasuryOutboundTransfersResourceUsDomesticWireTrackingDetails {
                chips,
                imad,
                omad,
            });
            Ok(())
        }
    }
};

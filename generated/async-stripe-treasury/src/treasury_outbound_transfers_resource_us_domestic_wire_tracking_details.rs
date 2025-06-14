#[derive(Clone, Debug)]
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
#[doc(hidden)]
pub struct TreasuryOutboundTransfersResourceUsDomesticWireTrackingDetailsBuilder {
    chips: Option<Option<String>>,
    imad: Option<Option<String>>,
    omad: Option<Option<String>>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
            builder: TreasuryOutboundTransfersResourceUsDomesticWireTrackingDetailsBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for TreasuryOutboundTransfersResourceUsDomesticWireTrackingDetailsBuilder {
        type Out = TreasuryOutboundTransfersResourceUsDomesticWireTrackingDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "chips" => Deserialize::begin(&mut self.chips),
                "imad" => Deserialize::begin(&mut self.imad),
                "omad" => Deserialize::begin(&mut self.omad),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                chips: Deserialize::default(),
                imad: Deserialize::default(),
                omad: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(chips), Some(imad), Some(omad)) =
                (self.chips.take(), self.imad.take(), self.omad.take())
            else {
                return None;
            };
            Some(Self::Out { chips, imad, omad })
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

    impl ObjectDeser for TreasuryOutboundTransfersResourceUsDomesticWireTrackingDetails {
        type Builder = TreasuryOutboundTransfersResourceUsDomesticWireTrackingDetailsBuilder;
    }

    impl FromValueOpt for TreasuryOutboundTransfersResourceUsDomesticWireTrackingDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TreasuryOutboundTransfersResourceUsDomesticWireTrackingDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "chips" => b.chips = FromValueOpt::from_value(v),
                    "imad" => b.imad = FromValueOpt::from_value(v),
                    "omad" => b.omad = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};

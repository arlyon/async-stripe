#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingTransactionFlightData {
    /// The time that the flight departed.
    pub departure_at: Option<i64>,
    /// The name of the passenger.
    pub passenger_name: Option<String>,
    /// Whether the ticket is refundable.
    pub refundable: Option<bool>,
    /// The legs of the trip.
    pub segments: Option<Vec<stripe_shared::IssuingTransactionFlightDataLeg>>,
    /// The travel agency that issued the ticket.
    pub travel_agency: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingTransactionFlightData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingTransactionFlightData").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingTransactionFlightDataBuilder {
    departure_at: Option<Option<i64>>,
    passenger_name: Option<Option<String>>,
    refundable: Option<Option<bool>>,
    segments: Option<Option<Vec<stripe_shared::IssuingTransactionFlightDataLeg>>>,
    travel_agency: Option<Option<String>>,
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

    impl Deserialize for IssuingTransactionFlightData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingTransactionFlightData>,
        builder: IssuingTransactionFlightDataBuilder,
    }

    impl Visitor for Place<IssuingTransactionFlightData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingTransactionFlightDataBuilder {
                    departure_at: Deserialize::default(),
                    passenger_name: Deserialize::default(),
                    refundable: Deserialize::default(),
                    segments: Deserialize::default(),
                    travel_agency: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "departure_at" => Deserialize::begin(&mut self.builder.departure_at),
                "passenger_name" => Deserialize::begin(&mut self.builder.passenger_name),
                "refundable" => Deserialize::begin(&mut self.builder.refundable),
                "segments" => Deserialize::begin(&mut self.builder.segments),
                "travel_agency" => Deserialize::begin(&mut self.builder.travel_agency),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(departure_at),
                Some(passenger_name),
                Some(refundable),
                Some(segments),
                Some(travel_agency),
            ) = (
                self.builder.departure_at,
                self.builder.passenger_name.take(),
                self.builder.refundable,
                self.builder.segments.take(),
                self.builder.travel_agency.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(IssuingTransactionFlightData {
                departure_at,
                passenger_name,
                refundable,
                segments,
                travel_agency,
            });
            Ok(())
        }
    }
};

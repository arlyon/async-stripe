#[derive(Clone, Debug, Default)]
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
#[doc(hidden)]
pub struct IssuingTransactionFlightDataBuilder {
    departure_at: Option<Option<i64>>,
    passenger_name: Option<Option<String>>,
    refundable: Option<Option<bool>>,
    segments: Option<Option<Vec<stripe_shared::IssuingTransactionFlightDataLeg>>>,
    travel_agency: Option<Option<String>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: IssuingTransactionFlightDataBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingTransactionFlightDataBuilder {
        type Out = IssuingTransactionFlightData;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "departure_at" => Deserialize::begin(&mut self.departure_at),
                "passenger_name" => Deserialize::begin(&mut self.passenger_name),
                "refundable" => Deserialize::begin(&mut self.refundable),
                "segments" => Deserialize::begin(&mut self.segments),
                "travel_agency" => Deserialize::begin(&mut self.travel_agency),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                departure_at: Deserialize::default(),
                passenger_name: Deserialize::default(),
                refundable: Deserialize::default(),
                segments: Deserialize::default(),
                travel_agency: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                departure_at: self.departure_at?,
                passenger_name: self.passenger_name.take()?,
                refundable: self.refundable?,
                segments: self.segments.take()?,
                travel_agency: self.travel_agency.take()?,
            })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for IssuingTransactionFlightData {
        type Builder = IssuingTransactionFlightDataBuilder;
    }

    impl FromValueOpt for IssuingTransactionFlightData {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingTransactionFlightDataBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "departure_at" => b.departure_at = Some(FromValueOpt::from_value(v)?),
                    "passenger_name" => b.passenger_name = Some(FromValueOpt::from_value(v)?),
                    "refundable" => b.refundable = Some(FromValueOpt::from_value(v)?),
                    "segments" => b.segments = Some(FromValueOpt::from_value(v)?),
                    "travel_agency" => b.travel_agency = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};

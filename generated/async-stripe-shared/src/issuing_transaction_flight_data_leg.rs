#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingTransactionFlightDataLeg {
    /// The three-letter IATA airport code of the flight's destination.
    pub arrival_airport_code: Option<String>,
    /// The airline carrier code.
    pub carrier: Option<String>,
    /// The three-letter IATA airport code that the flight departed from.
    pub departure_airport_code: Option<String>,
    /// The flight number.
    pub flight_number: Option<String>,
    /// The flight's service class.
    pub service_class: Option<String>,
    /// Whether a stopover is allowed on this flight.
    pub stopover_allowed: Option<bool>,
}
#[doc(hidden)]
pub struct IssuingTransactionFlightDataLegBuilder {
    arrival_airport_code: Option<Option<String>>,
    carrier: Option<Option<String>>,
    departure_airport_code: Option<Option<String>>,
    flight_number: Option<Option<String>>,
    service_class: Option<Option<String>>,
    stopover_allowed: Option<Option<bool>>,
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

    impl Deserialize for IssuingTransactionFlightDataLeg {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingTransactionFlightDataLeg>,
        builder: IssuingTransactionFlightDataLegBuilder,
    }

    impl Visitor for Place<IssuingTransactionFlightDataLeg> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingTransactionFlightDataLegBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingTransactionFlightDataLegBuilder {
        type Out = IssuingTransactionFlightDataLeg;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "arrival_airport_code" => Deserialize::begin(&mut self.arrival_airport_code),
                "carrier" => Deserialize::begin(&mut self.carrier),
                "departure_airport_code" => Deserialize::begin(&mut self.departure_airport_code),
                "flight_number" => Deserialize::begin(&mut self.flight_number),
                "service_class" => Deserialize::begin(&mut self.service_class),
                "stopover_allowed" => Deserialize::begin(&mut self.stopover_allowed),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                arrival_airport_code: Deserialize::default(),
                carrier: Deserialize::default(),
                departure_airport_code: Deserialize::default(),
                flight_number: Deserialize::default(),
                service_class: Deserialize::default(),
                stopover_allowed: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(arrival_airport_code),
                Some(carrier),
                Some(departure_airport_code),
                Some(flight_number),
                Some(service_class),
                Some(stopover_allowed),
            ) = (
                self.arrival_airport_code.take(),
                self.carrier.take(),
                self.departure_airport_code.take(),
                self.flight_number.take(),
                self.service_class.take(),
                self.stopover_allowed,
            )
            else {
                return None;
            };
            Some(Self::Out {
                arrival_airport_code,
                carrier,
                departure_airport_code,
                flight_number,
                service_class,
                stopover_allowed,
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

    impl ObjectDeser for IssuingTransactionFlightDataLeg {
        type Builder = IssuingTransactionFlightDataLegBuilder;
    }

    impl FromValueOpt for IssuingTransactionFlightDataLeg {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingTransactionFlightDataLegBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "arrival_airport_code" => b.arrival_airport_code = FromValueOpt::from_value(v),
                    "carrier" => b.carrier = FromValueOpt::from_value(v),
                    "departure_airport_code" => {
                        b.departure_airport_code = FromValueOpt::from_value(v)
                    }
                    "flight_number" => b.flight_number = FromValueOpt::from_value(v),
                    "service_class" => b.service_class = FromValueOpt::from_value(v),
                    "stopover_allowed" => b.stopover_allowed = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};

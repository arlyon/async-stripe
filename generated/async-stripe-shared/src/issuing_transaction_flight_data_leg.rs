#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingTransactionFlightDataLeg {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingTransactionFlightDataLeg").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: IssuingTransactionFlightDataLegBuilder {
                    arrival_airport_code: Deserialize::default(),
                    carrier: Deserialize::default(),
                    departure_airport_code: Deserialize::default(),
                    flight_number: Deserialize::default(),
                    service_class: Deserialize::default(),
                    stopover_allowed: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "arrival_airport_code" => {
                    Deserialize::begin(&mut self.builder.arrival_airport_code)
                }
                "carrier" => Deserialize::begin(&mut self.builder.carrier),
                "departure_airport_code" => {
                    Deserialize::begin(&mut self.builder.departure_airport_code)
                }
                "flight_number" => Deserialize::begin(&mut self.builder.flight_number),
                "service_class" => Deserialize::begin(&mut self.builder.service_class),
                "stopover_allowed" => Deserialize::begin(&mut self.builder.stopover_allowed),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(arrival_airport_code),
                Some(carrier),
                Some(departure_airport_code),
                Some(flight_number),
                Some(service_class),
                Some(stopover_allowed),
            ) = (
                self.builder.arrival_airport_code.take(),
                self.builder.carrier.take(),
                self.builder.departure_airport_code.take(),
                self.builder.flight_number.take(),
                self.builder.service_class.take(),
                self.builder.stopover_allowed,
            )
            else {
                return Ok(());
            };
            *self.out = Some(IssuingTransactionFlightDataLeg {
                arrival_airport_code,
                carrier,
                departure_airport_code,
                flight_number,
                service_class,
                stopover_allowed,
            });
            Ok(())
        }
    }
};

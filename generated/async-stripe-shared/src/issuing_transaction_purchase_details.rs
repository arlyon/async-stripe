#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingTransactionPurchaseDetails {
    /// Fleet-specific information for transactions using Fleet cards.
    pub fleet: Option<stripe_shared::IssuingTransactionFleetData>,
    /// Information about the flight that was purchased with this transaction.
    pub flight: Option<stripe_shared::IssuingTransactionFlightData>,
    /// Information about fuel that was purchased with this transaction.
    pub fuel: Option<stripe_shared::IssuingTransactionFuelData>,
    /// Information about lodging that was purchased with this transaction.
    pub lodging: Option<stripe_shared::IssuingTransactionLodgingData>,
    /// The line items in the purchase.
    pub receipt: Option<Vec<stripe_shared::IssuingTransactionReceiptData>>,
    /// A merchant-specific order number.
    pub reference: Option<String>,
}
#[doc(hidden)]
pub struct IssuingTransactionPurchaseDetailsBuilder {
    fleet: Option<Option<stripe_shared::IssuingTransactionFleetData>>,
    flight: Option<Option<stripe_shared::IssuingTransactionFlightData>>,
    fuel: Option<Option<stripe_shared::IssuingTransactionFuelData>>,
    lodging: Option<Option<stripe_shared::IssuingTransactionLodgingData>>,
    receipt: Option<Option<Vec<stripe_shared::IssuingTransactionReceiptData>>>,
    reference: Option<Option<String>>,
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

    impl Deserialize for IssuingTransactionPurchaseDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingTransactionPurchaseDetails>,
        builder: IssuingTransactionPurchaseDetailsBuilder,
    }

    impl Visitor for Place<IssuingTransactionPurchaseDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingTransactionPurchaseDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingTransactionPurchaseDetailsBuilder {
        type Out = IssuingTransactionPurchaseDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "fleet" => Deserialize::begin(&mut self.fleet),
                "flight" => Deserialize::begin(&mut self.flight),
                "fuel" => Deserialize::begin(&mut self.fuel),
                "lodging" => Deserialize::begin(&mut self.lodging),
                "receipt" => Deserialize::begin(&mut self.receipt),
                "reference" => Deserialize::begin(&mut self.reference),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                fleet: Deserialize::default(),
                flight: Deserialize::default(),
                fuel: Deserialize::default(),
                lodging: Deserialize::default(),
                receipt: Deserialize::default(),
                reference: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(fleet),
                Some(flight),
                Some(fuel),
                Some(lodging),
                Some(receipt),
                Some(reference),
            ) = (
                self.fleet.take(),
                self.flight.take(),
                self.fuel.take(),
                self.lodging,
                self.receipt.take(),
                self.reference.take(),
            )
            else {
                return None;
            };
            Some(Self::Out { fleet, flight, fuel, lodging, receipt, reference })
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

    impl ObjectDeser for IssuingTransactionPurchaseDetails {
        type Builder = IssuingTransactionPurchaseDetailsBuilder;
    }

    impl FromValueOpt for IssuingTransactionPurchaseDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingTransactionPurchaseDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "fleet" => b.fleet = FromValueOpt::from_value(v),
                    "flight" => b.flight = FromValueOpt::from_value(v),
                    "fuel" => b.fuel = FromValueOpt::from_value(v),
                    "lodging" => b.lodging = FromValueOpt::from_value(v),
                    "receipt" => b.receipt = FromValueOpt::from_value(v),
                    "reference" => b.reference = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};

#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingTransactionPurchaseDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingTransactionPurchaseDetails").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: IssuingTransactionPurchaseDetailsBuilder {
                    fleet: Deserialize::default(),
                    flight: Deserialize::default(),
                    fuel: Deserialize::default(),
                    lodging: Deserialize::default(),
                    receipt: Deserialize::default(),
                    reference: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "fleet" => Deserialize::begin(&mut self.builder.fleet),
                "flight" => Deserialize::begin(&mut self.builder.flight),
                "fuel" => Deserialize::begin(&mut self.builder.fuel),
                "lodging" => Deserialize::begin(&mut self.builder.lodging),
                "receipt" => Deserialize::begin(&mut self.builder.receipt),
                "reference" => Deserialize::begin(&mut self.builder.reference),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(fleet),
                Some(flight),
                Some(fuel),
                Some(lodging),
                Some(receipt),
                Some(reference),
            ) = (
                self.builder.fleet.take(),
                self.builder.flight.take(),
                self.builder.fuel.take(),
                self.builder.lodging,
                self.builder.receipt.take(),
                self.builder.reference.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(IssuingTransactionPurchaseDetails {
                fleet,
                flight,
                fuel,
                lodging,
                receipt,
                reference,
            });
            Ok(())
        }
    }
};

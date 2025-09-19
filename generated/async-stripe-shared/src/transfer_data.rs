#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TransferData {
    /// The amount transferred to the destination account.
    /// This transfer will occur automatically after the payment succeeds.
    /// If no amount is specified, by default the entire payment amount is transferred to the destination account.
    /// The amount must be less than or equal to the [amount](https://stripe.com/docs/api/payment_intents/object#payment_intent_object-amount), and must be a positive integer.
    ///  representing how much to transfer in the smallest currency unit (e.g., 100 cents to charge $1.00).
    pub amount: Option<i64>,
    /// The account (if any) that the payment is attributed to for tax reporting, and where funds from the payment are transferred to after payment success.
    pub destination: stripe_types::Expandable<stripe_shared::Account>,
}
#[doc(hidden)]
pub struct TransferDataBuilder {
    amount: Option<Option<i64>>,
    destination: Option<stripe_types::Expandable<stripe_shared::Account>>,
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
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TransferData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TransferData>,
        builder: TransferDataBuilder,
    }

    impl Visitor for Place<TransferData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TransferDataBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TransferDataBuilder {
        type Out = TransferData;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "destination" => Deserialize::begin(&mut self.destination),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { amount: Deserialize::default(), destination: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(amount), Some(destination)) = (self.amount, self.destination.take()) else {
                return None;
            };
            Some(Self::Out { amount, destination })
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

    impl ObjectDeser for TransferData {
        type Builder = TransferDataBuilder;
    }

    impl FromValueOpt for TransferData {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TransferDataBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "destination" => b.destination = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};

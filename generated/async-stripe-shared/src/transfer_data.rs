#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TransferData {
    /// Amount intended to be collected by this PaymentIntent.
    /// A positive integer representing how much to charge in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) (e.g., 100 cents to charge $1.00 or 100 to charge ¥100, a zero-decimal currency).
    /// The minimum amount is $0.50 US or [equivalent in charge currency](https://stripe.com/docs/currencies#minimum-and-maximum-charge-amounts).
    /// The amount value supports up to eight digits (e.g., a value of 99999999 for a USD charge of $999,999.99).
    pub amount: Option<i64>,
    /// The account (if any) that the payment is attributed to for tax
    /// reporting, and where funds from the payment are transferred to after
    /// payment success.
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

    impl<'a> Map for Builder<'a> {
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

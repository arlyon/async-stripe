#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitions {
    /// Timestamp describing when the Transaction changed status to `posted`.
    pub posted_at: Option<stripe_types::Timestamp>,
    /// Timestamp describing when the Transaction changed status to `void`.
    pub void_at: Option<stripe_types::Timestamp>,
}
#[doc(hidden)]
pub struct TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitionsBuilder {
    posted_at: Option<Option<stripe_types::Timestamp>>,
    void_at: Option<Option<stripe_types::Timestamp>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<
            TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitions,
        >,
        builder: TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitionsBuilder,
    }

    impl Visitor for Place<TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitionsBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder
        for TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitionsBuilder
    {
        type Out = TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "posted_at" => Deserialize::begin(&mut self.posted_at),
                "void_at" => Deserialize::begin(&mut self.void_at),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { posted_at: Deserialize::default(), void_at: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { posted_at: self.posted_at?, void_at: self.void_at? })
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

    impl ObjectDeser for TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitions {
        type Builder =
            TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitionsBuilder;
    }

    impl FromValueOpt for TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "posted_at" => b.posted_at = Some(FromValueOpt::from_value(v)?),
                    "void_at" => b.void_at = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};

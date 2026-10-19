#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodP24 {
    /// The customer's bank, if provided.
    pub bank: Option<stripe_shared::PaymentMethodP24Bank>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodP24 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodP24").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodP24Builder {
    bank: Option<Option<stripe_shared::PaymentMethodP24Bank>>,
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

    impl Deserialize for PaymentMethodP24 {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodP24>,
        builder: PaymentMethodP24Builder,
    }

    impl Visitor for Place<PaymentMethodP24> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodP24Builder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodP24Builder {
        type Out = PaymentMethodP24;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bank" => Deserialize::begin(&mut self.bank),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { bank: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(bank),) = (self.bank.take(),) else {
                return None;
            };
            Some(Self::Out { bank })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for PaymentMethodP24 {
        type Builder = PaymentMethodP24Builder;
    }

    impl FromValueOpt for PaymentMethodP24 {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodP24Builder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "bank" => b.bank = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The customer's bank, if provided.
pub type PaymentMethodP24Bank = stripe_shared::PaymentMethodP24Bank;

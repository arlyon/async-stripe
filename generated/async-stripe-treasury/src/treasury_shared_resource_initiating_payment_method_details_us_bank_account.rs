#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccount {
    /// Bank name.
    pub bank_name: Option<String>,
    /// The last four digits of the bank account number.
    pub last4: Option<String>,
    /// The routing number for the bank account.
    pub routing_number: Option<String>,
}
#[doc(hidden)]
pub struct TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccountBuilder {
    bank_name: Option<Option<String>>,
    last4: Option<Option<String>>,
    routing_number: Option<Option<String>>,
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

    impl Deserialize for TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccount>,
        builder: TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccountBuilder,
    }

    impl Visitor for Place<TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccountBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccountBuilder {
        type Out = TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccount;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bank_name" => Deserialize::begin(&mut self.bank_name),
                "last4" => Deserialize::begin(&mut self.last4),
                "routing_number" => Deserialize::begin(&mut self.routing_number),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                bank_name: Deserialize::default(),
                last4: Deserialize::default(),
                routing_number: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(bank_name), Some(last4), Some(routing_number)) =
                (self.bank_name.take(), self.last4.take(), self.routing_number.take())
            else {
                return None;
            };
            Some(Self::Out { bank_name, last4, routing_number })
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

    impl ObjectDeser for TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccount {
        type Builder = TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccountBuilder;
    }

    impl FromValueOpt for TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccount {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccountBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "bank_name" => b.bank_name = FromValueOpt::from_value(v),
                    "last4" => b.last4 = FromValueOpt::from_value(v),
                    "routing_number" => b.routing_number = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};

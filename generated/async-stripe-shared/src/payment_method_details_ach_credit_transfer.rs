#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsAchCreditTransfer {
    /// Account number to transfer funds to.
    pub account_number: Option<String>,
    /// Name of the bank associated with the routing number.
    pub bank_name: Option<String>,
    /// Routing transit number for the bank account to transfer funds to.
    pub routing_number: Option<String>,
    /// SWIFT code of the bank associated with the routing number.
    pub swift_code: Option<String>,
}
#[doc(hidden)]
pub struct PaymentMethodDetailsAchCreditTransferBuilder {
    account_number: Option<Option<String>>,
    bank_name: Option<Option<String>>,
    routing_number: Option<Option<String>>,
    swift_code: Option<Option<String>>,
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

    impl Deserialize for PaymentMethodDetailsAchCreditTransfer {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsAchCreditTransfer>,
        builder: PaymentMethodDetailsAchCreditTransferBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsAchCreditTransfer> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsAchCreditTransferBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsAchCreditTransferBuilder {
        type Out = PaymentMethodDetailsAchCreditTransfer;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account_number" => Deserialize::begin(&mut self.account_number),
                "bank_name" => Deserialize::begin(&mut self.bank_name),
                "routing_number" => Deserialize::begin(&mut self.routing_number),
                "swift_code" => Deserialize::begin(&mut self.swift_code),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                account_number: Deserialize::default(),
                bank_name: Deserialize::default(),
                routing_number: Deserialize::default(),
                swift_code: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(account_number), Some(bank_name), Some(routing_number), Some(swift_code)) = (
                self.account_number.take(),
                self.bank_name.take(),
                self.routing_number.take(),
                self.swift_code.take(),
            ) else {
                return None;
            };
            Some(Self::Out { account_number, bank_name, routing_number, swift_code })
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

    impl ObjectDeser for PaymentMethodDetailsAchCreditTransfer {
        type Builder = PaymentMethodDetailsAchCreditTransferBuilder;
    }

    impl FromValueOpt for PaymentMethodDetailsAchCreditTransfer {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodDetailsAchCreditTransferBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "account_number" => b.account_number = FromValueOpt::from_value(v),
                    "bank_name" => b.bank_name = FromValueOpt::from_value(v),
                    "routing_number" => b.routing_number = FromValueOpt::from_value(v),
                    "swift_code" => b.swift_code = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};

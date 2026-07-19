#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodDetailsAchCreditTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodDetailsAchCreditTransfer").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: PaymentMethodDetailsAchCreditTransferBuilder {
                    account_number: Deserialize::default(),
                    bank_name: Deserialize::default(),
                    routing_number: Deserialize::default(),
                    swift_code: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account_number" => Deserialize::begin(&mut self.builder.account_number),
                "bank_name" => Deserialize::begin(&mut self.builder.bank_name),
                "routing_number" => Deserialize::begin(&mut self.builder.routing_number),
                "swift_code" => Deserialize::begin(&mut self.builder.swift_code),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(account_number), Some(bank_name), Some(routing_number), Some(swift_code)) = (
                self.builder.account_number.take(),
                self.builder.bank_name.take(),
                self.builder.routing_number.take(),
                self.builder.swift_code.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(PaymentMethodDetailsAchCreditTransfer {
                account_number,
                bank_name,
                routing_number,
                swift_code,
            });
            Ok(())
        }
    }
};

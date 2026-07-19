#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceTypeAchCreditTransfer {
    pub account_number: Option<String>,
    pub bank_name: Option<String>,
    pub fingerprint: Option<String>,
    pub refund_account_holder_name: Option<String>,
    pub refund_account_holder_type: Option<String>,
    pub refund_routing_number: Option<String>,
    pub routing_number: Option<String>,
    pub swift_code: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SourceTypeAchCreditTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SourceTypeAchCreditTransfer").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SourceTypeAchCreditTransferBuilder {
    account_number: Option<Option<String>>,
    bank_name: Option<Option<String>>,
    fingerprint: Option<Option<String>>,
    refund_account_holder_name: Option<Option<String>>,
    refund_account_holder_type: Option<Option<String>>,
    refund_routing_number: Option<Option<String>>,
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

    impl Deserialize for SourceTypeAchCreditTransfer {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTypeAchCreditTransfer>,
        builder: SourceTypeAchCreditTransferBuilder,
    }

    impl Visitor for Place<SourceTypeAchCreditTransfer> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SourceTypeAchCreditTransferBuilder {
                    account_number: Deserialize::default(),
                    bank_name: Deserialize::default(),
                    fingerprint: Deserialize::default(),
                    refund_account_holder_name: Deserialize::default(),
                    refund_account_holder_type: Deserialize::default(),
                    refund_routing_number: Deserialize::default(),
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
                "fingerprint" => Deserialize::begin(&mut self.builder.fingerprint),
                "refund_account_holder_name" => {
                    Deserialize::begin(&mut self.builder.refund_account_holder_name)
                }
                "refund_account_holder_type" => {
                    Deserialize::begin(&mut self.builder.refund_account_holder_type)
                }
                "refund_routing_number" => {
                    Deserialize::begin(&mut self.builder.refund_routing_number)
                }
                "routing_number" => Deserialize::begin(&mut self.builder.routing_number),
                "swift_code" => Deserialize::begin(&mut self.builder.swift_code),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(account_number),
                Some(bank_name),
                Some(fingerprint),
                Some(refund_account_holder_name),
                Some(refund_account_holder_type),
                Some(refund_routing_number),
                Some(routing_number),
                Some(swift_code),
            ) = (
                self.builder.account_number.take(),
                self.builder.bank_name.take(),
                self.builder.fingerprint.take(),
                self.builder.refund_account_holder_name.take(),
                self.builder.refund_account_holder_type.take(),
                self.builder.refund_routing_number.take(),
                self.builder.routing_number.take(),
                self.builder.swift_code.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(SourceTypeAchCreditTransfer {
                account_number,
                bank_name,
                fingerprint,
                refund_account_holder_name,
                refund_account_holder_type,
                refund_routing_number,
                routing_number,
                swift_code,
            });
            Ok(())
        }
    }
};

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
                builder: SourceTypeAchCreditTransferBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SourceTypeAchCreditTransferBuilder {
        type Out = SourceTypeAchCreditTransfer;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account_number" => Deserialize::begin(&mut self.account_number),
                "bank_name" => Deserialize::begin(&mut self.bank_name),
                "fingerprint" => Deserialize::begin(&mut self.fingerprint),
                "refund_account_holder_name" => {
                    Deserialize::begin(&mut self.refund_account_holder_name)
                }
                "refund_account_holder_type" => {
                    Deserialize::begin(&mut self.refund_account_holder_type)
                }
                "refund_routing_number" => Deserialize::begin(&mut self.refund_routing_number),
                "routing_number" => Deserialize::begin(&mut self.routing_number),
                "swift_code" => Deserialize::begin(&mut self.swift_code),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                account_number: Some(None),
                bank_name: Some(None),
                fingerprint: Some(None),
                refund_account_holder_name: Some(None),
                refund_account_holder_type: Some(None),
                refund_routing_number: Some(None),
                routing_number: Some(None),
                swift_code: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    account_number: Some(account_number),
                    bank_name: Some(bank_name),
                    fingerprint: Some(fingerprint),
                    refund_account_holder_name: Some(refund_account_holder_name),
                    refund_account_holder_type: Some(refund_account_holder_type),
                    refund_routing_number: Some(refund_routing_number),
                    routing_number: Some(routing_number),
                    swift_code: Some(swift_code),
                } => Some(Self::Out {
                    account_number,
                    bank_name,
                    fingerprint,
                    refund_account_holder_name,
                    refund_account_holder_type,
                    refund_routing_number,
                    routing_number,
                    swift_code,
                }),
                builder => {
                    *self = builder;
                    None
                }
            }
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

    impl ObjectDeser for SourceTypeAchCreditTransfer {
        type Builder = SourceTypeAchCreditTransferBuilder;
    }

    impl FromValueOpt for SourceTypeAchCreditTransfer {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};

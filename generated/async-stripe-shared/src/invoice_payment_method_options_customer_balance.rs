#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoicePaymentMethodOptionsCustomerBalance {
    pub bank_transfer:
        Option<stripe_shared::InvoicePaymentMethodOptionsCustomerBalanceBankTransfer>,
    /// The funding method type to be used when there are not enough funds in the customer balance.
    /// Permitted values include: `bank_transfer`.
    pub funding_type: Option<stripe_shared::FundingType>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoicePaymentMethodOptionsCustomerBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InvoicePaymentMethodOptionsCustomerBalance").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InvoicePaymentMethodOptionsCustomerBalanceBuilder {
    bank_transfer:
        Option<Option<stripe_shared::InvoicePaymentMethodOptionsCustomerBalanceBankTransfer>>,
    funding_type: Option<Option<stripe_shared::FundingType>>,
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

    impl Deserialize for InvoicePaymentMethodOptionsCustomerBalance {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoicePaymentMethodOptionsCustomerBalance>,
        builder: InvoicePaymentMethodOptionsCustomerBalanceBuilder,
    }

    impl Visitor for Place<InvoicePaymentMethodOptionsCustomerBalance> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InvoicePaymentMethodOptionsCustomerBalanceBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InvoicePaymentMethodOptionsCustomerBalanceBuilder {
        type Out = InvoicePaymentMethodOptionsCustomerBalance;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bank_transfer" => Deserialize::begin(&mut self.bank_transfer),
                "funding_type" => Deserialize::begin(&mut self.funding_type),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { bank_transfer: Some(None), funding_type: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(bank_transfer), Some(funding_type)) =
                (self.bank_transfer.take(), self.funding_type.take())
            else {
                return None;
            };
            Some(Self::Out { bank_transfer, funding_type })
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

    impl ObjectDeser for InvoicePaymentMethodOptionsCustomerBalance {
        type Builder = InvoicePaymentMethodOptionsCustomerBalanceBuilder;
    }

    impl FromValueOpt for InvoicePaymentMethodOptionsCustomerBalance {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InvoicePaymentMethodOptionsCustomerBalanceBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "bank_transfer" => b.bank_transfer = FromValueOpt::from_value(v),
                    "funding_type" => b.funding_type = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The funding method type to be used when there are not enough funds in the customer balance.
/// Permitted values include: `bank_transfer`.
pub type InvoicePaymentMethodOptionsCustomerBalanceFundingType = stripe_shared::FundingType;

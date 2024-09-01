#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoicePaymentMethodOptionsCustomerBalanceBankTransfer {
    pub eu_bank_transfer:
        Option<stripe_shared::InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer>,
    /// The bank transfer type that can be used for funding.
    /// Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, `mx_bank_transfer`, or `us_bank_transfer`.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: Option<String>,
}
#[doc(hidden)]
pub struct InvoicePaymentMethodOptionsCustomerBalanceBankTransferBuilder {
    eu_bank_transfer: Option<
        Option<stripe_shared::InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer>,
    >,
    type_: Option<Option<String>>,
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

    impl Deserialize for InvoicePaymentMethodOptionsCustomerBalanceBankTransfer {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoicePaymentMethodOptionsCustomerBalanceBankTransfer>,
        builder: InvoicePaymentMethodOptionsCustomerBalanceBankTransferBuilder,
    }

    impl Visitor for Place<InvoicePaymentMethodOptionsCustomerBalanceBankTransfer> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    InvoicePaymentMethodOptionsCustomerBalanceBankTransferBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InvoicePaymentMethodOptionsCustomerBalanceBankTransferBuilder {
        type Out = InvoicePaymentMethodOptionsCustomerBalanceBankTransfer;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "eu_bank_transfer" => Deserialize::begin(&mut self.eu_bank_transfer),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { eu_bank_transfer: Deserialize::default(), type_: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(eu_bank_transfer), Some(type_)) = (self.eu_bank_transfer, self.type_.take())
            else {
                return None;
            };
            Some(Self::Out { eu_bank_transfer, type_ })
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

    impl ObjectDeser for InvoicePaymentMethodOptionsCustomerBalanceBankTransfer {
        type Builder = InvoicePaymentMethodOptionsCustomerBalanceBankTransferBuilder;
    }

    impl FromValueOpt for InvoicePaymentMethodOptionsCustomerBalanceBankTransfer {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                InvoicePaymentMethodOptionsCustomerBalanceBankTransferBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "eu_bank_transfer" => b.eu_bank_transfer = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoicePaymentMethodOptionsCustomerBalanceBankTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InvoicePaymentMethodOptionsCustomerBalanceBankTransfer")
            .finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: InvoicePaymentMethodOptionsCustomerBalanceBankTransferBuilder {
                    eu_bank_transfer: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "eu_bank_transfer" => Deserialize::begin(&mut self.builder.eu_bank_transfer),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(eu_bank_transfer), Some(type_)) =
                (self.builder.eu_bank_transfer.take(), self.builder.type_.take())
            else {
                return Ok(());
            };
            *self.out = Some(InvoicePaymentMethodOptionsCustomerBalanceBankTransfer {
                eu_bank_transfer,
                type_,
            });
            Ok(())
        }
    }
};

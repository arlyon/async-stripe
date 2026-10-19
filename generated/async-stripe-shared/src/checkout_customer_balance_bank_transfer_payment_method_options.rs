#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CheckoutCustomerBalanceBankTransferPaymentMethodOptions {
    pub eu_bank_transfer: Option<stripe_shared::PaymentMethodOptionsCustomerBalanceEuBankAccount>,
    /// List of address types that should be returned in the financial_addresses response.
    /// If not specified, all valid types will be returned.
    ///
    /// Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.
    pub requested_address_types: Option<Vec<stripe_shared::RequestedAddressTypes>>,
    /// The bank transfer type that this PaymentIntent is allowed to use for funding Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, `mx_bank_transfer`, or `us_bank_transfer`.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: Option<stripe_shared::CreateFundingInstructionsCustomerBankTransferType>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CheckoutCustomerBalanceBankTransferPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CheckoutCustomerBalanceBankTransferPaymentMethodOptions")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CheckoutCustomerBalanceBankTransferPaymentMethodOptionsBuilder {
    eu_bank_transfer:
        Option<Option<stripe_shared::PaymentMethodOptionsCustomerBalanceEuBankAccount>>,
    requested_address_types: Option<Option<Vec<stripe_shared::RequestedAddressTypes>>>,
    type_: Option<Option<stripe_shared::CreateFundingInstructionsCustomerBankTransferType>>,
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

    impl Deserialize for CheckoutCustomerBalanceBankTransferPaymentMethodOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CheckoutCustomerBalanceBankTransferPaymentMethodOptions>,
        builder: CheckoutCustomerBalanceBankTransferPaymentMethodOptionsBuilder,
    }

    impl Visitor for Place<CheckoutCustomerBalanceBankTransferPaymentMethodOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    CheckoutCustomerBalanceBankTransferPaymentMethodOptionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for CheckoutCustomerBalanceBankTransferPaymentMethodOptionsBuilder {
        type Out = CheckoutCustomerBalanceBankTransferPaymentMethodOptions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "eu_bank_transfer" => Deserialize::begin(&mut self.eu_bank_transfer),
                "requested_address_types" => Deserialize::begin(&mut self.requested_address_types),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                eu_bank_transfer: Some(None),
                requested_address_types: Some(None),
                type_: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(eu_bank_transfer), Some(requested_address_types), Some(type_)) = (
                self.eu_bank_transfer.take(),
                self.requested_address_types.take(),
                self.type_.take(),
            ) else {
                return None;
            };
            Some(Self::Out { eu_bank_transfer, requested_address_types, type_ })
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

    impl ObjectDeser for CheckoutCustomerBalanceBankTransferPaymentMethodOptions {
        type Builder = CheckoutCustomerBalanceBankTransferPaymentMethodOptionsBuilder;
    }

    impl FromValueOpt for CheckoutCustomerBalanceBankTransferPaymentMethodOptions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                CheckoutCustomerBalanceBankTransferPaymentMethodOptionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "eu_bank_transfer" => b.eu_bank_transfer = FromValueOpt::from_value(v),
                    "requested_address_types" => {
                        b.requested_address_types = FromValueOpt::from_value(v)
                    }
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// List of address types that should be returned in the financial_addresses response.
/// If not specified, all valid types will be returned.
///
/// Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.
pub type CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypes =
    stripe_shared::RequestedAddressTypes;
/// The bank transfer type that this PaymentIntent is allowed to use for funding Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, `mx_bank_transfer`, or `us_bank_transfer`.
pub type CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType =
    stripe_shared::CreateFundingInstructionsCustomerBankTransferType;

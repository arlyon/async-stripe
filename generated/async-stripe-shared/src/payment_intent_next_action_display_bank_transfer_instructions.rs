#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentIntentNextActionDisplayBankTransferInstructions {
    /// The remaining amount that needs to be transferred to complete the payment.
    pub amount_remaining: Option<i64>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Option<stripe_types::Currency>,
    /// A list of financial addresses that can be used to fund the customer balance
    pub financial_addresses:
        Option<Vec<stripe_shared::FundingInstructionsBankTransferFinancialAddress>>,
    /// A link to a hosted page that guides your customer through completing the transfer.
    pub hosted_instructions_url: Option<String>,
    /// A string identifying this payment.
    /// Instruct your customer to include this code in the reference or memo field of their bank transfer.
    pub reference: Option<String>,
    /// Type of bank transfer
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: stripe_shared::CreateFundingInstructionsCustomerBankTransferType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentIntentNextActionDisplayBankTransferInstructions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentIntentNextActionDisplayBankTransferInstructions")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentIntentNextActionDisplayBankTransferInstructionsBuilder {
    amount_remaining: Option<Option<i64>>,
    currency: Option<Option<stripe_types::Currency>>,
    financial_addresses:
        Option<Option<Vec<stripe_shared::FundingInstructionsBankTransferFinancialAddress>>>,
    hosted_instructions_url: Option<Option<String>>,
    reference: Option<Option<String>>,
    type_: Option<stripe_shared::CreateFundingInstructionsCustomerBankTransferType>,
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

    impl Deserialize for PaymentIntentNextActionDisplayBankTransferInstructions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextActionDisplayBankTransferInstructions>,
        builder: PaymentIntentNextActionDisplayBankTransferInstructionsBuilder,
    }

    impl Visitor for Place<PaymentIntentNextActionDisplayBankTransferInstructions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    PaymentIntentNextActionDisplayBankTransferInstructionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentIntentNextActionDisplayBankTransferInstructionsBuilder {
        type Out = PaymentIntentNextActionDisplayBankTransferInstructions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount_remaining" => Deserialize::begin(&mut self.amount_remaining),
                "currency" => Deserialize::begin(&mut self.currency),
                "financial_addresses" => Deserialize::begin(&mut self.financial_addresses),
                "hosted_instructions_url" => Deserialize::begin(&mut self.hosted_instructions_url),
                "reference" => Deserialize::begin(&mut self.reference),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount_remaining: Some(None),
                currency: Some(None),
                financial_addresses: Some(None),
                hosted_instructions_url: Some(None),
                reference: Some(None),
                type_: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(amount_remaining),
                Some(currency),
                Some(financial_addresses),
                Some(hosted_instructions_url),
                Some(reference),
                Some(type_),
            ) = (
                self.amount_remaining,
                self.currency.take(),
                self.financial_addresses.take(),
                self.hosted_instructions_url.take(),
                self.reference.take(),
                self.type_.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                amount_remaining,
                currency,
                financial_addresses,
                hosted_instructions_url,
                reference,
                type_,
            })
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

    impl ObjectDeser for PaymentIntentNextActionDisplayBankTransferInstructions {
        type Builder = PaymentIntentNextActionDisplayBankTransferInstructionsBuilder;
    }

    impl FromValueOpt for PaymentIntentNextActionDisplayBankTransferInstructions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                PaymentIntentNextActionDisplayBankTransferInstructionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount_remaining" => b.amount_remaining = FromValueOpt::from_value(v),
                    "currency" => b.currency = FromValueOpt::from_value(v),
                    "financial_addresses" => b.financial_addresses = FromValueOpt::from_value(v),
                    "hosted_instructions_url" => {
                        b.hosted_instructions_url = FromValueOpt::from_value(v)
                    }
                    "reference" => b.reference = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Type of bank transfer
pub type PaymentIntentNextActionDisplayBankTransferInstructionsType =
    stripe_shared::CreateFundingInstructionsCustomerBankTransferType;

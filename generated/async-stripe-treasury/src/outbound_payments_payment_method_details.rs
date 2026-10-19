#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct OutboundPaymentsPaymentMethodDetails {
    pub billing_details: stripe_treasury::TreasurySharedResourceBillingDetails,
    pub financial_account:
        Option<stripe_treasury::OutboundPaymentsPaymentMethodDetailsFinancialAccount>,
    /// The type of the payment method used in the OutboundPayment.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: stripe_shared::OutboundPaymentsPaymentMethodDetailsType,
    pub us_bank_account: Option<stripe_treasury::OutboundPaymentsPaymentMethodDetailsUsBankAccount>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for OutboundPaymentsPaymentMethodDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("OutboundPaymentsPaymentMethodDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct OutboundPaymentsPaymentMethodDetailsBuilder {
    billing_details: Option<stripe_treasury::TreasurySharedResourceBillingDetails>,
    financial_account:
        Option<Option<stripe_treasury::OutboundPaymentsPaymentMethodDetailsFinancialAccount>>,
    type_: Option<stripe_shared::OutboundPaymentsPaymentMethodDetailsType>,
    us_bank_account:
        Option<Option<stripe_treasury::OutboundPaymentsPaymentMethodDetailsUsBankAccount>>,
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

    impl Deserialize for OutboundPaymentsPaymentMethodDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<OutboundPaymentsPaymentMethodDetails>,
        builder: OutboundPaymentsPaymentMethodDetailsBuilder,
    }

    impl Visitor for Place<OutboundPaymentsPaymentMethodDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: OutboundPaymentsPaymentMethodDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for OutboundPaymentsPaymentMethodDetailsBuilder {
        type Out = OutboundPaymentsPaymentMethodDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "billing_details" => Deserialize::begin(&mut self.billing_details),
                "financial_account" => Deserialize::begin(&mut self.financial_account),
                "type" => Deserialize::begin(&mut self.type_),
                "us_bank_account" => Deserialize::begin(&mut self.us_bank_account),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                billing_details: None,
                financial_account: Some(None),
                type_: None,
                us_bank_account: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(billing_details),
                Some(financial_account),
                Some(type_),
                Some(us_bank_account),
            ) = (
                self.billing_details.take(),
                self.financial_account.take(),
                self.type_.take(),
                self.us_bank_account.take(),
            )
            else {
                return None;
            };
            Some(Self::Out { billing_details, financial_account, type_, us_bank_account })
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

    impl ObjectDeser for OutboundPaymentsPaymentMethodDetails {
        type Builder = OutboundPaymentsPaymentMethodDetailsBuilder;
    }

    impl FromValueOpt for OutboundPaymentsPaymentMethodDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = OutboundPaymentsPaymentMethodDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "billing_details" => b.billing_details = FromValueOpt::from_value(v),
                    "financial_account" => b.financial_account = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    "us_bank_account" => b.us_bank_account = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The type of the payment method used in the OutboundPayment.
pub type OutboundPaymentsPaymentMethodDetailsType =
    stripe_shared::OutboundPaymentsPaymentMethodDetailsType;

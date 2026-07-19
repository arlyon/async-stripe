#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceInstallments {
    /// Installment plan selected for the payment.
pub plan: Option<stripe_shared::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceInstallmentPlan>,

}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceInstallments
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(
            "PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceInstallments",
        )
        .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceInstallmentsBuilder {
    plan: Option<Option<stripe_shared::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceInstallmentPlan>>,

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

    impl Deserialize
        for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceInstallments
    {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
    out: &'a mut Option<PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceInstallments>,
    builder: PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceInstallmentsBuilder,
}

    impl Visitor
        for Place<
            PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceInstallments,
        >
    {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceInstallmentsBuilder { plan: Deserialize::default(),
 },
        }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "plan" => Deserialize::begin(&mut self.builder.plan),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(plan),) = (self.builder.plan.take(),) else {
                return Ok(());
            };
            *self.out = Some(PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceInstallments { plan });
            Ok(())
        }
    }
};

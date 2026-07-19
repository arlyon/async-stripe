/// Custom Payment Methods represent Payment Method types not modeled directly in
/// the Stripe API. This resource consists of details about the custom payment method
/// used for this payment attempt.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCustomDetails {
    /// Display name for the custom (user-defined) payment method type used to make this payment.
    pub display_name: String,
    /// The custom payment method type associated with this payment.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCustomDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCustomDetails")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCustomDetailsBuilder {
    display_name: Option<String>,
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

    impl Deserialize for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCustomDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCustomDetails>,
        builder: PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCustomDetailsBuilder,
    }

    impl Visitor for Place<PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCustomDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCustomDetailsBuilder {
                        display_name: Deserialize::default(),
                        type_: Deserialize::default(),
                    },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "display_name" => Deserialize::begin(&mut self.builder.display_name),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(display_name), Some(type_)) =
                (self.builder.display_name.take(), self.builder.type_.take())
            else {
                return Ok(());
            };
            *self.out = Some(PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCustomDetails {
                display_name,
                type_,
            });
            Ok(())
        }
    }
};

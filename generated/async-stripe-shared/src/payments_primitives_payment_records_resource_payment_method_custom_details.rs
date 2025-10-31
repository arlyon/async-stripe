/// Custom Payment Methods represent Payment Method types not modeled directly in
/// the Stripe API. This resource consists of details about the custom payment method
/// used for this payment attempt.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCustomDetails {
    /// Display name for the custom (user-defined) payment method type used to make this payment.
    pub display_name: String,
    /// The custom payment method type associated with this payment.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: Option<String>,
}
#[doc(hidden)]
pub struct PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCustomDetailsBuilder {
    display_name: Option<String>,
    type_: Option<Option<String>>,
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
            builder: PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCustomDetailsBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCustomDetailsBuilder {
        type Out = PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCustomDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "display_name" => Deserialize::begin(&mut self.display_name),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { display_name: Deserialize::default(), type_: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(display_name), Some(type_)) = (self.display_name.take(), self.type_.take())
            else {
                return None;
            };
            Some(Self::Out { display_name, type_ })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCustomDetails {
        type Builder = PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCustomDetailsBuilder;
    }

    impl FromValueOpt for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCustomDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCustomDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "display_name" => b.display_name = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};

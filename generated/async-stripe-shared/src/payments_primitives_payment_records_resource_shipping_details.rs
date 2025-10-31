/// The customer's shipping information associated with this payment.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentsPrimitivesPaymentRecordsResourceShippingDetails {
    pub address: stripe_shared::PaymentsPrimitivesPaymentRecordsResourceAddress,
    /// The shipping recipient's name.
    pub name: Option<String>,
    /// The shipping recipient's phone number.
    pub phone: Option<String>,
}
#[doc(hidden)]
pub struct PaymentsPrimitivesPaymentRecordsResourceShippingDetailsBuilder {
    address: Option<stripe_shared::PaymentsPrimitivesPaymentRecordsResourceAddress>,
    name: Option<Option<String>>,
    phone: Option<Option<String>>,
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

    impl Deserialize for PaymentsPrimitivesPaymentRecordsResourceShippingDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentsPrimitivesPaymentRecordsResourceShippingDetails>,
        builder: PaymentsPrimitivesPaymentRecordsResourceShippingDetailsBuilder,
    }

    impl Visitor for Place<PaymentsPrimitivesPaymentRecordsResourceShippingDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    PaymentsPrimitivesPaymentRecordsResourceShippingDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentsPrimitivesPaymentRecordsResourceShippingDetailsBuilder {
        type Out = PaymentsPrimitivesPaymentRecordsResourceShippingDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "address" => Deserialize::begin(&mut self.address),
                "name" => Deserialize::begin(&mut self.name),
                "phone" => Deserialize::begin(&mut self.phone),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                address: Deserialize::default(),
                name: Deserialize::default(),
                phone: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(address), Some(name), Some(phone)) =
                (self.address.take(), self.name.take(), self.phone.take())
            else {
                return None;
            };
            Some(Self::Out { address, name, phone })
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

    impl ObjectDeser for PaymentsPrimitivesPaymentRecordsResourceShippingDetails {
        type Builder = PaymentsPrimitivesPaymentRecordsResourceShippingDetailsBuilder;
    }

    impl FromValueOpt for PaymentsPrimitivesPaymentRecordsResourceShippingDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                PaymentsPrimitivesPaymentRecordsResourceShippingDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "address" => b.address = FromValueOpt::from_value(v),
                    "name" => b.name = FromValueOpt::from_value(v),
                    "phone" => b.phone = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};

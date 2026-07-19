/// The customer's shipping information associated with this payment.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentsPrimitivesPaymentRecordsResourceShippingDetails {
    pub address: stripe_shared::PaymentsPrimitivesPaymentRecordsResourceAddress,
    /// The shipping recipient's name.
    pub name: Option<String>,
    /// The shipping recipient's phone number.
    pub phone: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentsPrimitivesPaymentRecordsResourceShippingDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentsPrimitivesPaymentRecordsResourceShippingDetails")
            .finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: PaymentsPrimitivesPaymentRecordsResourceShippingDetailsBuilder {
                    address: Deserialize::default(),
                    name: Deserialize::default(),
                    phone: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "address" => Deserialize::begin(&mut self.builder.address),
                "name" => Deserialize::begin(&mut self.builder.name),
                "phone" => Deserialize::begin(&mut self.builder.phone),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(address), Some(name), Some(phone)) =
                (self.builder.address.take(), self.builder.name.take(), self.builder.phone.take())
            else {
                return Ok(());
            };
            *self.out = Some(PaymentsPrimitivesPaymentRecordsResourceShippingDetails {
                address,
                name,
                phone,
            });
            Ok(())
        }
    }
};

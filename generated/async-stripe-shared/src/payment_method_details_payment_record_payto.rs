#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsPaymentRecordPayto {
    /// Bank-State-Branch number of the bank account.
    pub bsb_number: Option<String>,
    /// Last four digits of the bank account number.
    pub last4: Option<String>,
    /// ID of the mandate used to make this payment.
    pub mandate: Option<String>,
    /// The PayID alias for the bank account.
    pub pay_id: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodDetailsPaymentRecordPayto {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodDetailsPaymentRecordPayto").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodDetailsPaymentRecordPaytoBuilder {
    bsb_number: Option<Option<String>>,
    last4: Option<Option<String>>,
    mandate: Option<Option<String>>,
    pay_id: Option<Option<String>>,
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

    impl Deserialize for PaymentMethodDetailsPaymentRecordPayto {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsPaymentRecordPayto>,
        builder: PaymentMethodDetailsPaymentRecordPaytoBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsPaymentRecordPayto> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsPaymentRecordPaytoBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsPaymentRecordPaytoBuilder {
        type Out = PaymentMethodDetailsPaymentRecordPayto;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bsb_number" => Deserialize::begin(&mut self.bsb_number),
                "last4" => Deserialize::begin(&mut self.last4),
                "mandate" => Deserialize::begin(&mut self.mandate),
                "pay_id" => Deserialize::begin(&mut self.pay_id),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                bsb_number: Some(None),
                last4: Some(None),
                mandate: Some(None),
                pay_id: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    bsb_number: Some(bsb_number),
                    last4: Some(last4),
                    mandate: Some(mandate),
                    pay_id: Some(pay_id),
                } => Some(Self::Out { bsb_number, last4, mandate, pay_id }),
                builder => {
                    *self = builder;
                    None
                }
            }
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

    impl ObjectDeser for PaymentMethodDetailsPaymentRecordPayto {
        type Builder = PaymentMethodDetailsPaymentRecordPaytoBuilder;
    }

    impl FromValueOpt for PaymentMethodDetailsPaymentRecordPayto {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};

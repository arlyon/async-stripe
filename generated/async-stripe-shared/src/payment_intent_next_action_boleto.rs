#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentIntentNextActionBoleto {
    /// The timestamp after which the boleto expires.
    pub expires_at: Option<stripe_types::Timestamp>,
    /// The URL to the hosted boleto voucher page, which allows customers to view the boleto voucher.
    pub hosted_voucher_url: Option<String>,
    /// The boleto number.
    pub number: Option<String>,
    /// The URL to the downloadable boleto voucher PDF.
    pub pdf: Option<String>,
}
#[doc(hidden)]
pub struct PaymentIntentNextActionBoletoBuilder {
    expires_at: Option<Option<stripe_types::Timestamp>>,
    hosted_voucher_url: Option<Option<String>>,
    number: Option<Option<String>>,
    pdf: Option<Option<String>>,
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

    impl Deserialize for PaymentIntentNextActionBoleto {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextActionBoleto>,
        builder: PaymentIntentNextActionBoletoBuilder,
    }

    impl Visitor for Place<PaymentIntentNextActionBoleto> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentIntentNextActionBoletoBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentIntentNextActionBoletoBuilder {
        type Out = PaymentIntentNextActionBoleto;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "expires_at" => Deserialize::begin(&mut self.expires_at),
                "hosted_voucher_url" => Deserialize::begin(&mut self.hosted_voucher_url),
                "number" => Deserialize::begin(&mut self.number),
                "pdf" => Deserialize::begin(&mut self.pdf),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                expires_at: Deserialize::default(),
                hosted_voucher_url: Deserialize::default(),
                number: Deserialize::default(),
                pdf: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(expires_at), Some(hosted_voucher_url), Some(number), Some(pdf)) = (
                self.expires_at,
                self.hosted_voucher_url.take(),
                self.number.take(),
                self.pdf.take(),
            ) else {
                return None;
            };
            Some(Self::Out { expires_at, hosted_voucher_url, number, pdf })
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

    impl ObjectDeser for PaymentIntentNextActionBoleto {
        type Builder = PaymentIntentNextActionBoletoBuilder;
    }

    impl FromValueOpt for PaymentIntentNextActionBoleto {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentIntentNextActionBoletoBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "expires_at" => b.expires_at = FromValueOpt::from_value(v),
                    "hosted_voucher_url" => b.hosted_voucher_url = FromValueOpt::from_value(v),
                    "number" => b.number = FromValueOpt::from_value(v),
                    "pdf" => b.pdf = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};

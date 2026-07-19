#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentIntentNextActionBoleto {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentIntentNextActionBoleto").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: PaymentIntentNextActionBoletoBuilder {
                    expires_at: Deserialize::default(),
                    hosted_voucher_url: Deserialize::default(),
                    number: Deserialize::default(),
                    pdf: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "expires_at" => Deserialize::begin(&mut self.builder.expires_at),
                "hosted_voucher_url" => Deserialize::begin(&mut self.builder.hosted_voucher_url),
                "number" => Deserialize::begin(&mut self.builder.number),
                "pdf" => Deserialize::begin(&mut self.builder.pdf),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(expires_at), Some(hosted_voucher_url), Some(number), Some(pdf)) = (
                self.builder.expires_at,
                self.builder.hosted_voucher_url.take(),
                self.builder.number.take(),
                self.builder.pdf.take(),
            ) else {
                return Ok(());
            };
            *self.out =
                Some(PaymentIntentNextActionBoleto { expires_at, hosted_voucher_url, number, pdf });
            Ok(())
        }
    }
};

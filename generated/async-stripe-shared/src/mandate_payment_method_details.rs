#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct MandatePaymentMethodDetails {
    pub acss_debit: Option<stripe_shared::MandateAcssDebit>,
    pub amazon_pay: Option<stripe_shared::MandateAmazonPay>,
    pub au_becs_debit: Option<stripe_shared::MandateAuBecsDebit>,
    pub bacs_debit: Option<stripe_shared::MandateBacsDebit>,
    pub card: Option<stripe_shared::CardMandatePaymentMethodDetails>,
    pub cashapp: Option<stripe_shared::MandateCashapp>,
    pub kakao_pay: Option<stripe_shared::MandateKakaoPay>,
    pub klarna: Option<stripe_shared::MandateKlarna>,
    pub kr_card: Option<stripe_shared::MandateKrCard>,
    pub link: Option<stripe_shared::MandateLink>,
    pub naver_pay: Option<stripe_shared::MandateNaverPay>,
    pub nz_bank_account: Option<stripe_shared::MandateNzBankAccount>,
    pub paypal: Option<stripe_shared::MandatePaypal>,
    pub payto: Option<stripe_shared::MandatePayto>,
    pub pix: Option<stripe_shared::MandatePix>,
    pub revolut_pay: Option<stripe_shared::MandateRevolutPay>,
    pub sepa_debit: Option<stripe_shared::MandateSepaDebit>,
    pub twint: Option<stripe_shared::MandateTwint>,
    /// This mandate corresponds with a specific payment method type.
    /// The `payment_method_details` includes an additional hash with the same name and contains mandate information that's specific to that payment method.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: String,
    pub upi: Option<stripe_shared::MandateUpi>,
    pub us_bank_account: Option<stripe_shared::MandateUsBankAccount>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for MandatePaymentMethodDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MandatePaymentMethodDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct MandatePaymentMethodDetailsBuilder {
    acss_debit: Option<stripe_shared::MandateAcssDebit>,
    amazon_pay: Option<stripe_shared::MandateAmazonPay>,
    au_becs_debit: Option<stripe_shared::MandateAuBecsDebit>,
    bacs_debit: Option<stripe_shared::MandateBacsDebit>,
    card: Option<stripe_shared::CardMandatePaymentMethodDetails>,
    cashapp: Option<stripe_shared::MandateCashapp>,
    kakao_pay: Option<stripe_shared::MandateKakaoPay>,
    klarna: Option<stripe_shared::MandateKlarna>,
    kr_card: Option<stripe_shared::MandateKrCard>,
    link: Option<stripe_shared::MandateLink>,
    naver_pay: Option<stripe_shared::MandateNaverPay>,
    nz_bank_account: Option<stripe_shared::MandateNzBankAccount>,
    paypal: Option<stripe_shared::MandatePaypal>,
    payto: Option<stripe_shared::MandatePayto>,
    pix: Option<stripe_shared::MandatePix>,
    revolut_pay: Option<stripe_shared::MandateRevolutPay>,
    sepa_debit: Option<stripe_shared::MandateSepaDebit>,
    twint: Option<stripe_shared::MandateTwint>,
    type_: Option<String>,
    upi: Option<stripe_shared::MandateUpi>,
    us_bank_account: Option<stripe_shared::MandateUsBankAccount>,
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

    impl Deserialize for MandatePaymentMethodDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<MandatePaymentMethodDetails>,
        builder: MandatePaymentMethodDetailsBuilder,
    }

    impl Visitor for Place<MandatePaymentMethodDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: MandatePaymentMethodDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for MandatePaymentMethodDetailsBuilder {
        type Out = MandatePaymentMethodDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "acss_debit" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.acss_debit)
                }
                "amazon_pay" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.amazon_pay)
                }
                "au_becs_debit" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.au_becs_debit)
                }
                "bacs_debit" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.bacs_debit)
                }
                "card" => stripe_types::miniserde_helpers::begin_nullable(&mut self.card),
                "cashapp" => stripe_types::miniserde_helpers::begin_nullable(&mut self.cashapp),
                "kakao_pay" => stripe_types::miniserde_helpers::begin_nullable(&mut self.kakao_pay),
                "klarna" => stripe_types::miniserde_helpers::begin_nullable(&mut self.klarna),
                "kr_card" => stripe_types::miniserde_helpers::begin_nullable(&mut self.kr_card),
                "link" => stripe_types::miniserde_helpers::begin_nullable(&mut self.link),
                "naver_pay" => stripe_types::miniserde_helpers::begin_nullable(&mut self.naver_pay),
                "nz_bank_account" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.nz_bank_account)
                }
                "paypal" => stripe_types::miniserde_helpers::begin_nullable(&mut self.paypal),
                "payto" => stripe_types::miniserde_helpers::begin_nullable(&mut self.payto),
                "pix" => stripe_types::miniserde_helpers::begin_nullable(&mut self.pix),
                "revolut_pay" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.revolut_pay)
                }
                "sepa_debit" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.sepa_debit)
                }
                "twint" => stripe_types::miniserde_helpers::begin_nullable(&mut self.twint),
                "type" => Deserialize::begin(&mut self.type_),
                "upi" => stripe_types::miniserde_helpers::begin_nullable(&mut self.upi),
                "us_bank_account" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.us_bank_account)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                acss_debit: None,
                amazon_pay: None,
                au_becs_debit: None,
                bacs_debit: None,
                card: None,
                cashapp: None,
                kakao_pay: None,
                klarna: None,
                kr_card: None,
                link: None,
                naver_pay: None,
                nz_bank_account: None,
                paypal: None,
                payto: None,
                pix: None,
                revolut_pay: None,
                sepa_debit: None,
                twint: None,
                type_: None,
                upi: None,
                us_bank_account: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    acss_debit,
                    amazon_pay,
                    au_becs_debit,
                    bacs_debit,
                    card,
                    cashapp,
                    kakao_pay,
                    klarna,
                    kr_card,
                    link,
                    naver_pay,
                    nz_bank_account,
                    paypal,
                    payto,
                    pix,
                    revolut_pay,
                    sepa_debit,
                    twint,
                    type_: Some(type_),
                    upi,
                    us_bank_account,
                } => Some(Self::Out {
                    acss_debit,
                    amazon_pay,
                    au_becs_debit,
                    bacs_debit,
                    card,
                    cashapp,
                    kakao_pay,
                    klarna,
                    kr_card,
                    link,
                    naver_pay,
                    nz_bank_account,
                    paypal,
                    payto,
                    pix,
                    revolut_pay,
                    sepa_debit,
                    twint,
                    type_,
                    upi,
                    us_bank_account,
                }),
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

    impl ObjectDeser for MandatePaymentMethodDetails {
        type Builder = MandatePaymentMethodDetailsBuilder;
    }

    impl FromValueOpt for MandatePaymentMethodDetails {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};

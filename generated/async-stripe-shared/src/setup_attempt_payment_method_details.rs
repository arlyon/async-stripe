#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SetupAttemptPaymentMethodDetails {
    pub acss_debit: Option<stripe_shared::SetupAttemptPaymentMethodDetailsAcssDebit>,
    pub amazon_pay: Option<stripe_shared::SetupAttemptPaymentMethodDetailsAmazonPay>,
    pub au_becs_debit: Option<stripe_shared::SetupAttemptPaymentMethodDetailsAuBecsDebit>,
    pub bacs_debit: Option<stripe_shared::SetupAttemptPaymentMethodDetailsBacsDebit>,
    pub bancontact: Option<stripe_shared::SetupAttemptPaymentMethodDetailsBancontact>,
    pub boleto: Option<stripe_shared::SetupAttemptPaymentMethodDetailsBoleto>,
    pub card: Option<stripe_shared::SetupAttemptPaymentMethodDetailsCard>,
    pub card_present: Option<stripe_shared::SetupAttemptPaymentMethodDetailsCardPresent>,
    pub cashapp: Option<stripe_shared::SetupAttemptPaymentMethodDetailsCashapp>,
    pub ideal: Option<stripe_shared::SetupAttemptPaymentMethodDetailsIdeal>,
    pub kakao_pay: Option<stripe_shared::SetupAttemptPaymentMethodDetailsKakaoPay>,
    pub klarna: Option<stripe_shared::SetupAttemptPaymentMethodDetailsKlarna>,
    pub kr_card: Option<stripe_shared::SetupAttemptPaymentMethodDetailsKrCard>,
    pub link: Option<stripe_shared::SetupAttemptPaymentMethodDetailsLink>,
    pub naver_pay: Option<stripe_shared::SetupAttemptPaymentMethodDetailsNaverPay>,
    pub nz_bank_account: Option<stripe_shared::SetupAttemptPaymentMethodDetailsNzBankAccount>,
    pub paypal: Option<stripe_shared::SetupAttemptPaymentMethodDetailsPaypal>,
    pub payto: Option<stripe_shared::SetupAttemptPaymentMethodDetailsPayto>,
    pub pix: Option<stripe_shared::SetupAttemptPaymentMethodDetailsPix>,
    pub revolut_pay: Option<stripe_shared::SetupAttemptPaymentMethodDetailsRevolutPay>,
    pub satispay:
        Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsSatispaySetupAttemptDetails>,
    pub sepa_debit: Option<stripe_shared::SetupAttemptPaymentMethodDetailsSepaDebit>,
    pub sofort: Option<stripe_shared::SetupAttemptPaymentMethodDetailsSofort>,
    pub twint: Option<stripe_shared::SetupAttemptPaymentMethodDetailsTwint>,
    /// The type of the payment method used in the SetupIntent (e.g., `card`).
    /// An additional hash is included on `payment_method_details` with a name matching this value.
    /// It contains confirmation-specific information for the payment method.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: String,
    pub upi: Option<stripe_shared::SetupAttemptPaymentMethodDetailsUpi>,
    pub us_bank_account: Option<stripe_shared::SetupAttemptPaymentMethodDetailsUsBankAccount>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SetupAttemptPaymentMethodDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SetupAttemptPaymentMethodDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SetupAttemptPaymentMethodDetailsBuilder {
    acss_debit: Option<stripe_shared::SetupAttemptPaymentMethodDetailsAcssDebit>,
    amazon_pay: Option<stripe_shared::SetupAttemptPaymentMethodDetailsAmazonPay>,
    au_becs_debit: Option<stripe_shared::SetupAttemptPaymentMethodDetailsAuBecsDebit>,
    bacs_debit: Option<stripe_shared::SetupAttemptPaymentMethodDetailsBacsDebit>,
    bancontact: Option<stripe_shared::SetupAttemptPaymentMethodDetailsBancontact>,
    boleto: Option<stripe_shared::SetupAttemptPaymentMethodDetailsBoleto>,
    card: Option<stripe_shared::SetupAttemptPaymentMethodDetailsCard>,
    card_present: Option<stripe_shared::SetupAttemptPaymentMethodDetailsCardPresent>,
    cashapp: Option<stripe_shared::SetupAttemptPaymentMethodDetailsCashapp>,
    ideal: Option<stripe_shared::SetupAttemptPaymentMethodDetailsIdeal>,
    kakao_pay: Option<stripe_shared::SetupAttemptPaymentMethodDetailsKakaoPay>,
    klarna: Option<stripe_shared::SetupAttemptPaymentMethodDetailsKlarna>,
    kr_card: Option<stripe_shared::SetupAttemptPaymentMethodDetailsKrCard>,
    link: Option<stripe_shared::SetupAttemptPaymentMethodDetailsLink>,
    naver_pay: Option<stripe_shared::SetupAttemptPaymentMethodDetailsNaverPay>,
    nz_bank_account: Option<stripe_shared::SetupAttemptPaymentMethodDetailsNzBankAccount>,
    paypal: Option<stripe_shared::SetupAttemptPaymentMethodDetailsPaypal>,
    payto: Option<stripe_shared::SetupAttemptPaymentMethodDetailsPayto>,
    pix: Option<stripe_shared::SetupAttemptPaymentMethodDetailsPix>,
    revolut_pay: Option<stripe_shared::SetupAttemptPaymentMethodDetailsRevolutPay>,
    satispay: Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsSatispaySetupAttemptDetails>,
    sepa_debit: Option<stripe_shared::SetupAttemptPaymentMethodDetailsSepaDebit>,
    sofort: Option<stripe_shared::SetupAttemptPaymentMethodDetailsSofort>,
    twint: Option<stripe_shared::SetupAttemptPaymentMethodDetailsTwint>,
    type_: Option<String>,
    upi: Option<stripe_shared::SetupAttemptPaymentMethodDetailsUpi>,
    us_bank_account: Option<stripe_shared::SetupAttemptPaymentMethodDetailsUsBankAccount>,
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

    impl Deserialize for SetupAttemptPaymentMethodDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SetupAttemptPaymentMethodDetails>,
        builder: SetupAttemptPaymentMethodDetailsBuilder,
    }

    impl Visitor for Place<SetupAttemptPaymentMethodDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SetupAttemptPaymentMethodDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SetupAttemptPaymentMethodDetailsBuilder {
        type Out = SetupAttemptPaymentMethodDetails;
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
                "bancontact" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.bancontact)
                }
                "boleto" => stripe_types::miniserde_helpers::begin_nullable(&mut self.boleto),
                "card" => stripe_types::miniserde_helpers::begin_nullable(&mut self.card),
                "card_present" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.card_present)
                }
                "cashapp" => stripe_types::miniserde_helpers::begin_nullable(&mut self.cashapp),
                "ideal" => stripe_types::miniserde_helpers::begin_nullable(&mut self.ideal),
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
                "satispay" => stripe_types::miniserde_helpers::begin_nullable(&mut self.satispay),
                "sepa_debit" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.sepa_debit)
                }
                "sofort" => stripe_types::miniserde_helpers::begin_nullable(&mut self.sofort),
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
                bancontact: None,
                boleto: None,
                card: None,
                card_present: None,
                cashapp: None,
                ideal: None,
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
                satispay: None,
                sepa_debit: None,
                sofort: None,
                twint: None,
                type_: None,
                upi: None,
                us_bank_account: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                acss_debit,
                amazon_pay,
                au_becs_debit,
                bacs_debit,
                bancontact,
                boleto,
                card,
                card_present,
                cashapp,
                ideal,
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
                satispay,
                sepa_debit,
                sofort,
                twint,
                Some(type_),
                upi,
                us_bank_account,
            ) = (
                self.acss_debit,
                self.amazon_pay,
                self.au_becs_debit,
                self.bacs_debit,
                self.bancontact.take(),
                self.boleto,
                self.card.take(),
                self.card_present.take(),
                self.cashapp,
                self.ideal.take(),
                self.kakao_pay,
                self.klarna,
                self.kr_card,
                self.link,
                self.naver_pay.take(),
                self.nz_bank_account,
                self.paypal,
                self.payto,
                self.pix.take(),
                self.revolut_pay,
                self.satispay,
                self.sepa_debit,
                self.sofort.take(),
                self.twint,
                self.type_.take(),
                self.upi,
                self.us_bank_account,
            )
            else {
                return None;
            };
            Some(Self::Out {
                acss_debit,
                amazon_pay,
                au_becs_debit,
                bacs_debit,
                bancontact,
                boleto,
                card,
                card_present,
                cashapp,
                ideal,
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
                satispay,
                sepa_debit,
                sofort,
                twint,
                type_,
                upi,
                us_bank_account,
            })
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

    impl ObjectDeser for SetupAttemptPaymentMethodDetails {
        type Builder = SetupAttemptPaymentMethodDetailsBuilder;
    }

    impl FromValueOpt for SetupAttemptPaymentMethodDetails {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};

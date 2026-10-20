#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SetupIntentPaymentMethodOptions {
    pub acss_debit: Option<stripe_shared::SetupIntentPaymentMethodOptionsAcssDebit>,
    pub amazon_pay: Option<stripe_shared::SetupIntentPaymentMethodOptionsAmazonPay>,
    pub bacs_debit: Option<stripe_shared::SetupIntentPaymentMethodOptionsBacsDebit>,
    pub bizum: Option<stripe_shared::SetupIntentPaymentMethodOptionsBizum>,
    pub card: Option<stripe_shared::SetupIntentPaymentMethodOptionsCard>,
    pub card_present: Option<stripe_shared::SetupIntentPaymentMethodOptionsCardPresent>,
    pub klarna: Option<stripe_shared::SetupIntentPaymentMethodOptionsKlarna>,
    pub link: Option<stripe_shared::SetupIntentPaymentMethodOptionsLink>,
    pub paypal: Option<stripe_shared::SetupIntentPaymentMethodOptionsPaypal>,
    pub payto: Option<stripe_shared::SetupIntentPaymentMethodOptionsPayto>,
    pub pix: Option<stripe_shared::SetupIntentPaymentMethodOptionsPix>,
    pub sepa_debit: Option<stripe_shared::SetupIntentPaymentMethodOptionsSepaDebit>,
    pub upi: Option<stripe_shared::SetupIntentPaymentMethodOptionsUpi>,
    pub us_bank_account: Option<stripe_shared::SetupIntentPaymentMethodOptionsUsBankAccount>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SetupIntentPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SetupIntentPaymentMethodOptions").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SetupIntentPaymentMethodOptionsBuilder {
    acss_debit: Option<stripe_shared::SetupIntentPaymentMethodOptionsAcssDebit>,
    amazon_pay: Option<stripe_shared::SetupIntentPaymentMethodOptionsAmazonPay>,
    bacs_debit: Option<stripe_shared::SetupIntentPaymentMethodOptionsBacsDebit>,
    bizum: Option<stripe_shared::SetupIntentPaymentMethodOptionsBizum>,
    card: Option<stripe_shared::SetupIntentPaymentMethodOptionsCard>,
    card_present: Option<stripe_shared::SetupIntentPaymentMethodOptionsCardPresent>,
    klarna: Option<stripe_shared::SetupIntentPaymentMethodOptionsKlarna>,
    link: Option<stripe_shared::SetupIntentPaymentMethodOptionsLink>,
    paypal: Option<stripe_shared::SetupIntentPaymentMethodOptionsPaypal>,
    payto: Option<stripe_shared::SetupIntentPaymentMethodOptionsPayto>,
    pix: Option<stripe_shared::SetupIntentPaymentMethodOptionsPix>,
    sepa_debit: Option<stripe_shared::SetupIntentPaymentMethodOptionsSepaDebit>,
    upi: Option<stripe_shared::SetupIntentPaymentMethodOptionsUpi>,
    us_bank_account: Option<stripe_shared::SetupIntentPaymentMethodOptionsUsBankAccount>,
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

    impl Deserialize for SetupIntentPaymentMethodOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SetupIntentPaymentMethodOptions>,
        builder: SetupIntentPaymentMethodOptionsBuilder,
    }

    impl Visitor for Place<SetupIntentPaymentMethodOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SetupIntentPaymentMethodOptionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SetupIntentPaymentMethodOptionsBuilder {
        type Out = SetupIntentPaymentMethodOptions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "acss_debit" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.acss_debit)
                }
                "amazon_pay" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.amazon_pay)
                }
                "bacs_debit" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.bacs_debit)
                }
                "bizum" => stripe_types::miniserde_helpers::begin_nullable(&mut self.bizum),
                "card" => stripe_types::miniserde_helpers::begin_nullable(&mut self.card),
                "card_present" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.card_present)
                }
                "klarna" => stripe_types::miniserde_helpers::begin_nullable(&mut self.klarna),
                "link" => stripe_types::miniserde_helpers::begin_nullable(&mut self.link),
                "paypal" => stripe_types::miniserde_helpers::begin_nullable(&mut self.paypal),
                "payto" => stripe_types::miniserde_helpers::begin_nullable(&mut self.payto),
                "pix" => stripe_types::miniserde_helpers::begin_nullable(&mut self.pix),
                "sepa_debit" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.sepa_debit)
                }
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
                bacs_debit: None,
                bizum: None,
                card: None,
                card_present: None,
                klarna: None,
                link: None,
                paypal: None,
                payto: None,
                pix: None,
                sepa_debit: None,
                upi: None,
                us_bank_account: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                acss_debit,
                amazon_pay,
                bacs_debit,
                bizum,
                card,
                card_present,
                klarna,
                link,
                paypal,
                payto,
                pix,
                sepa_debit,
                upi,
                us_bank_account,
            ) = (
                self.acss_debit.take(),
                self.amazon_pay,
                self.bacs_debit.take(),
                self.bizum,
                self.card.take(),
                self.card_present,
                self.klarna.take(),
                self.link.take(),
                self.paypal.take(),
                self.payto.take(),
                self.pix.take(),
                self.sepa_debit.take(),
                self.upi.take(),
                self.us_bank_account.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                acss_debit,
                amazon_pay,
                bacs_debit,
                bizum,
                card,
                card_present,
                klarna,
                link,
                paypal,
                payto,
                pix,
                sepa_debit,
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

    impl ObjectDeser for SetupIntentPaymentMethodOptions {
        type Builder = SetupIntentPaymentMethodOptionsBuilder;
    }

    impl FromValueOpt for SetupIntentPaymentMethodOptions {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};

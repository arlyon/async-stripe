#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SetupIntentPaymentMethodOptions {
    pub acss_debit: Option<stripe_shared::SetupIntentPaymentMethodOptionsAcssDebit>,
    pub amazon_pay: Option<stripe_shared::SetupIntentPaymentMethodOptionsAmazonPay>,
    pub bacs_debit: Option<stripe_shared::SetupIntentPaymentMethodOptionsBacsDebit>,
    pub card: Option<stripe_shared::SetupIntentPaymentMethodOptionsCard>,
    pub card_present: Option<stripe_shared::SetupIntentPaymentMethodOptionsCardPresent>,
    pub klarna: Option<stripe_shared::SetupIntentPaymentMethodOptionsKlarna>,
    pub link: Option<stripe_shared::SetupIntentPaymentMethodOptionsLink>,
    pub paypal: Option<stripe_shared::SetupIntentPaymentMethodOptionsPaypal>,
    pub payto: Option<stripe_shared::SetupIntentPaymentMethodOptionsPayto>,
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
    acss_debit: Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsAcssDebit>>,
    amazon_pay: Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsAmazonPay>>,
    bacs_debit: Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsBacsDebit>>,
    card: Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsCard>>,
    card_present: Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsCardPresent>>,
    klarna: Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsKlarna>>,
    link: Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsLink>>,
    paypal: Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsPaypal>>,
    payto: Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsPayto>>,
    sepa_debit: Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsSepaDebit>>,
    upi: Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsUpi>>,
    us_bank_account: Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsUsBankAccount>>,
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
                builder: SetupIntentPaymentMethodOptionsBuilder {
                    acss_debit: Deserialize::default(),
                    amazon_pay: Deserialize::default(),
                    bacs_debit: Deserialize::default(),
                    card: Deserialize::default(),
                    card_present: Deserialize::default(),
                    klarna: Deserialize::default(),
                    link: Deserialize::default(),
                    paypal: Deserialize::default(),
                    payto: Deserialize::default(),
                    sepa_debit: Deserialize::default(),
                    upi: Deserialize::default(),
                    us_bank_account: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "acss_debit" => Deserialize::begin(&mut self.builder.acss_debit),
                "amazon_pay" => Deserialize::begin(&mut self.builder.amazon_pay),
                "bacs_debit" => Deserialize::begin(&mut self.builder.bacs_debit),
                "card" => Deserialize::begin(&mut self.builder.card),
                "card_present" => Deserialize::begin(&mut self.builder.card_present),
                "klarna" => Deserialize::begin(&mut self.builder.klarna),
                "link" => Deserialize::begin(&mut self.builder.link),
                "paypal" => Deserialize::begin(&mut self.builder.paypal),
                "payto" => Deserialize::begin(&mut self.builder.payto),
                "sepa_debit" => Deserialize::begin(&mut self.builder.sepa_debit),
                "upi" => Deserialize::begin(&mut self.builder.upi),
                "us_bank_account" => Deserialize::begin(&mut self.builder.us_bank_account),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(acss_debit),
                Some(amazon_pay),
                Some(bacs_debit),
                Some(card),
                Some(card_present),
                Some(klarna),
                Some(link),
                Some(paypal),
                Some(payto),
                Some(sepa_debit),
                Some(upi),
                Some(us_bank_account),
            ) = (
                self.builder.acss_debit.take(),
                self.builder.amazon_pay,
                self.builder.bacs_debit.take(),
                self.builder.card.take(),
                self.builder.card_present,
                self.builder.klarna.take(),
                self.builder.link.take(),
                self.builder.paypal.take(),
                self.builder.payto.take(),
                self.builder.sepa_debit.take(),
                self.builder.upi.take(),
                self.builder.us_bank_account.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(SetupIntentPaymentMethodOptions {
                acss_debit,
                amazon_pay,
                bacs_debit,
                card,
                card_present,
                klarna,
                link,
                paypal,
                payto,
                sepa_debit,
                upi,
                us_bank_account,
            });
            Ok(())
        }
    }
};

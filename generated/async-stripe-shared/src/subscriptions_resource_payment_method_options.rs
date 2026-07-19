#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionsResourcePaymentMethodOptions {
    /// This sub-hash contains details about the Canadian pre-authorized debit payment method options to pass to invoices created by the subscription.
    pub acss_debit: Option<stripe_shared::InvoicePaymentMethodOptionsAcssDebit>,
    /// This sub-hash contains details about the Bancontact payment method options to pass to invoices created by the subscription.
    pub bancontact: Option<stripe_shared::InvoicePaymentMethodOptionsBancontact>,
    /// This sub-hash contains details about the Card payment method options to pass to invoices created by the subscription.
    pub card: Option<stripe_shared::SubscriptionPaymentMethodOptionsCard>,
    /// This sub-hash contains details about the Bank transfer payment method options to pass to invoices created by the subscription.
    pub customer_balance: Option<stripe_shared::InvoicePaymentMethodOptionsCustomerBalance>,
    /// This sub-hash contains details about the Konbini payment method options to pass to invoices created by the subscription.
    pub konbini: Option<stripe_shared::InvoicePaymentMethodOptionsKonbini>,
    /// This sub-hash contains details about the PayTo payment method options to pass to invoices created by the subscription.
    pub payto: Option<stripe_shared::InvoicePaymentMethodOptionsPayto>,
    /// This sub-hash contains details about the SEPA Direct Debit payment method options to pass to invoices created by the subscription.
    pub sepa_debit: Option<stripe_shared::InvoicePaymentMethodOptionsSepaDebit>,
    /// This sub-hash contains details about the ACH direct debit payment method options to pass to invoices created by the subscription.
    pub us_bank_account: Option<stripe_shared::InvoicePaymentMethodOptionsUsBankAccount>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionsResourcePaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionsResourcePaymentMethodOptions").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionsResourcePaymentMethodOptionsBuilder {
    acss_debit: Option<Option<stripe_shared::InvoicePaymentMethodOptionsAcssDebit>>,
    bancontact: Option<Option<stripe_shared::InvoicePaymentMethodOptionsBancontact>>,
    card: Option<Option<stripe_shared::SubscriptionPaymentMethodOptionsCard>>,
    customer_balance: Option<Option<stripe_shared::InvoicePaymentMethodOptionsCustomerBalance>>,
    konbini: Option<Option<stripe_shared::InvoicePaymentMethodOptionsKonbini>>,
    payto: Option<Option<stripe_shared::InvoicePaymentMethodOptionsPayto>>,
    sepa_debit: Option<Option<stripe_shared::InvoicePaymentMethodOptionsSepaDebit>>,
    us_bank_account: Option<Option<stripe_shared::InvoicePaymentMethodOptionsUsBankAccount>>,
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

    impl Deserialize for SubscriptionsResourcePaymentMethodOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionsResourcePaymentMethodOptions>,
        builder: SubscriptionsResourcePaymentMethodOptionsBuilder,
    }

    impl Visitor for Place<SubscriptionsResourcePaymentMethodOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionsResourcePaymentMethodOptionsBuilder {
                    acss_debit: Deserialize::default(),
                    bancontact: Deserialize::default(),
                    card: Deserialize::default(),
                    customer_balance: Deserialize::default(),
                    konbini: Deserialize::default(),
                    payto: Deserialize::default(),
                    sepa_debit: Deserialize::default(),
                    us_bank_account: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "acss_debit" => Deserialize::begin(&mut self.builder.acss_debit),
                "bancontact" => Deserialize::begin(&mut self.builder.bancontact),
                "card" => Deserialize::begin(&mut self.builder.card),
                "customer_balance" => Deserialize::begin(&mut self.builder.customer_balance),
                "konbini" => Deserialize::begin(&mut self.builder.konbini),
                "payto" => Deserialize::begin(&mut self.builder.payto),
                "sepa_debit" => Deserialize::begin(&mut self.builder.sepa_debit),
                "us_bank_account" => Deserialize::begin(&mut self.builder.us_bank_account),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(acss_debit),
                Some(bancontact),
                Some(card),
                Some(customer_balance),
                Some(konbini),
                Some(payto),
                Some(sepa_debit),
                Some(us_bank_account),
            ) = (
                self.builder.acss_debit.take(),
                self.builder.bancontact.take(),
                self.builder.card.take(),
                self.builder.customer_balance.take(),
                self.builder.konbini,
                self.builder.payto.take(),
                self.builder.sepa_debit,
                self.builder.us_bank_account.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(SubscriptionsResourcePaymentMethodOptions {
                acss_debit,
                bancontact,
                card,
                customer_balance,
                konbini,
                payto,
                sepa_debit,
                us_bank_account,
            });
            Ok(())
        }
    }
};

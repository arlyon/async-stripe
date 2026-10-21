#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionsResourcePaymentMethodOptions {
    /// This sub-hash contains details about the Canadian pre-authorized debit payment method options to pass to invoices created by the subscription.
    pub acss_debit: Option<stripe_shared::InvoicePaymentMethodOptionsAcssDebit>,
    /// This sub-hash contains details about the Bancontact payment method options to pass to invoices created by the subscription.
    pub bancontact: Option<stripe_shared::InvoicePaymentMethodOptionsBancontact>,
    /// This sub-hash contains details about the Billie payment method options to pass to invoices created by the subscription.
    pub billie: Option<stripe_shared::SubscriptionPaymentMethodOptionsBillie>,
    /// This sub-hash contains details about the Card payment method options to pass to invoices created by the subscription.
    pub card: Option<stripe_shared::SubscriptionPaymentMethodOptionsCard>,
    /// This sub-hash contains details about the Bank transfer payment method options to pass to invoices created by the subscription.
    pub customer_balance: Option<stripe_shared::InvoicePaymentMethodOptionsCustomerBalance>,
    /// This sub-hash contains details about the Konbini payment method options to pass to invoices created by the subscription.
    pub konbini: Option<stripe_shared::InvoicePaymentMethodOptionsKonbini>,
    /// This sub-hash contains details about the PayTo payment method options to pass to invoices created by the subscription.
    pub payto: Option<stripe_shared::InvoicePaymentMethodOptionsPayto>,
    /// This sub-hash contains details about the Pix payment method options to pass to invoices created by the subscription.
    pub pix: Option<stripe_shared::SubscriptionPaymentMethodOptionsPix>,
    /// This sub-hash contains details about the SEPA Direct Debit payment method options to pass to invoices created by the subscription.
    pub sepa_debit: Option<stripe_shared::InvoicePaymentMethodOptionsSepaDebit>,
    /// This sub-hash contains details about the UPI payment method options to pass to invoices created by the subscription.
    pub upi: Option<stripe_shared::InvoicePaymentMethodOptionsUpi>,
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
    acss_debit: Option<stripe_shared::InvoicePaymentMethodOptionsAcssDebit>,
    bancontact: Option<stripe_shared::InvoicePaymentMethodOptionsBancontact>,
    billie: Option<stripe_shared::SubscriptionPaymentMethodOptionsBillie>,
    card: Option<stripe_shared::SubscriptionPaymentMethodOptionsCard>,
    customer_balance: Option<stripe_shared::InvoicePaymentMethodOptionsCustomerBalance>,
    konbini: Option<stripe_shared::InvoicePaymentMethodOptionsKonbini>,
    payto: Option<stripe_shared::InvoicePaymentMethodOptionsPayto>,
    pix: Option<stripe_shared::SubscriptionPaymentMethodOptionsPix>,
    sepa_debit: Option<stripe_shared::InvoicePaymentMethodOptionsSepaDebit>,
    upi: Option<stripe_shared::InvoicePaymentMethodOptionsUpi>,
    us_bank_account: Option<stripe_shared::InvoicePaymentMethodOptionsUsBankAccount>,
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
                builder: SubscriptionsResourcePaymentMethodOptionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SubscriptionsResourcePaymentMethodOptionsBuilder {
        type Out = SubscriptionsResourcePaymentMethodOptions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "acss_debit" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.acss_debit)
                }
                "bancontact" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.bancontact)
                }
                "billie" => stripe_types::miniserde_helpers::begin_nullable(&mut self.billie),
                "card" => stripe_types::miniserde_helpers::begin_nullable(&mut self.card),
                "customer_balance" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.customer_balance)
                }
                "konbini" => stripe_types::miniserde_helpers::begin_nullable(&mut self.konbini),
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
                bancontact: None,
                billie: None,
                card: None,
                customer_balance: None,
                konbini: None,
                payto: None,
                pix: None,
                sepa_debit: None,
                upi: None,
                us_bank_account: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let Self {
                acss_debit,
                bancontact,
                billie,
                card,
                customer_balance,
                konbini,
                payto,
                pix,
                sepa_debit,
                upi,
                us_bank_account,
            } = core::mem::replace(self, Self::deser_default());
            Some(Self::Out {
                acss_debit,
                bancontact,
                billie,
                card,
                customer_balance,
                konbini,
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

    impl ObjectDeser for SubscriptionsResourcePaymentMethodOptions {
        type Builder = SubscriptionsResourcePaymentMethodOptionsBuilder;
    }

    impl FromValueOpt for SubscriptionsResourcePaymentMethodOptions {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountBacsDebitPaymentsSettings {
    /// The Bacs Direct Debit display name for this account.
    /// For payments made with Bacs Direct Debit, this name appears on the mandate as the statement descriptor.
    /// Mobile banking apps display it as the name of the business.
    /// To use custom branding, set the Bacs Direct Debit Display Name during or right after creation.
    /// Custom branding incurs an additional monthly fee for the platform.
    /// The fee appears 5 business days after requesting Bacs.
    /// If you don't set the display name before requesting Bacs capability, it's automatically set as "Stripe" and the account is onboarded to Stripe branding, which is free.
    pub display_name: Option<String>,
    /// The Bacs Direct Debit Service user number for this account.
    /// For payments made with Bacs Direct Debit, this number is a unique identifier of the account with our banking partners.
    pub service_user_number: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AccountBacsDebitPaymentsSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccountBacsDebitPaymentsSettings").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct AccountBacsDebitPaymentsSettingsBuilder {
    display_name: Option<Option<String>>,
    service_user_number: Option<Option<String>>,
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

    impl Deserialize for AccountBacsDebitPaymentsSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountBacsDebitPaymentsSettings>,
        builder: AccountBacsDebitPaymentsSettingsBuilder,
    }

    impl Visitor for Place<AccountBacsDebitPaymentsSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: AccountBacsDebitPaymentsSettingsBuilder {
                    display_name: Deserialize::default(),
                    service_user_number: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "display_name" => Deserialize::begin(&mut self.builder.display_name),
                "service_user_number" => Deserialize::begin(&mut self.builder.service_user_number),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(display_name), Some(service_user_number)) =
                (self.builder.display_name.take(), self.builder.service_user_number.take())
            else {
                return Ok(());
            };
            *self.out =
                Some(AccountBacsDebitPaymentsSettings { display_name, service_user_number });
            Ok(())
        }
    }
};

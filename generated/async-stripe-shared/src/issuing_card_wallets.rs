#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingCardWallets {
    pub apple_pay: stripe_shared::IssuingCardApplePay,
    pub google_pay: stripe_shared::IssuingCardGooglePay,
    /// Unique identifier for a card used with digital wallets
    pub primary_account_identifier: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingCardWallets {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingCardWallets").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingCardWalletsBuilder {
    apple_pay: Option<stripe_shared::IssuingCardApplePay>,
    google_pay: Option<stripe_shared::IssuingCardGooglePay>,
    primary_account_identifier: Option<Option<String>>,
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

    impl Deserialize for IssuingCardWallets {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingCardWallets>,
        builder: IssuingCardWalletsBuilder,
    }

    impl Visitor for Place<IssuingCardWallets> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingCardWalletsBuilder {
                    apple_pay: Deserialize::default(),
                    google_pay: Deserialize::default(),
                    primary_account_identifier: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "apple_pay" => Deserialize::begin(&mut self.builder.apple_pay),
                "google_pay" => Deserialize::begin(&mut self.builder.google_pay),
                "primary_account_identifier" => {
                    Deserialize::begin(&mut self.builder.primary_account_identifier)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(apple_pay), Some(google_pay), Some(primary_account_identifier)) = (
                self.builder.apple_pay.take(),
                self.builder.google_pay.take(),
                self.builder.primary_account_identifier.take(),
            ) else {
                return Ok(());
            };
            *self.out =
                Some(IssuingCardWallets { apple_pay, google_pay, primary_account_identifier });
            Ok(())
        }
    }
};

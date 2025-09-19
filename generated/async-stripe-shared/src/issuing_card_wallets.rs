#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingCardWallets {
    pub apple_pay: stripe_shared::IssuingCardApplePay,
    pub google_pay: stripe_shared::IssuingCardGooglePay,
    /// Unique identifier for a card used with digital wallets
    pub primary_account_identifier: Option<String>,
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
                builder: IssuingCardWalletsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingCardWalletsBuilder {
        type Out = IssuingCardWallets;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "apple_pay" => Deserialize::begin(&mut self.apple_pay),
                "google_pay" => Deserialize::begin(&mut self.google_pay),
                "primary_account_identifier" => {
                    Deserialize::begin(&mut self.primary_account_identifier)
                }

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                apple_pay: Deserialize::default(),
                google_pay: Deserialize::default(),
                primary_account_identifier: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(apple_pay), Some(google_pay), Some(primary_account_identifier)) =
                (self.apple_pay, self.google_pay, self.primary_account_identifier.take())
            else {
                return None;
            };
            Some(Self::Out { apple_pay, google_pay, primary_account_identifier })
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

    impl ObjectDeser for IssuingCardWallets {
        type Builder = IssuingCardWalletsBuilder;
    }

    impl FromValueOpt for IssuingCardWallets {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingCardWalletsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "apple_pay" => b.apple_pay = FromValueOpt::from_value(v),
                    "google_pay" => b.google_pay = FromValueOpt::from_value(v),
                    "primary_account_identifier" => {
                        b.primary_account_identifier = FromValueOpt::from_value(v)
                    }

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountPaymentsSettings {
    /// The default text that appears on credit card statements when a charge is made.
    /// This field prefixes any dynamic `statement_descriptor` specified on the charge.
    pub statement_descriptor: Option<String>,
    /// The Kana variation of `statement_descriptor` used for charges in Japan.
    /// Japanese statement descriptors have [special requirements](https://docs.stripe.com/get-started/account/statement-descriptors#set-japanese-statement-descriptors).
    pub statement_descriptor_kana: Option<String>,
    /// The Kanji variation of `statement_descriptor` used for charges in Japan.
    /// Japanese statement descriptors have [special requirements](https://docs.stripe.com/get-started/account/statement-descriptors#set-japanese-statement-descriptors).
    pub statement_descriptor_kanji: Option<String>,
    /// The Kana variation of `statement_descriptor_prefix` used for card charges in Japan.
    /// Japanese statement descriptors have [special requirements](https://docs.stripe.com/get-started/account/statement-descriptors#set-japanese-statement-descriptors).
    pub statement_descriptor_prefix_kana: Option<String>,
    /// The Kanji variation of `statement_descriptor_prefix` used for card charges in Japan.
    /// Japanese statement descriptors have [special requirements](https://docs.stripe.com/get-started/account/statement-descriptors#set-japanese-statement-descriptors).
    pub statement_descriptor_prefix_kanji: Option<String>,
}
#[doc(hidden)]
pub struct AccountPaymentsSettingsBuilder {
    statement_descriptor: Option<Option<String>>,
    statement_descriptor_kana: Option<Option<String>>,
    statement_descriptor_kanji: Option<Option<String>>,
    statement_descriptor_prefix_kana: Option<Option<String>>,
    statement_descriptor_prefix_kanji: Option<Option<String>>,
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

    impl Deserialize for AccountPaymentsSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountPaymentsSettings>,
        builder: AccountPaymentsSettingsBuilder,
    }

    impl Visitor for Place<AccountPaymentsSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: AccountPaymentsSettingsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for AccountPaymentsSettingsBuilder {
        type Out = AccountPaymentsSettings;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "statement_descriptor" => Deserialize::begin(&mut self.statement_descriptor),
                "statement_descriptor_kana" => {
                    Deserialize::begin(&mut self.statement_descriptor_kana)
                }
                "statement_descriptor_kanji" => {
                    Deserialize::begin(&mut self.statement_descriptor_kanji)
                }
                "statement_descriptor_prefix_kana" => {
                    Deserialize::begin(&mut self.statement_descriptor_prefix_kana)
                }
                "statement_descriptor_prefix_kanji" => {
                    Deserialize::begin(&mut self.statement_descriptor_prefix_kanji)
                }

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                statement_descriptor: Deserialize::default(),
                statement_descriptor_kana: Deserialize::default(),
                statement_descriptor_kanji: Deserialize::default(),
                statement_descriptor_prefix_kana: Deserialize::default(),
                statement_descriptor_prefix_kanji: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(statement_descriptor),
                Some(statement_descriptor_kana),
                Some(statement_descriptor_kanji),
                Some(statement_descriptor_prefix_kana),
                Some(statement_descriptor_prefix_kanji),
            ) = (
                self.statement_descriptor.take(),
                self.statement_descriptor_kana.take(),
                self.statement_descriptor_kanji.take(),
                self.statement_descriptor_prefix_kana.take(),
                self.statement_descriptor_prefix_kanji.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                statement_descriptor,
                statement_descriptor_kana,
                statement_descriptor_kanji,
                statement_descriptor_prefix_kana,
                statement_descriptor_prefix_kanji,
            })
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

    impl ObjectDeser for AccountPaymentsSettings {
        type Builder = AccountPaymentsSettingsBuilder;
    }

    impl FromValueOpt for AccountPaymentsSettings {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = AccountPaymentsSettingsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "statement_descriptor" => b.statement_descriptor = FromValueOpt::from_value(v),
                    "statement_descriptor_kana" => {
                        b.statement_descriptor_kana = FromValueOpt::from_value(v)
                    }
                    "statement_descriptor_kanji" => {
                        b.statement_descriptor_kanji = FromValueOpt::from_value(v)
                    }
                    "statement_descriptor_prefix_kana" => {
                        b.statement_descriptor_prefix_kana = FromValueOpt::from_value(v)
                    }
                    "statement_descriptor_prefix_kanji" => {
                        b.statement_descriptor_prefix_kanji = FromValueOpt::from_value(v)
                    }

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};

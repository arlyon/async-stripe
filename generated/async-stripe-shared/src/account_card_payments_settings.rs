#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountCardPaymentsSettings {
    pub decline_on: Option<stripe_shared::AccountDeclineChargeOn>,
    /// The default text that appears on credit card statements when a charge is made.
    /// This field prefixes any dynamic `statement_descriptor` specified on the charge.
    /// `statement_descriptor_prefix` is useful for maximizing descriptor space for the dynamic portion.
    pub statement_descriptor_prefix: Option<String>,
    /// The Kana variation of the default text that appears on credit card statements when a charge is made (Japan only).
    /// This field prefixes any dynamic `statement_descriptor_suffix_kana` specified on the charge.
    /// `statement_descriptor_prefix_kana` is useful for maximizing descriptor space for the dynamic portion.
    pub statement_descriptor_prefix_kana: Option<String>,
    /// The Kanji variation of the default text that appears on credit card statements when a charge is made (Japan only).
    /// This field prefixes any dynamic `statement_descriptor_suffix_kanji` specified on the charge.
    /// `statement_descriptor_prefix_kanji` is useful for maximizing descriptor space for the dynamic portion.
    pub statement_descriptor_prefix_kanji: Option<String>,
}
#[doc(hidden)]
pub struct AccountCardPaymentsSettingsBuilder {
    decline_on: Option<Option<stripe_shared::AccountDeclineChargeOn>>,
    statement_descriptor_prefix: Option<Option<String>>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for AccountCardPaymentsSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountCardPaymentsSettings>,
        builder: AccountCardPaymentsSettingsBuilder,
    }

    impl Visitor for Place<AccountCardPaymentsSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: AccountCardPaymentsSettingsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for AccountCardPaymentsSettingsBuilder {
        type Out = AccountCardPaymentsSettings;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "decline_on" => Deserialize::begin(&mut self.decline_on),
                "statement_descriptor_prefix" => {
                    Deserialize::begin(&mut self.statement_descriptor_prefix)
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
                decline_on: Deserialize::default(),
                statement_descriptor_prefix: Deserialize::default(),
                statement_descriptor_prefix_kana: Deserialize::default(),
                statement_descriptor_prefix_kanji: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(decline_on),
                Some(statement_descriptor_prefix),
                Some(statement_descriptor_prefix_kana),
                Some(statement_descriptor_prefix_kanji),
            ) = (
                self.decline_on,
                self.statement_descriptor_prefix.take(),
                self.statement_descriptor_prefix_kana.take(),
                self.statement_descriptor_prefix_kanji.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                decline_on,
                statement_descriptor_prefix,
                statement_descriptor_prefix_kana,
                statement_descriptor_prefix_kanji,
            })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for AccountCardPaymentsSettings {
        type Builder = AccountCardPaymentsSettingsBuilder;
    }

    impl FromValueOpt for AccountCardPaymentsSettings {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = AccountCardPaymentsSettingsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "decline_on" => b.decline_on = FromValueOpt::from_value(v),
                    "statement_descriptor_prefix" => {
                        b.statement_descriptor_prefix = FromValueOpt::from_value(v)
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

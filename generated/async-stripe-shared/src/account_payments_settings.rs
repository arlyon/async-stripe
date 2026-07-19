#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AccountPaymentsSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccountPaymentsSettings").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: AccountPaymentsSettingsBuilder {
                    statement_descriptor: Deserialize::default(),
                    statement_descriptor_kana: Deserialize::default(),
                    statement_descriptor_kanji: Deserialize::default(),
                    statement_descriptor_prefix_kana: Deserialize::default(),
                    statement_descriptor_prefix_kanji: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "statement_descriptor" => {
                    Deserialize::begin(&mut self.builder.statement_descriptor)
                }
                "statement_descriptor_kana" => {
                    Deserialize::begin(&mut self.builder.statement_descriptor_kana)
                }
                "statement_descriptor_kanji" => {
                    Deserialize::begin(&mut self.builder.statement_descriptor_kanji)
                }
                "statement_descriptor_prefix_kana" => {
                    Deserialize::begin(&mut self.builder.statement_descriptor_prefix_kana)
                }
                "statement_descriptor_prefix_kanji" => {
                    Deserialize::begin(&mut self.builder.statement_descriptor_prefix_kanji)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(statement_descriptor),
                Some(statement_descriptor_kana),
                Some(statement_descriptor_kanji),
                Some(statement_descriptor_prefix_kana),
                Some(statement_descriptor_prefix_kanji),
            ) = (
                self.builder.statement_descriptor.take(),
                self.builder.statement_descriptor_kana.take(),
                self.builder.statement_descriptor_kanji.take(),
                self.builder.statement_descriptor_prefix_kana.take(),
                self.builder.statement_descriptor_prefix_kanji.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(AccountPaymentsSettings {
                statement_descriptor,
                statement_descriptor_kana,
                statement_descriptor_kanji,
                statement_descriptor_prefix_kana,
                statement_descriptor_prefix_kanji,
            });
            Ok(())
        }
    }
};

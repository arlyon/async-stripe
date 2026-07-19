#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AccountCardPaymentsSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccountCardPaymentsSettings").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: AccountCardPaymentsSettingsBuilder {
                    decline_on: Deserialize::default(),
                    statement_descriptor_prefix: Deserialize::default(),
                    statement_descriptor_prefix_kana: Deserialize::default(),
                    statement_descriptor_prefix_kanji: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "decline_on" => Deserialize::begin(&mut self.builder.decline_on),
                "statement_descriptor_prefix" => {
                    Deserialize::begin(&mut self.builder.statement_descriptor_prefix)
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
                Some(decline_on),
                Some(statement_descriptor_prefix),
                Some(statement_descriptor_prefix_kana),
                Some(statement_descriptor_prefix_kanji),
            ) = (
                self.builder.decline_on,
                self.builder.statement_descriptor_prefix.take(),
                self.builder.statement_descriptor_prefix_kana.take(),
                self.builder.statement_descriptor_prefix_kanji.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(AccountCardPaymentsSettings {
                decline_on,
                statement_descriptor_prefix,
                statement_descriptor_prefix_kana,
                statement_descriptor_prefix_kanji,
            });
            Ok(())
        }
    }
};

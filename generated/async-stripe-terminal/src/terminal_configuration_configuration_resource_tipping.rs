#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalConfigurationConfigurationResourceTipping {
    pub aed:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    pub aud:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    pub cad:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    pub chf:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    pub czk:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    pub dkk:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    pub eur:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    pub gbp:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    pub gip:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    pub hkd:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    pub huf:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    pub jpy:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    pub mxn:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    pub myr:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    pub nok:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    pub nzd:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    pub pln:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    pub ron:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    pub sek:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    pub sgd:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    pub usd:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalConfigurationConfigurationResourceTipping {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TerminalConfigurationConfigurationResourceTipping").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TerminalConfigurationConfigurationResourceTippingBuilder {
    aed: Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    aud: Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    cad: Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    chf: Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    czk: Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    dkk: Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    eur: Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    gbp: Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    gip: Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    hkd: Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    huf: Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    jpy: Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    mxn: Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    myr: Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    nok: Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    nzd: Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    pln: Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    ron: Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    sek: Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    sgd: Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    usd: Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
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

    impl Deserialize for TerminalConfigurationConfigurationResourceTipping {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalConfigurationConfigurationResourceTipping>,
        builder: TerminalConfigurationConfigurationResourceTippingBuilder,
    }

    impl Visitor for Place<TerminalConfigurationConfigurationResourceTipping> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TerminalConfigurationConfigurationResourceTippingBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TerminalConfigurationConfigurationResourceTippingBuilder {
        type Out = TerminalConfigurationConfigurationResourceTipping;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "aed" => stripe_types::miniserde_helpers::begin_nullable(&mut self.aed),
                "aud" => stripe_types::miniserde_helpers::begin_nullable(&mut self.aud),
                "cad" => stripe_types::miniserde_helpers::begin_nullable(&mut self.cad),
                "chf" => stripe_types::miniserde_helpers::begin_nullable(&mut self.chf),
                "czk" => stripe_types::miniserde_helpers::begin_nullable(&mut self.czk),
                "dkk" => stripe_types::miniserde_helpers::begin_nullable(&mut self.dkk),
                "eur" => stripe_types::miniserde_helpers::begin_nullable(&mut self.eur),
                "gbp" => stripe_types::miniserde_helpers::begin_nullable(&mut self.gbp),
                "gip" => stripe_types::miniserde_helpers::begin_nullable(&mut self.gip),
                "hkd" => stripe_types::miniserde_helpers::begin_nullable(&mut self.hkd),
                "huf" => stripe_types::miniserde_helpers::begin_nullable(&mut self.huf),
                "jpy" => stripe_types::miniserde_helpers::begin_nullable(&mut self.jpy),
                "mxn" => stripe_types::miniserde_helpers::begin_nullable(&mut self.mxn),
                "myr" => stripe_types::miniserde_helpers::begin_nullable(&mut self.myr),
                "nok" => stripe_types::miniserde_helpers::begin_nullable(&mut self.nok),
                "nzd" => stripe_types::miniserde_helpers::begin_nullable(&mut self.nzd),
                "pln" => stripe_types::miniserde_helpers::begin_nullable(&mut self.pln),
                "ron" => stripe_types::miniserde_helpers::begin_nullable(&mut self.ron),
                "sek" => stripe_types::miniserde_helpers::begin_nullable(&mut self.sek),
                "sgd" => stripe_types::miniserde_helpers::begin_nullable(&mut self.sgd),
                "usd" => stripe_types::miniserde_helpers::begin_nullable(&mut self.usd),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                aed: None,
                aud: None,
                cad: None,
                chf: None,
                czk: None,
                dkk: None,
                eur: None,
                gbp: None,
                gip: None,
                hkd: None,
                huf: None,
                jpy: None,
                mxn: None,
                myr: None,
                nok: None,
                nzd: None,
                pln: None,
                ron: None,
                sek: None,
                sgd: None,
                usd: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                aed,
                aud,
                cad,
                chf,
                czk,
                dkk,
                eur,
                gbp,
                gip,
                hkd,
                huf,
                jpy,
                mxn,
                myr,
                nok,
                nzd,
                pln,
                ron,
                sek,
                sgd,
                usd,
            ) = (
                self.aed.take(),
                self.aud.take(),
                self.cad.take(),
                self.chf.take(),
                self.czk.take(),
                self.dkk.take(),
                self.eur.take(),
                self.gbp.take(),
                self.gip.take(),
                self.hkd.take(),
                self.huf.take(),
                self.jpy.take(),
                self.mxn.take(),
                self.myr.take(),
                self.nok.take(),
                self.nzd.take(),
                self.pln.take(),
                self.ron.take(),
                self.sek.take(),
                self.sgd.take(),
                self.usd.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                aed,
                aud,
                cad,
                chf,
                czk,
                dkk,
                eur,
                gbp,
                gip,
                hkd,
                huf,
                jpy,
                mxn,
                myr,
                nok,
                nzd,
                pln,
                ron,
                sek,
                sgd,
                usd,
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

    impl ObjectDeser for TerminalConfigurationConfigurationResourceTipping {
        type Builder = TerminalConfigurationConfigurationResourceTippingBuilder;
    }

    impl FromValueOpt for TerminalConfigurationConfigurationResourceTipping {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};

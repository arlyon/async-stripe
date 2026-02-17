#[derive(Clone, Debug, Eq, PartialEq)]
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
#[doc(hidden)]
pub struct TerminalConfigurationConfigurationResourceTippingBuilder {
    aed: Option<
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    >,
    aud: Option<
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    >,
    cad: Option<
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    >,
    chf: Option<
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    >,
    czk: Option<
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    >,
    dkk: Option<
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    >,
    eur: Option<
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    >,
    gbp: Option<
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    >,
    gip: Option<
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    >,
    hkd: Option<
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    >,
    huf: Option<
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    >,
    jpy: Option<
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    >,
    mxn: Option<
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    >,
    myr: Option<
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    >,
    nok: Option<
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    >,
    nzd: Option<
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    >,
    pln: Option<
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    >,
    ron: Option<
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    >,
    sek: Option<
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    >,
    sgd: Option<
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    >,
    usd: Option<
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    >,
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
                "aed" => Deserialize::begin(&mut self.aed),
                "aud" => Deserialize::begin(&mut self.aud),
                "cad" => Deserialize::begin(&mut self.cad),
                "chf" => Deserialize::begin(&mut self.chf),
                "czk" => Deserialize::begin(&mut self.czk),
                "dkk" => Deserialize::begin(&mut self.dkk),
                "eur" => Deserialize::begin(&mut self.eur),
                "gbp" => Deserialize::begin(&mut self.gbp),
                "gip" => Deserialize::begin(&mut self.gip),
                "hkd" => Deserialize::begin(&mut self.hkd),
                "huf" => Deserialize::begin(&mut self.huf),
                "jpy" => Deserialize::begin(&mut self.jpy),
                "mxn" => Deserialize::begin(&mut self.mxn),
                "myr" => Deserialize::begin(&mut self.myr),
                "nok" => Deserialize::begin(&mut self.nok),
                "nzd" => Deserialize::begin(&mut self.nzd),
                "pln" => Deserialize::begin(&mut self.pln),
                "ron" => Deserialize::begin(&mut self.ron),
                "sek" => Deserialize::begin(&mut self.sek),
                "sgd" => Deserialize::begin(&mut self.sgd),
                "usd" => Deserialize::begin(&mut self.usd),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                aed: Deserialize::default(),
                aud: Deserialize::default(),
                cad: Deserialize::default(),
                chf: Deserialize::default(),
                czk: Deserialize::default(),
                dkk: Deserialize::default(),
                eur: Deserialize::default(),
                gbp: Deserialize::default(),
                gip: Deserialize::default(),
                hkd: Deserialize::default(),
                huf: Deserialize::default(),
                jpy: Deserialize::default(),
                mxn: Deserialize::default(),
                myr: Deserialize::default(),
                nok: Deserialize::default(),
                nzd: Deserialize::default(),
                pln: Deserialize::default(),
                ron: Deserialize::default(),
                sek: Deserialize::default(),
                sgd: Deserialize::default(),
                usd: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(aed),
                Some(aud),
                Some(cad),
                Some(chf),
                Some(czk),
                Some(dkk),
                Some(eur),
                Some(gbp),
                Some(gip),
                Some(hkd),
                Some(huf),
                Some(jpy),
                Some(mxn),
                Some(myr),
                Some(nok),
                Some(nzd),
                Some(pln),
                Some(ron),
                Some(sek),
                Some(sgd),
                Some(usd),
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
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for TerminalConfigurationConfigurationResourceTipping {
        type Builder = TerminalConfigurationConfigurationResourceTippingBuilder;
    }

    impl FromValueOpt for TerminalConfigurationConfigurationResourceTipping {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TerminalConfigurationConfigurationResourceTippingBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "aed" => b.aed = FromValueOpt::from_value(v),
                    "aud" => b.aud = FromValueOpt::from_value(v),
                    "cad" => b.cad = FromValueOpt::from_value(v),
                    "chf" => b.chf = FromValueOpt::from_value(v),
                    "czk" => b.czk = FromValueOpt::from_value(v),
                    "dkk" => b.dkk = FromValueOpt::from_value(v),
                    "eur" => b.eur = FromValueOpt::from_value(v),
                    "gbp" => b.gbp = FromValueOpt::from_value(v),
                    "gip" => b.gip = FromValueOpt::from_value(v),
                    "hkd" => b.hkd = FromValueOpt::from_value(v),
                    "huf" => b.huf = FromValueOpt::from_value(v),
                    "jpy" => b.jpy = FromValueOpt::from_value(v),
                    "mxn" => b.mxn = FromValueOpt::from_value(v),
                    "myr" => b.myr = FromValueOpt::from_value(v),
                    "nok" => b.nok = FromValueOpt::from_value(v),
                    "nzd" => b.nzd = FromValueOpt::from_value(v),
                    "pln" => b.pln = FromValueOpt::from_value(v),
                    "ron" => b.ron = FromValueOpt::from_value(v),
                    "sek" => b.sek = FromValueOpt::from_value(v),
                    "sgd" => b.sgd = FromValueOpt::from_value(v),
                    "usd" => b.usd = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};

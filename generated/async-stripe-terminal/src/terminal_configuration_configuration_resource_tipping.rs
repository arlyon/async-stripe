#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalConfigurationConfigurationResourceTipping {
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
    pub hkd:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    pub myr:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    pub nok:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    pub nzd:
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
    hkd: Option<
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
    use miniserde::{make_place, Deserialize, Result};
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
                "aud" => Deserialize::begin(&mut self.aud),
                "cad" => Deserialize::begin(&mut self.cad),
                "chf" => Deserialize::begin(&mut self.chf),
                "czk" => Deserialize::begin(&mut self.czk),
                "dkk" => Deserialize::begin(&mut self.dkk),
                "eur" => Deserialize::begin(&mut self.eur),
                "gbp" => Deserialize::begin(&mut self.gbp),
                "hkd" => Deserialize::begin(&mut self.hkd),
                "myr" => Deserialize::begin(&mut self.myr),
                "nok" => Deserialize::begin(&mut self.nok),
                "nzd" => Deserialize::begin(&mut self.nzd),
                "sek" => Deserialize::begin(&mut self.sek),
                "sgd" => Deserialize::begin(&mut self.sgd),
                "usd" => Deserialize::begin(&mut self.usd),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                aud: Deserialize::default(),
                cad: Deserialize::default(),
                chf: Deserialize::default(),
                czk: Deserialize::default(),
                dkk: Deserialize::default(),
                eur: Deserialize::default(),
                gbp: Deserialize::default(),
                hkd: Deserialize::default(),
                myr: Deserialize::default(),
                nok: Deserialize::default(),
                nzd: Deserialize::default(),
                sek: Deserialize::default(),
                sgd: Deserialize::default(),
                usd: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(aud),
                Some(cad),
                Some(chf),
                Some(czk),
                Some(dkk),
                Some(eur),
                Some(gbp),
                Some(hkd),
                Some(myr),
                Some(nok),
                Some(nzd),
                Some(sek),
                Some(sgd),
                Some(usd),
            ) = (
                self.aud.take(),
                self.cad.take(),
                self.chf.take(),
                self.czk.take(),
                self.dkk.take(),
                self.eur.take(),
                self.gbp.take(),
                self.hkd.take(),
                self.myr.take(),
                self.nok.take(),
                self.nzd.take(),
                self.sek.take(),
                self.sgd.take(),
                self.usd.take(),
            )
            else {
                return None;
            };
            Some(Self::Out { aud, cad, chf, czk, dkk, eur, gbp, hkd, myr, nok, nzd, sek, sgd, usd })
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
                    "aud" => b.aud = FromValueOpt::from_value(v),
                    "cad" => b.cad = FromValueOpt::from_value(v),
                    "chf" => b.chf = FromValueOpt::from_value(v),
                    "czk" => b.czk = FromValueOpt::from_value(v),
                    "dkk" => b.dkk = FromValueOpt::from_value(v),
                    "eur" => b.eur = FromValueOpt::from_value(v),
                    "gbp" => b.gbp = FromValueOpt::from_value(v),
                    "hkd" => b.hkd = FromValueOpt::from_value(v),
                    "myr" => b.myr = FromValueOpt::from_value(v),
                    "nok" => b.nok = FromValueOpt::from_value(v),
                    "nzd" => b.nzd = FromValueOpt::from_value(v),
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

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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: TerminalConfigurationConfigurationResourceTippingBuilder {
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
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "aed" => Deserialize::begin(&mut self.builder.aed),
                "aud" => Deserialize::begin(&mut self.builder.aud),
                "cad" => Deserialize::begin(&mut self.builder.cad),
                "chf" => Deserialize::begin(&mut self.builder.chf),
                "czk" => Deserialize::begin(&mut self.builder.czk),
                "dkk" => Deserialize::begin(&mut self.builder.dkk),
                "eur" => Deserialize::begin(&mut self.builder.eur),
                "gbp" => Deserialize::begin(&mut self.builder.gbp),
                "gip" => Deserialize::begin(&mut self.builder.gip),
                "hkd" => Deserialize::begin(&mut self.builder.hkd),
                "huf" => Deserialize::begin(&mut self.builder.huf),
                "jpy" => Deserialize::begin(&mut self.builder.jpy),
                "mxn" => Deserialize::begin(&mut self.builder.mxn),
                "myr" => Deserialize::begin(&mut self.builder.myr),
                "nok" => Deserialize::begin(&mut self.builder.nok),
                "nzd" => Deserialize::begin(&mut self.builder.nzd),
                "pln" => Deserialize::begin(&mut self.builder.pln),
                "ron" => Deserialize::begin(&mut self.builder.ron),
                "sek" => Deserialize::begin(&mut self.builder.sek),
                "sgd" => Deserialize::begin(&mut self.builder.sgd),
                "usd" => Deserialize::begin(&mut self.builder.usd),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
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
                self.builder.aed.take(),
                self.builder.aud.take(),
                self.builder.cad.take(),
                self.builder.chf.take(),
                self.builder.czk.take(),
                self.builder.dkk.take(),
                self.builder.eur.take(),
                self.builder.gbp.take(),
                self.builder.gip.take(),
                self.builder.hkd.take(),
                self.builder.huf.take(),
                self.builder.jpy.take(),
                self.builder.mxn.take(),
                self.builder.myr.take(),
                self.builder.nok.take(),
                self.builder.nzd.take(),
                self.builder.pln.take(),
                self.builder.ron.take(),
                self.builder.sek.take(),
                self.builder.sgd.take(),
                self.builder.usd.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(TerminalConfigurationConfigurationResourceTipping {
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
            });
            Ok(())
        }
    }
};

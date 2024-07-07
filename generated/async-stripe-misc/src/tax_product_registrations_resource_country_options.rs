#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductRegistrationsResourceCountryOptions {
    pub ae: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsDefault>,
    pub at: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub au: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsDefault>,
    pub be: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub bg: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub ca: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsCanada>,
    pub ch: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsDefault>,
    pub cl: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsSimplified>,
    pub co: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsSimplified>,
    pub cy: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub cz: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub de: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub dk: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub ee: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub es: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub fi: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub fr: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub gb: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsDefault>,
    pub gr: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub hr: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub hu: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub id: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsSimplified>,
    pub ie: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub is: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsDefault>,
    pub it: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub jp: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsDefault>,
    pub kr: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsSimplified>,
    pub lt: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub lu: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub lv: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub mt: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub mx: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsSimplified>,
    pub my: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsSimplified>,
    pub nl: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub no: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsDefault>,
    pub nz: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsDefault>,
    pub pl: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub pt: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub ro: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub sa: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsSimplified>,
    pub se: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub sg: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsDefault>,
    pub si: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub sk: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub th: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsSimplified>,
    pub tr: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsSimplified>,
    pub us: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUnitedStates>,
    pub vn: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsSimplified>,
    pub za: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsDefault>,
}
#[doc(hidden)]
pub struct TaxProductRegistrationsResourceCountryOptionsBuilder {
    ae: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsDefault>>,
    at: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    au: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsDefault>>,
    be: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    bg: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    ca: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsCanada>>,
    ch: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsDefault>>,
    cl: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsSimplified>>,
    co: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsSimplified>>,
    cy: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    cz: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    de: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    dk: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    ee: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    es: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    fi: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    fr: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    gb: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsDefault>>,
    gr: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    hr: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    hu: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    id: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsSimplified>>,
    ie: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    is: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsDefault>>,
    it: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    jp: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsDefault>>,
    kr: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsSimplified>>,
    lt: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    lu: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    lv: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    mt: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    mx: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsSimplified>>,
    my: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsSimplified>>,
    nl: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    no: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsDefault>>,
    nz: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsDefault>>,
    pl: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    pt: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    ro: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    sa: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsSimplified>>,
    se: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    sg: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsDefault>>,
    si: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    sk: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEurope>>,
    th: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsSimplified>>,
    tr: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsSimplified>>,
    us: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUnitedStates>>,
    vn: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsSimplified>>,
    za: Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsDefault>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TaxProductRegistrationsResourceCountryOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductRegistrationsResourceCountryOptions>,
        builder: TaxProductRegistrationsResourceCountryOptionsBuilder,
    }

    impl Visitor for Place<TaxProductRegistrationsResourceCountryOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TaxProductRegistrationsResourceCountryOptionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TaxProductRegistrationsResourceCountryOptionsBuilder {
        type Out = TaxProductRegistrationsResourceCountryOptions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "ae" => Deserialize::begin(&mut self.ae),
                "at" => Deserialize::begin(&mut self.at),
                "au" => Deserialize::begin(&mut self.au),
                "be" => Deserialize::begin(&mut self.be),
                "bg" => Deserialize::begin(&mut self.bg),
                "ca" => Deserialize::begin(&mut self.ca),
                "ch" => Deserialize::begin(&mut self.ch),
                "cl" => Deserialize::begin(&mut self.cl),
                "co" => Deserialize::begin(&mut self.co),
                "cy" => Deserialize::begin(&mut self.cy),
                "cz" => Deserialize::begin(&mut self.cz),
                "de" => Deserialize::begin(&mut self.de),
                "dk" => Deserialize::begin(&mut self.dk),
                "ee" => Deserialize::begin(&mut self.ee),
                "es" => Deserialize::begin(&mut self.es),
                "fi" => Deserialize::begin(&mut self.fi),
                "fr" => Deserialize::begin(&mut self.fr),
                "gb" => Deserialize::begin(&mut self.gb),
                "gr" => Deserialize::begin(&mut self.gr),
                "hr" => Deserialize::begin(&mut self.hr),
                "hu" => Deserialize::begin(&mut self.hu),
                "id" => Deserialize::begin(&mut self.id),
                "ie" => Deserialize::begin(&mut self.ie),
                "is" => Deserialize::begin(&mut self.is),
                "it" => Deserialize::begin(&mut self.it),
                "jp" => Deserialize::begin(&mut self.jp),
                "kr" => Deserialize::begin(&mut self.kr),
                "lt" => Deserialize::begin(&mut self.lt),
                "lu" => Deserialize::begin(&mut self.lu),
                "lv" => Deserialize::begin(&mut self.lv),
                "mt" => Deserialize::begin(&mut self.mt),
                "mx" => Deserialize::begin(&mut self.mx),
                "my" => Deserialize::begin(&mut self.my),
                "nl" => Deserialize::begin(&mut self.nl),
                "no" => Deserialize::begin(&mut self.no),
                "nz" => Deserialize::begin(&mut self.nz),
                "pl" => Deserialize::begin(&mut self.pl),
                "pt" => Deserialize::begin(&mut self.pt),
                "ro" => Deserialize::begin(&mut self.ro),
                "sa" => Deserialize::begin(&mut self.sa),
                "se" => Deserialize::begin(&mut self.se),
                "sg" => Deserialize::begin(&mut self.sg),
                "si" => Deserialize::begin(&mut self.si),
                "sk" => Deserialize::begin(&mut self.sk),
                "th" => Deserialize::begin(&mut self.th),
                "tr" => Deserialize::begin(&mut self.tr),
                "us" => Deserialize::begin(&mut self.us),
                "vn" => Deserialize::begin(&mut self.vn),
                "za" => Deserialize::begin(&mut self.za),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                ae: Deserialize::default(),
                at: Deserialize::default(),
                au: Deserialize::default(),
                be: Deserialize::default(),
                bg: Deserialize::default(),
                ca: Deserialize::default(),
                ch: Deserialize::default(),
                cl: Deserialize::default(),
                co: Deserialize::default(),
                cy: Deserialize::default(),
                cz: Deserialize::default(),
                de: Deserialize::default(),
                dk: Deserialize::default(),
                ee: Deserialize::default(),
                es: Deserialize::default(),
                fi: Deserialize::default(),
                fr: Deserialize::default(),
                gb: Deserialize::default(),
                gr: Deserialize::default(),
                hr: Deserialize::default(),
                hu: Deserialize::default(),
                id: Deserialize::default(),
                ie: Deserialize::default(),
                is: Deserialize::default(),
                it: Deserialize::default(),
                jp: Deserialize::default(),
                kr: Deserialize::default(),
                lt: Deserialize::default(),
                lu: Deserialize::default(),
                lv: Deserialize::default(),
                mt: Deserialize::default(),
                mx: Deserialize::default(),
                my: Deserialize::default(),
                nl: Deserialize::default(),
                no: Deserialize::default(),
                nz: Deserialize::default(),
                pl: Deserialize::default(),
                pt: Deserialize::default(),
                ro: Deserialize::default(),
                sa: Deserialize::default(),
                se: Deserialize::default(),
                sg: Deserialize::default(),
                si: Deserialize::default(),
                sk: Deserialize::default(),
                th: Deserialize::default(),
                tr: Deserialize::default(),
                us: Deserialize::default(),
                vn: Deserialize::default(),
                za: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                ae: self.ae?,
                at: self.at?,
                au: self.au?,
                be: self.be?,
                bg: self.bg?,
                ca: self.ca.take()?,
                ch: self.ch?,
                cl: self.cl?,
                co: self.co?,
                cy: self.cy?,
                cz: self.cz?,
                de: self.de?,
                dk: self.dk?,
                ee: self.ee?,
                es: self.es?,
                fi: self.fi?,
                fr: self.fr?,
                gb: self.gb?,
                gr: self.gr?,
                hr: self.hr?,
                hu: self.hu?,
                id: self.id?,
                ie: self.ie?,
                is: self.is?,
                it: self.it?,
                jp: self.jp?,
                kr: self.kr?,
                lt: self.lt?,
                lu: self.lu?,
                lv: self.lv?,
                mt: self.mt?,
                mx: self.mx?,
                my: self.my?,
                nl: self.nl?,
                no: self.no?,
                nz: self.nz?,
                pl: self.pl?,
                pt: self.pt?,
                ro: self.ro?,
                sa: self.sa?,
                se: self.se?,
                sg: self.sg?,
                si: self.si?,
                sk: self.sk?,
                th: self.th?,
                tr: self.tr?,
                us: self.us.take()?,
                vn: self.vn?,
                za: self.za?,
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

    impl ObjectDeser for TaxProductRegistrationsResourceCountryOptions {
        type Builder = TaxProductRegistrationsResourceCountryOptionsBuilder;
    }

    impl FromValueOpt for TaxProductRegistrationsResourceCountryOptions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TaxProductRegistrationsResourceCountryOptionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "ae" => b.ae = Some(FromValueOpt::from_value(v)?),
                    "at" => b.at = Some(FromValueOpt::from_value(v)?),
                    "au" => b.au = Some(FromValueOpt::from_value(v)?),
                    "be" => b.be = Some(FromValueOpt::from_value(v)?),
                    "bg" => b.bg = Some(FromValueOpt::from_value(v)?),
                    "ca" => b.ca = Some(FromValueOpt::from_value(v)?),
                    "ch" => b.ch = Some(FromValueOpt::from_value(v)?),
                    "cl" => b.cl = Some(FromValueOpt::from_value(v)?),
                    "co" => b.co = Some(FromValueOpt::from_value(v)?),
                    "cy" => b.cy = Some(FromValueOpt::from_value(v)?),
                    "cz" => b.cz = Some(FromValueOpt::from_value(v)?),
                    "de" => b.de = Some(FromValueOpt::from_value(v)?),
                    "dk" => b.dk = Some(FromValueOpt::from_value(v)?),
                    "ee" => b.ee = Some(FromValueOpt::from_value(v)?),
                    "es" => b.es = Some(FromValueOpt::from_value(v)?),
                    "fi" => b.fi = Some(FromValueOpt::from_value(v)?),
                    "fr" => b.fr = Some(FromValueOpt::from_value(v)?),
                    "gb" => b.gb = Some(FromValueOpt::from_value(v)?),
                    "gr" => b.gr = Some(FromValueOpt::from_value(v)?),
                    "hr" => b.hr = Some(FromValueOpt::from_value(v)?),
                    "hu" => b.hu = Some(FromValueOpt::from_value(v)?),
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "ie" => b.ie = Some(FromValueOpt::from_value(v)?),
                    "is" => b.is = Some(FromValueOpt::from_value(v)?),
                    "it" => b.it = Some(FromValueOpt::from_value(v)?),
                    "jp" => b.jp = Some(FromValueOpt::from_value(v)?),
                    "kr" => b.kr = Some(FromValueOpt::from_value(v)?),
                    "lt" => b.lt = Some(FromValueOpt::from_value(v)?),
                    "lu" => b.lu = Some(FromValueOpt::from_value(v)?),
                    "lv" => b.lv = Some(FromValueOpt::from_value(v)?),
                    "mt" => b.mt = Some(FromValueOpt::from_value(v)?),
                    "mx" => b.mx = Some(FromValueOpt::from_value(v)?),
                    "my" => b.my = Some(FromValueOpt::from_value(v)?),
                    "nl" => b.nl = Some(FromValueOpt::from_value(v)?),
                    "no" => b.no = Some(FromValueOpt::from_value(v)?),
                    "nz" => b.nz = Some(FromValueOpt::from_value(v)?),
                    "pl" => b.pl = Some(FromValueOpt::from_value(v)?),
                    "pt" => b.pt = Some(FromValueOpt::from_value(v)?),
                    "ro" => b.ro = Some(FromValueOpt::from_value(v)?),
                    "sa" => b.sa = Some(FromValueOpt::from_value(v)?),
                    "se" => b.se = Some(FromValueOpt::from_value(v)?),
                    "sg" => b.sg = Some(FromValueOpt::from_value(v)?),
                    "si" => b.si = Some(FromValueOpt::from_value(v)?),
                    "sk" => b.sk = Some(FromValueOpt::from_value(v)?),
                    "th" => b.th = Some(FromValueOpt::from_value(v)?),
                    "tr" => b.tr = Some(FromValueOpt::from_value(v)?),
                    "us" => b.us = Some(FromValueOpt::from_value(v)?),
                    "vn" => b.vn = Some(FromValueOpt::from_value(v)?),
                    "za" => b.za = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};

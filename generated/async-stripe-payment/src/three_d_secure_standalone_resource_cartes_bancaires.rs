/// Contains details for Cartes Bancaires specific fields in the authentication outcomes.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ThreeDSecureStandaloneResourceCartesBancaires {
    /// The cryptogram calculation algorithm used by the card Issuer's ACS to calculate the Authentication cryptogram.
    /// Also known as cavvAlgorithm.
    /// ARes/RReq messageExtension: `CB-AVALGO`.
    pub avalgo: String,
    /// The exemption indicator returned from Cartes Bancaires in the ARes.
    /// This is a 3 byte bitmap (lowest significant byte first and most significant bit first) that has been Base64 encoded.
    /// String (4 characters).
    /// ARes message extension: `CB-EXEMPTION`.
    pub cb_exemption: Option<String>,
    /// The risk score returned from Cartes Bancaires in the ARes.
    /// Numeric value 0-99.
    /// ARes/RReq message extension: `CB-SCORE`.
    pub cb_score: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ThreeDSecureStandaloneResourceCartesBancaires {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ThreeDSecureStandaloneResourceCartesBancaires").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ThreeDSecureStandaloneResourceCartesBancairesBuilder {
    avalgo: Option<String>,
    cb_exemption: Option<Option<String>>,
    cb_score: Option<Option<String>>,
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

    impl Deserialize for ThreeDSecureStandaloneResourceCartesBancaires {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ThreeDSecureStandaloneResourceCartesBancaires>,
        builder: ThreeDSecureStandaloneResourceCartesBancairesBuilder,
    }

    impl Visitor for Place<ThreeDSecureStandaloneResourceCartesBancaires> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ThreeDSecureStandaloneResourceCartesBancairesBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ThreeDSecureStandaloneResourceCartesBancairesBuilder {
        type Out = ThreeDSecureStandaloneResourceCartesBancaires;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "avalgo" => Deserialize::begin(&mut self.avalgo),
                "cb_exemption" => Deserialize::begin(&mut self.cb_exemption),
                "cb_score" => Deserialize::begin(&mut self.cb_score),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { avalgo: None, cb_exemption: Some(None), cb_score: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(avalgo), Some(cb_exemption), Some(cb_score)) =
                (self.avalgo.take(), self.cb_exemption.take(), self.cb_score.take())
            else {
                return None;
            };
            Some(Self::Out { avalgo, cb_exemption, cb_score })
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

    impl ObjectDeser for ThreeDSecureStandaloneResourceCartesBancaires {
        type Builder = ThreeDSecureStandaloneResourceCartesBancairesBuilder;
    }

    impl FromValueOpt for ThreeDSecureStandaloneResourceCartesBancaires {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ThreeDSecureStandaloneResourceCartesBancairesBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "avalgo" => b.avalgo = FromValueOpt::from_value(v),
                    "cb_exemption" => b.cb_exemption = FromValueOpt::from_value(v),
                    "cb_score" => b.cb_score = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};

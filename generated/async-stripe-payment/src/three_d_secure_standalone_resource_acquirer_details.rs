/// Contains additional details about the acquirer for a 3DS Authentication.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ThreeDSecureStandaloneResourceAcquirerDetails {
    /// The Acquirer BIN (specific to the directory_server).
    pub acquirer_bin: Option<String>,
    /// The two-letter country code of the acquirer ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub acquirer_country: Option<String>,
    /// The Merchant ID (or Card Acceptor ID) that your acquirer assigned you (specific to the directory_server).
    pub acquirer_merchant_id: Option<String>,
    /// The [merchant category code](https://en.wikipedia.org/wiki/Merchant_category_code) as defined by each payment system or directory server.
    pub mcc: Option<String>,
    /// The merchant name assigned by the acquirer or payment system.
    /// Same name used in the authorization message as defined in [ISO 8583](https://en.wikipedia.org/wiki/ISO_8583).
    pub merchant_name: Option<String>,
    /// Requestor ID if you’re enrolled in the card network’s 3DS program.
    /// Otherwise, you can omit this field because Stripe assigns a Requestor ID with the card networks.
    pub requestor_id: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ThreeDSecureStandaloneResourceAcquirerDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ThreeDSecureStandaloneResourceAcquirerDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ThreeDSecureStandaloneResourceAcquirerDetailsBuilder {
    acquirer_bin: Option<Option<String>>,
    acquirer_country: Option<Option<String>>,
    acquirer_merchant_id: Option<Option<String>>,
    mcc: Option<Option<String>>,
    merchant_name: Option<Option<String>>,
    requestor_id: Option<Option<String>>,
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

    impl Deserialize for ThreeDSecureStandaloneResourceAcquirerDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ThreeDSecureStandaloneResourceAcquirerDetails>,
        builder: ThreeDSecureStandaloneResourceAcquirerDetailsBuilder,
    }

    impl Visitor for Place<ThreeDSecureStandaloneResourceAcquirerDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ThreeDSecureStandaloneResourceAcquirerDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ThreeDSecureStandaloneResourceAcquirerDetailsBuilder {
        type Out = ThreeDSecureStandaloneResourceAcquirerDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "acquirer_bin" => Deserialize::begin(&mut self.acquirer_bin),
                "acquirer_country" => Deserialize::begin(&mut self.acquirer_country),
                "acquirer_merchant_id" => Deserialize::begin(&mut self.acquirer_merchant_id),
                "mcc" => Deserialize::begin(&mut self.mcc),
                "merchant_name" => Deserialize::begin(&mut self.merchant_name),
                "requestor_id" => Deserialize::begin(&mut self.requestor_id),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                acquirer_bin: Some(None),
                acquirer_country: Some(None),
                acquirer_merchant_id: Some(None),
                mcc: Some(None),
                merchant_name: Some(None),
                requestor_id: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(acquirer_bin),
                Some(acquirer_country),
                Some(acquirer_merchant_id),
                Some(mcc),
                Some(merchant_name),
                Some(requestor_id),
            ) = (
                self.acquirer_bin.take(),
                self.acquirer_country.take(),
                self.acquirer_merchant_id.take(),
                self.mcc.take(),
                self.merchant_name.take(),
                self.requestor_id.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                acquirer_bin,
                acquirer_country,
                acquirer_merchant_id,
                mcc,
                merchant_name,
                requestor_id,
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

    impl ObjectDeser for ThreeDSecureStandaloneResourceAcquirerDetails {
        type Builder = ThreeDSecureStandaloneResourceAcquirerDetailsBuilder;
    }

    impl FromValueOpt for ThreeDSecureStandaloneResourceAcquirerDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ThreeDSecureStandaloneResourceAcquirerDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "acquirer_bin" => b.acquirer_bin = FromValueOpt::from_value(v),
                    "acquirer_country" => b.acquirer_country = FromValueOpt::from_value(v),
                    "acquirer_merchant_id" => b.acquirer_merchant_id = FromValueOpt::from_value(v),
                    "mcc" => b.mcc = FromValueOpt::from_value(v),
                    "merchant_name" => b.merchant_name = FromValueOpt::from_value(v),
                    "requestor_id" => b.requestor_id = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};

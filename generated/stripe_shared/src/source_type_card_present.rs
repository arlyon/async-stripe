#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceTypeCardPresent {
    pub application_cryptogram: Option<String>,
    pub application_preferred_name: Option<String>,
    pub authorization_code: Option<String>,
    pub authorization_response_code: Option<String>,
    pub brand: Option<String>,
    pub country: Option<String>,
    pub cvm_type: Option<String>,
    pub data_type: Option<String>,
    pub dedicated_file_name: Option<String>,
    pub description: Option<String>,
    pub emv_auth_data: Option<String>,
    pub evidence_customer_signature: Option<String>,
    pub evidence_transaction_certificate: Option<String>,
    pub exp_month: Option<i64>,
    pub exp_year: Option<i64>,
    pub fingerprint: Option<String>,
    pub funding: Option<String>,
    pub iin: Option<String>,
    pub issuer: Option<String>,
    pub last4: Option<String>,
    pub pos_device_id: Option<String>,
    pub pos_entry_mode: Option<String>,
    pub read_method: Option<String>,
    pub reader: Option<String>,
    pub terminal_verification_results: Option<String>,
    pub transaction_status_information: Option<String>,
}
#[doc(hidden)]
pub struct SourceTypeCardPresentBuilder {
    application_cryptogram: Option<Option<String>>,
    application_preferred_name: Option<Option<String>>,
    authorization_code: Option<Option<String>>,
    authorization_response_code: Option<Option<String>>,
    brand: Option<Option<String>>,
    country: Option<Option<String>>,
    cvm_type: Option<Option<String>>,
    data_type: Option<Option<String>>,
    dedicated_file_name: Option<Option<String>>,
    description: Option<Option<String>>,
    emv_auth_data: Option<Option<String>>,
    evidence_customer_signature: Option<Option<String>>,
    evidence_transaction_certificate: Option<Option<String>>,
    exp_month: Option<Option<i64>>,
    exp_year: Option<Option<i64>>,
    fingerprint: Option<Option<String>>,
    funding: Option<Option<String>>,
    iin: Option<Option<String>>,
    issuer: Option<Option<String>>,
    last4: Option<Option<String>>,
    pos_device_id: Option<Option<String>>,
    pos_entry_mode: Option<Option<String>>,
    read_method: Option<Option<String>>,
    reader: Option<Option<String>>,
    terminal_verification_results: Option<Option<String>>,
    transaction_status_information: Option<Option<String>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SourceTypeCardPresent {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTypeCardPresent>,
        builder: SourceTypeCardPresentBuilder,
    }

    impl Visitor for Place<SourceTypeCardPresent> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SourceTypeCardPresentBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SourceTypeCardPresentBuilder {
        type Out = SourceTypeCardPresent;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "application_cryptogram" => Deserialize::begin(&mut self.application_cryptogram),
                "application_preferred_name" => {
                    Deserialize::begin(&mut self.application_preferred_name)
                }
                "authorization_code" => Deserialize::begin(&mut self.authorization_code),
                "authorization_response_code" => {
                    Deserialize::begin(&mut self.authorization_response_code)
                }
                "brand" => Deserialize::begin(&mut self.brand),
                "country" => Deserialize::begin(&mut self.country),
                "cvm_type" => Deserialize::begin(&mut self.cvm_type),
                "data_type" => Deserialize::begin(&mut self.data_type),
                "dedicated_file_name" => Deserialize::begin(&mut self.dedicated_file_name),
                "description" => Deserialize::begin(&mut self.description),
                "emv_auth_data" => Deserialize::begin(&mut self.emv_auth_data),
                "evidence_customer_signature" => {
                    Deserialize::begin(&mut self.evidence_customer_signature)
                }
                "evidence_transaction_certificate" => {
                    Deserialize::begin(&mut self.evidence_transaction_certificate)
                }
                "exp_month" => Deserialize::begin(&mut self.exp_month),
                "exp_year" => Deserialize::begin(&mut self.exp_year),
                "fingerprint" => Deserialize::begin(&mut self.fingerprint),
                "funding" => Deserialize::begin(&mut self.funding),
                "iin" => Deserialize::begin(&mut self.iin),
                "issuer" => Deserialize::begin(&mut self.issuer),
                "last4" => Deserialize::begin(&mut self.last4),
                "pos_device_id" => Deserialize::begin(&mut self.pos_device_id),
                "pos_entry_mode" => Deserialize::begin(&mut self.pos_entry_mode),
                "read_method" => Deserialize::begin(&mut self.read_method),
                "reader" => Deserialize::begin(&mut self.reader),
                "terminal_verification_results" => {
                    Deserialize::begin(&mut self.terminal_verification_results)
                }
                "transaction_status_information" => {
                    Deserialize::begin(&mut self.transaction_status_information)
                }

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                application_cryptogram: Deserialize::default(),
                application_preferred_name: Deserialize::default(),
                authorization_code: Deserialize::default(),
                authorization_response_code: Deserialize::default(),
                brand: Deserialize::default(),
                country: Deserialize::default(),
                cvm_type: Deserialize::default(),
                data_type: Deserialize::default(),
                dedicated_file_name: Deserialize::default(),
                description: Deserialize::default(),
                emv_auth_data: Deserialize::default(),
                evidence_customer_signature: Deserialize::default(),
                evidence_transaction_certificate: Deserialize::default(),
                exp_month: Deserialize::default(),
                exp_year: Deserialize::default(),
                fingerprint: Deserialize::default(),
                funding: Deserialize::default(),
                iin: Deserialize::default(),
                issuer: Deserialize::default(),
                last4: Deserialize::default(),
                pos_device_id: Deserialize::default(),
                pos_entry_mode: Deserialize::default(),
                read_method: Deserialize::default(),
                reader: Deserialize::default(),
                terminal_verification_results: Deserialize::default(),
                transaction_status_information: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                application_cryptogram: self.application_cryptogram.take()?,
                application_preferred_name: self.application_preferred_name.take()?,
                authorization_code: self.authorization_code.take()?,
                authorization_response_code: self.authorization_response_code.take()?,
                brand: self.brand.take()?,
                country: self.country.take()?,
                cvm_type: self.cvm_type.take()?,
                data_type: self.data_type.take()?,
                dedicated_file_name: self.dedicated_file_name.take()?,
                description: self.description.take()?,
                emv_auth_data: self.emv_auth_data.take()?,
                evidence_customer_signature: self.evidence_customer_signature.take()?,
                evidence_transaction_certificate: self.evidence_transaction_certificate.take()?,
                exp_month: self.exp_month?,
                exp_year: self.exp_year?,
                fingerprint: self.fingerprint.take()?,
                funding: self.funding.take()?,
                iin: self.iin.take()?,
                issuer: self.issuer.take()?,
                last4: self.last4.take()?,
                pos_device_id: self.pos_device_id.take()?,
                pos_entry_mode: self.pos_entry_mode.take()?,
                read_method: self.read_method.take()?,
                reader: self.reader.take()?,
                terminal_verification_results: self.terminal_verification_results.take()?,
                transaction_status_information: self.transaction_status_information.take()?,
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

    impl ObjectDeser for SourceTypeCardPresent {
        type Builder = SourceTypeCardPresentBuilder;
    }

    impl FromValueOpt for SourceTypeCardPresent {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SourceTypeCardPresentBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "application_cryptogram" => {
                        b.application_cryptogram = Some(FromValueOpt::from_value(v)?)
                    }
                    "application_preferred_name" => {
                        b.application_preferred_name = Some(FromValueOpt::from_value(v)?)
                    }
                    "authorization_code" => {
                        b.authorization_code = Some(FromValueOpt::from_value(v)?)
                    }
                    "authorization_response_code" => {
                        b.authorization_response_code = Some(FromValueOpt::from_value(v)?)
                    }
                    "brand" => b.brand = Some(FromValueOpt::from_value(v)?),
                    "country" => b.country = Some(FromValueOpt::from_value(v)?),
                    "cvm_type" => b.cvm_type = Some(FromValueOpt::from_value(v)?),
                    "data_type" => b.data_type = Some(FromValueOpt::from_value(v)?),
                    "dedicated_file_name" => {
                        b.dedicated_file_name = Some(FromValueOpt::from_value(v)?)
                    }
                    "description" => b.description = Some(FromValueOpt::from_value(v)?),
                    "emv_auth_data" => b.emv_auth_data = Some(FromValueOpt::from_value(v)?),
                    "evidence_customer_signature" => {
                        b.evidence_customer_signature = Some(FromValueOpt::from_value(v)?)
                    }
                    "evidence_transaction_certificate" => {
                        b.evidence_transaction_certificate = Some(FromValueOpt::from_value(v)?)
                    }
                    "exp_month" => b.exp_month = Some(FromValueOpt::from_value(v)?),
                    "exp_year" => b.exp_year = Some(FromValueOpt::from_value(v)?),
                    "fingerprint" => b.fingerprint = Some(FromValueOpt::from_value(v)?),
                    "funding" => b.funding = Some(FromValueOpt::from_value(v)?),
                    "iin" => b.iin = Some(FromValueOpt::from_value(v)?),
                    "issuer" => b.issuer = Some(FromValueOpt::from_value(v)?),
                    "last4" => b.last4 = Some(FromValueOpt::from_value(v)?),
                    "pos_device_id" => b.pos_device_id = Some(FromValueOpt::from_value(v)?),
                    "pos_entry_mode" => b.pos_entry_mode = Some(FromValueOpt::from_value(v)?),
                    "read_method" => b.read_method = Some(FromValueOpt::from_value(v)?),
                    "reader" => b.reader = Some(FromValueOpt::from_value(v)?),
                    "terminal_verification_results" => {
                        b.terminal_verification_results = Some(FromValueOpt::from_value(v)?)
                    }
                    "transaction_status_information" => {
                        b.transaction_status_information = Some(FromValueOpt::from_value(v)?)
                    }

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};

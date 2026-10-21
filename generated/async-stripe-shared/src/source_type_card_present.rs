#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SourceTypeCardPresent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SourceTypeCardPresent").finish_non_exhaustive()
    }
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
                application_cryptogram: Some(None),
                application_preferred_name: Some(None),
                authorization_code: Some(None),
                authorization_response_code: Some(None),
                brand: Some(None),
                country: Some(None),
                cvm_type: Some(None),
                data_type: Some(None),
                dedicated_file_name: Some(None),
                description: Some(None),
                emv_auth_data: Some(None),
                evidence_customer_signature: Some(None),
                evidence_transaction_certificate: Some(None),
                exp_month: Some(None),
                exp_year: Some(None),
                fingerprint: Some(None),
                funding: Some(None),
                iin: Some(None),
                issuer: Some(None),
                last4: Some(None),
                pos_device_id: Some(None),
                pos_entry_mode: Some(None),
                read_method: Some(None),
                reader: Some(None),
                terminal_verification_results: Some(None),
                transaction_status_information: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    application_cryptogram: Some(application_cryptogram),
                    application_preferred_name: Some(application_preferred_name),
                    authorization_code: Some(authorization_code),
                    authorization_response_code: Some(authorization_response_code),
                    brand: Some(brand),
                    country: Some(country),
                    cvm_type: Some(cvm_type),
                    data_type: Some(data_type),
                    dedicated_file_name: Some(dedicated_file_name),
                    description: Some(description),
                    emv_auth_data: Some(emv_auth_data),
                    evidence_customer_signature: Some(evidence_customer_signature),
                    evidence_transaction_certificate: Some(evidence_transaction_certificate),
                    exp_month: Some(exp_month),
                    exp_year: Some(exp_year),
                    fingerprint: Some(fingerprint),
                    funding: Some(funding),
                    iin: Some(iin),
                    issuer: Some(issuer),
                    last4: Some(last4),
                    pos_device_id: Some(pos_device_id),
                    pos_entry_mode: Some(pos_entry_mode),
                    read_method: Some(read_method),
                    reader: Some(reader),
                    terminal_verification_results: Some(terminal_verification_results),
                    transaction_status_information: Some(transaction_status_information),
                } => Some(Self::Out {
                    application_cryptogram,
                    application_preferred_name,
                    authorization_code,
                    authorization_response_code,
                    brand,
                    country,
                    cvm_type,
                    data_type,
                    dedicated_file_name,
                    description,
                    emv_auth_data,
                    evidence_customer_signature,
                    evidence_transaction_certificate,
                    exp_month,
                    exp_year,
                    fingerprint,
                    funding,
                    iin,
                    issuer,
                    last4,
                    pos_device_id,
                    pos_entry_mode,
                    read_method,
                    reader,
                    terminal_verification_results,
                    transaction_status_information,
                }),
                builder => {
                    *self = builder;
                    None
                }
            }
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

    impl ObjectDeser for SourceTypeCardPresent {
        type Builder = SourceTypeCardPresentBuilder;
    }

    impl FromValueOpt for SourceTypeCardPresent {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};

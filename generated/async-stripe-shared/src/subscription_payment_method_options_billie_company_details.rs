#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionPaymentMethodOptionsBillieCompanyDetails {
    pub registered_address: Option<
        stripe_shared::SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegisteredAddress,
    >,
    /// Company or entity name.
    pub registered_name: Option<String>,
    /// The official registration number for the given registration type.
    pub registration_number: Option<String>,
    /// Type of registration the company or entity holds in their registered country.
    pub registration_type:
        Option<SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType>,
    /// VAT ID number.
    pub vat: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionPaymentMethodOptionsBillieCompanyDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionPaymentMethodOptionsBillieCompanyDetails")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionPaymentMethodOptionsBillieCompanyDetailsBuilder {
    registered_address: Option<
        Option<
            stripe_shared::SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegisteredAddress,
        >,
    >,
    registered_name: Option<Option<String>>,
    registration_number: Option<Option<String>>,
    registration_type:
        Option<Option<SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType>>,
    vat: Option<Option<String>>,
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

    impl Deserialize for SubscriptionPaymentMethodOptionsBillieCompanyDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionPaymentMethodOptionsBillieCompanyDetails>,
        builder: SubscriptionPaymentMethodOptionsBillieCompanyDetailsBuilder,
    }

    impl Visitor for Place<SubscriptionPaymentMethodOptionsBillieCompanyDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionPaymentMethodOptionsBillieCompanyDetailsBuilder::deser_default(
                ),
            }))
        }
    }

    impl MapBuilder for SubscriptionPaymentMethodOptionsBillieCompanyDetailsBuilder {
        type Out = SubscriptionPaymentMethodOptionsBillieCompanyDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "registered_address" => Deserialize::begin(&mut self.registered_address),
                "registered_name" => Deserialize::begin(&mut self.registered_name),
                "registration_number" => Deserialize::begin(&mut self.registration_number),
                "registration_type" => Deserialize::begin(&mut self.registration_type),
                "vat" => Deserialize::begin(&mut self.vat),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                registered_address: Some(None),
                registered_name: Some(None),
                registration_number: Some(None),
                registration_type: Some(None),
                vat: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(registered_address),
                Some(registered_name),
                Some(registration_number),
                Some(registration_type),
                Some(vat),
            ) = (
                self.registered_address.take(),
                self.registered_name.take(),
                self.registration_number.take(),
                self.registration_type.take(),
                self.vat.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                registered_address,
                registered_name,
                registration_number,
                registration_type,
                vat,
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

    impl ObjectDeser for SubscriptionPaymentMethodOptionsBillieCompanyDetails {
        type Builder = SubscriptionPaymentMethodOptionsBillieCompanyDetailsBuilder;
    }

    impl FromValueOpt for SubscriptionPaymentMethodOptionsBillieCompanyDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                SubscriptionPaymentMethodOptionsBillieCompanyDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "registered_address" => b.registered_address = FromValueOpt::from_value(v),
                    "registered_name" => b.registered_name = FromValueOpt::from_value(v),
                    "registration_number" => b.registration_number = FromValueOpt::from_value(v),
                    "registration_type" => b.registration_type = FromValueOpt::from_value(v),
                    "vat" => b.vat = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Type of registration the company or entity holds in their registered country.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType {
    ChEin,
    DeHrb,
    DkCvr,
    EsCif,
    FiTunnus,
    FrSiren,
    FrSiret,
    ItRea,
    NlKvk,
    NoOrgNumber,
    NoPno,
    SeOrgNumber,
    SePno,
    UkCrn,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType {
    pub fn as_str(&self) -> &str {
        use SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType::*;
        match self {
            ChEin => "ch_ein",
            DeHrb => "de_hrb",
            DkCvr => "dk_cvr",
            EsCif => "es_cif",
            FiTunnus => "fi_tunnus",
            FrSiren => "fr_siren",
            FrSiret => "fr_siret",
            ItRea => "it_rea",
            NlKvk => "nl_kvk",
            NoOrgNumber => "no_org_number",
            NoPno => "no_pno",
            SeOrgNumber => "se_org_number",
            SePno => "se_pno",
            UkCrn => "uk_crn",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType::*;
        match s {
            "ch_ein" => Ok(ChEin),
            "de_hrb" => Ok(DeHrb),
            "dk_cvr" => Ok(DkCvr),
            "es_cif" => Ok(EsCif),
            "fi_tunnus" => Ok(FiTunnus),
            "fr_siren" => Ok(FrSiren),
            "fr_siret" => Ok(FrSiret),
            "it_rea" => Ok(ItRea),
            "nl_kvk" => Ok(NlKvk),
            "no_org_number" => Ok(NoOrgNumber),
            "no_pno" => Ok(NoPno),
            "se_org_number" => Ok(SeOrgNumber),
            "se_pno" => Ok(SePno),
            "uk_crn" => Ok(UkCrn),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(
            SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType
        ))
        .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize
    for SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}

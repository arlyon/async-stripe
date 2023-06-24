#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Bancontact {
    /// Bank code of bank associated with the bank account.
    pub bank_code: Option<String>,
    /// Name of the bank associated with the bank account.
    pub bank_name: Option<String>,
    /// Bank Identifier Code of the bank associated with the bank account.
    pub bic: Option<String>,
    /// The ID of the SEPA Direct Debit PaymentMethod which was generated by this SetupAttempt.
    pub generated_sepa_debit:
        Option<stripe_types::Expandable<stripe_core::payment_method::PaymentMethod>>,
    /// The mandate for the SEPA Direct Debit PaymentMethod which was generated by this SetupAttempt.
    pub generated_sepa_debit_mandate:
        Option<stripe_types::Expandable<stripe_core::mandate::Mandate>>,
    /// Last four characters of the IBAN.
    pub iban_last4: Option<String>,
    /// Preferred language of the Bancontact authorization page that the customer is redirected to.
    /// Can be one of `en`, `de`, `fr`, or `nl`.
    pub preferred_language: Option<BancontactPreferredLanguage>,
    /// Owner's verified full name.
    ///
    /// Values are verified or provided by Bancontact directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub verified_name: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Bancontact {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Preferred language of the Bancontact authorization page that the customer is redirected to.
/// Can be one of `en`, `de`, `fr`, or `nl`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BancontactPreferredLanguage {
    De,
    En,
    Fr,
    Nl,
}

impl BancontactPreferredLanguage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::De => "de",
            Self::En => "en",
            Self::Fr => "fr",
            Self::Nl => "nl",
        }
    }
}

impl std::str::FromStr for BancontactPreferredLanguage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "de" => Ok(Self::De),
            "en" => Ok(Self::En),
            "fr" => Ok(Self::Fr),
            "nl" => Ok(Self::Nl),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for BancontactPreferredLanguage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BancontactPreferredLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for BancontactPreferredLanguage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BancontactPreferredLanguage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for BancontactPreferredLanguage"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for BancontactPreferredLanguage {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<BancontactPreferredLanguage> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(BancontactPreferredLanguage::from_str(s)?);
        Ok(())
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodDetailsSofort {
    /// Bank code of bank associated with the bank account.
    pub bank_code: Option<String>,
    /// Name of the bank associated with the bank account.
    pub bank_name: Option<String>,
    /// Bank Identifier Code of the bank associated with the bank account.
    pub bic: Option<String>,
    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: Option<String>,
    /// The ID of the SEPA Direct Debit PaymentMethod which was generated by this Charge.
    pub generated_sepa_debit: Option<stripe_types::Expandable<stripe_types::PaymentMethod>>,
    /// The mandate for the SEPA Direct Debit PaymentMethod which was generated by this Charge.
    pub generated_sepa_debit_mandate: Option<stripe_types::Expandable<stripe_types::Mandate>>,
    /// Last four characters of the IBAN.
    pub iban_last4: Option<String>,
    /// Preferred language of the SOFORT authorization page that the customer is redirected to.
    /// Can be one of `de`, `en`, `es`, `fr`, `it`, `nl`, or `pl`.
    pub preferred_language: Option<PaymentMethodDetailsSofortPreferredLanguage>,
    /// Owner's verified full name.
    ///
    /// Values are verified or provided by SOFORT directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub verified_name: Option<String>,
}
/// Preferred language of the SOFORT authorization page that the customer is redirected to.
/// Can be one of `de`, `en`, `es`, `fr`, `it`, `nl`, or `pl`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodDetailsSofortPreferredLanguage {
    De,
    En,
    Es,
    Fr,
    It,
    Nl,
    Pl,
}

impl PaymentMethodDetailsSofortPreferredLanguage {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodDetailsSofortPreferredLanguage::*;
        match self {
            De => "de",
            En => "en",
            Es => "es",
            Fr => "fr",
            It => "it",
            Nl => "nl",
            Pl => "pl",
        }
    }
}

impl std::str::FromStr for PaymentMethodDetailsSofortPreferredLanguage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodDetailsSofortPreferredLanguage::*;
        match s {
            "de" => Ok(De),
            "en" => Ok(En),
            "es" => Ok(Es),
            "fr" => Ok(Fr),
            "it" => Ok(It),
            "nl" => Ok(Nl),
            "pl" => Ok(Pl),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentMethodDetailsSofortPreferredLanguage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodDetailsSofortPreferredLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodDetailsSofortPreferredLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentMethodDetailsSofortPreferredLanguage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentMethodDetailsSofortPreferredLanguage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentMethodDetailsSofortPreferredLanguage",
            )
        })
    }
}

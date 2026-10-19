#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ItemType {
    Account,
    CardBin,
    CardFingerprint,
    CaseSensitiveString,
    Country,
    CryptoFingerprint,
    CustomerId,
    Email,
    IpAddress,
    SepaDebitFingerprint,
    String,
    UsBankAccountFingerprint,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ItemType {
    pub fn as_str(&self) -> &str {
        use ItemType::*;
        match self {
            Account => "account",
            CardBin => "card_bin",
            CardFingerprint => "card_fingerprint",
            CaseSensitiveString => "case_sensitive_string",
            Country => "country",
            CryptoFingerprint => "crypto_fingerprint",
            CustomerId => "customer_id",
            Email => "email",
            IpAddress => "ip_address",
            SepaDebitFingerprint => "sepa_debit_fingerprint",
            String => "string",
            UsBankAccountFingerprint => "us_bank_account_fingerprint",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ItemType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ItemType::*;
        match s {
            "account" => Ok(Account),
            "card_bin" => Ok(CardBin),
            "card_fingerprint" => Ok(CardFingerprint),
            "case_sensitive_string" => Ok(CaseSensitiveString),
            "country" => Ok(Country),
            "crypto_fingerprint" => Ok(CryptoFingerprint),
            "customer_id" => Ok(CustomerId),
            "email" => Ok(Email),
            "ip_address" => Ok(IpAddress),
            "sepa_debit_fingerprint" => Ok(SepaDebitFingerprint),
            "string" => Ok(String),
            "us_bank_account_fingerprint" => Ok(UsBankAccountFingerprint),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "ItemType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ItemType)).finish_non_exhaustive()
    }
}
impl serde::Serialize for ItemType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ItemType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ItemType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ItemType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(ItemType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}

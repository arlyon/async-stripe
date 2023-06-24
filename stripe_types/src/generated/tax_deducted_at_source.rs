#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TaxDeductedAtSource {
    /// Unique identifier for the object.
    pub id: stripe_types::tax_deducted_at_source::TaxDeductedAtSourceId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: TaxDeductedAtSourceObject,
    /// The end of the invoicing period.
    ///
    /// This TDS applies to Stripe fees collected during this invoicing period.
    pub period_end: stripe_types::Timestamp,
    /// The start of the invoicing period.
    ///
    /// This TDS applies to Stripe fees collected during this invoicing period.
    pub period_start: stripe_types::Timestamp,
    /// The TAN that was supplied to Stripe when TDS was assessed.
    pub tax_deduction_account_number: String,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TaxDeductedAtSource {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TaxDeductedAtSourceObject {
    TaxDeductedAtSource,
}

impl TaxDeductedAtSourceObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TaxDeductedAtSource => "tax_deducted_at_source",
        }
    }
}

impl std::str::FromStr for TaxDeductedAtSourceObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tax_deducted_at_source" => Ok(Self::TaxDeductedAtSource),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for TaxDeductedAtSourceObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxDeductedAtSourceObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for TaxDeductedAtSourceObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TaxDeductedAtSourceObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TaxDeductedAtSourceObject"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TaxDeductedAtSourceObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<TaxDeductedAtSourceObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TaxDeductedAtSourceObject::from_str(s)?);
        Ok(())
    }
}
impl stripe_types::Object for TaxDeductedAtSource {
    type Id = stripe_types::tax_deducted_at_source::TaxDeductedAtSourceId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(TaxDeductedAtSourceId, "itds_");

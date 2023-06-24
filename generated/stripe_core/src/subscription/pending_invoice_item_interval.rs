#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PendingInvoiceItemInterval {
    /// Specifies invoicing frequency.
    ///
    /// Either `day`, `week`, `month` or `year`.
    pub interval: PendingInvoiceItemIntervalInterval,
    /// The number of intervals between invoices.
    ///
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    pub interval_count: u64,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PendingInvoiceItemInterval {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Specifies invoicing frequency.
///
/// Either `day`, `week`, `month` or `year`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PendingInvoiceItemIntervalInterval {
    Day,
    Month,
    Week,
    Year,
}

impl PendingInvoiceItemIntervalInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Month => "month",
            Self::Week => "week",
            Self::Year => "year",
        }
    }
}

impl std::str::FromStr for PendingInvoiceItemIntervalInterval {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "day" => Ok(Self::Day),
            "month" => Ok(Self::Month),
            "week" => Ok(Self::Week),
            "year" => Ok(Self::Year),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for PendingInvoiceItemIntervalInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PendingInvoiceItemIntervalInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for PendingInvoiceItemIntervalInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PendingInvoiceItemIntervalInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for PendingInvoiceItemIntervalInterval")
        })
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PendingInvoiceItemIntervalInterval {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<PendingInvoiceItemIntervalInterval> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PendingInvoiceItemIntervalInterval::from_str(s)?);
        Ok(())
    }
}

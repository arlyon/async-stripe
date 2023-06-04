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
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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

#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct FinancialConnections {
    /// The list of permissions to request.
    ///
    /// The `payment_method` permission must be included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<FinancialConnectionsPermissions>>,
    /// For webview integrations only.
    ///
    /// Upon completing OAuth login in the native browser, the user will be redirected to this URL to return to your app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for FinancialConnections {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The list of permissions to request.
///
/// The `payment_method` permission must be included.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum FinancialConnectionsPermissions {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}

impl FinancialConnectionsPermissions {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Balances => "balances",
            Self::Ownership => "ownership",
            Self::PaymentMethod => "payment_method",
            Self::Transactions => "transactions",
        }
    }
}

impl AsRef<str> for FinancialConnectionsPermissions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FinancialConnectionsPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "linked_account_options_common".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LinkedAccountOptionsCommon {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFilters>,

    /// The list of permissions to request.
    ///
    /// The `payment_method` permission must be included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<LinkedAccountOptionsCommonPermissions>>,

    /// Data features requested to be retrieved upon account creation.
    pub prefetch: Option<Vec<LinkedAccountOptionsCommonPrefetch>>,

    /// For webview integrations only.
    ///
    /// Upon completing OAuth login in the native browser, the user will be redirected to this URL to return to your app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFilters {

    /// The account subcategories to use to filter for possible accounts to link.
    ///
    /// Valid subcategories are `checking` and `savings`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_subcategories: Option<Vec<PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFiltersAccountSubcategories>>,
}

/// An enum representing the possible values of an `LinkedAccountOptionsCommon`'s `permissions` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum LinkedAccountOptionsCommonPermissions {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}

impl LinkedAccountOptionsCommonPermissions {
    pub fn as_str(self) -> &'static str {
        match self {
            LinkedAccountOptionsCommonPermissions::Balances => "balances",
            LinkedAccountOptionsCommonPermissions::Ownership => "ownership",
            LinkedAccountOptionsCommonPermissions::PaymentMethod => "payment_method",
            LinkedAccountOptionsCommonPermissions::Transactions => "transactions",
        }
    }
}

impl AsRef<str> for LinkedAccountOptionsCommonPermissions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for LinkedAccountOptionsCommonPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for LinkedAccountOptionsCommonPermissions {
    fn default() -> Self {
        Self::Balances
    }
}

/// An enum representing the possible values of an `LinkedAccountOptionsCommon`'s `prefetch` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum LinkedAccountOptionsCommonPrefetch {
    Balances,
    Ownership,
    Transactions,
}

impl LinkedAccountOptionsCommonPrefetch {
    pub fn as_str(self) -> &'static str {
        match self {
            LinkedAccountOptionsCommonPrefetch::Balances => "balances",
            LinkedAccountOptionsCommonPrefetch::Ownership => "ownership",
            LinkedAccountOptionsCommonPrefetch::Transactions => "transactions",
        }
    }
}

impl AsRef<str> for LinkedAccountOptionsCommonPrefetch {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for LinkedAccountOptionsCommonPrefetch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for LinkedAccountOptionsCommonPrefetch {
    fn default() -> Self {
        Self::Balances
    }
}

/// An enum representing the possible values of an `PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFilters`'s `account_subcategories` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFiltersAccountSubcategories {
    Checking,
    Savings,
}

impl PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFiltersAccountSubcategories {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFiltersAccountSubcategories::Checking => "checking",
            PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFiltersAccountSubcategories::Savings => "savings",
        }
    }
}

impl AsRef<str> for PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFiltersAccountSubcategories {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFiltersAccountSubcategories {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFiltersAccountSubcategories {
    fn default() -> Self {
        Self::Checking
    }
}

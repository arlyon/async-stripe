// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "linked_account_options_us_bank_account".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LinkedAccountOptionsUsBankAccount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<PaymentFlowsPrivatePaymentMethodsUsBankAccountLinkedAccountOptionsFilters>,

    /// The list of permissions to request.
    ///
    /// The `payment_method` permission must be included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<LinkedAccountOptionsUsBankAccountPermissions>>,

    /// Data features requested to be retrieved upon account creation.
    pub prefetch: Option<Vec<LinkedAccountOptionsUsBankAccountPrefetch>>,

    /// For webview integrations only.
    ///
    /// Upon completing OAuth login in the native browser, the user will be redirected to this URL to return to your app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentFlowsPrivatePaymentMethodsUsBankAccountLinkedAccountOptionsFilters {

    /// The account subcategories to use to filter for possible accounts to link.
    ///
    /// Valid subcategories are `checking` and `savings`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_subcategories: Option<Vec<PaymentFlowsPrivatePaymentMethodsUsBankAccountLinkedAccountOptionsFiltersAccountSubcategories>>,
}

/// An enum representing the possible values of an `LinkedAccountOptionsUsBankAccount`'s `permissions` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum LinkedAccountOptionsUsBankAccountPermissions {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}

impl LinkedAccountOptionsUsBankAccountPermissions {
    pub fn as_str(self) -> &'static str {
        match self {
            LinkedAccountOptionsUsBankAccountPermissions::Balances => "balances",
            LinkedAccountOptionsUsBankAccountPermissions::Ownership => "ownership",
            LinkedAccountOptionsUsBankAccountPermissions::PaymentMethod => "payment_method",
            LinkedAccountOptionsUsBankAccountPermissions::Transactions => "transactions",
        }
    }
}

impl AsRef<str> for LinkedAccountOptionsUsBankAccountPermissions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for LinkedAccountOptionsUsBankAccountPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for LinkedAccountOptionsUsBankAccountPermissions {
    fn default() -> Self {
        Self::Balances
    }
}

/// An enum representing the possible values of an `LinkedAccountOptionsUsBankAccount`'s `prefetch` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum LinkedAccountOptionsUsBankAccountPrefetch {
    Balances,
    Ownership,
    Transactions,
}

impl LinkedAccountOptionsUsBankAccountPrefetch {
    pub fn as_str(self) -> &'static str {
        match self {
            LinkedAccountOptionsUsBankAccountPrefetch::Balances => "balances",
            LinkedAccountOptionsUsBankAccountPrefetch::Ownership => "ownership",
            LinkedAccountOptionsUsBankAccountPrefetch::Transactions => "transactions",
        }
    }
}

impl AsRef<str> for LinkedAccountOptionsUsBankAccountPrefetch {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for LinkedAccountOptionsUsBankAccountPrefetch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for LinkedAccountOptionsUsBankAccountPrefetch {
    fn default() -> Self {
        Self::Balances
    }
}

/// An enum representing the possible values of an `PaymentFlowsPrivatePaymentMethodsUsBankAccountLinkedAccountOptionsFilters`'s `account_subcategories` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentFlowsPrivatePaymentMethodsUsBankAccountLinkedAccountOptionsFiltersAccountSubcategories
{
    Checking,
    Savings,
}

impl PaymentFlowsPrivatePaymentMethodsUsBankAccountLinkedAccountOptionsFiltersAccountSubcategories {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentFlowsPrivatePaymentMethodsUsBankAccountLinkedAccountOptionsFiltersAccountSubcategories::Checking => "checking",
            PaymentFlowsPrivatePaymentMethodsUsBankAccountLinkedAccountOptionsFiltersAccountSubcategories::Savings => "savings",
        }
    }
}

impl AsRef<str> for PaymentFlowsPrivatePaymentMethodsUsBankAccountLinkedAccountOptionsFiltersAccountSubcategories {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentFlowsPrivatePaymentMethodsUsBankAccountLinkedAccountOptionsFiltersAccountSubcategories {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentFlowsPrivatePaymentMethodsUsBankAccountLinkedAccountOptionsFiltersAccountSubcategories {
    fn default() -> Self {
        Self::Checking
    }
}

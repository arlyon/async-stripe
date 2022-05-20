// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "payment_method_options_customer_balance".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodOptionsCustomerBalance {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer: Option<PaymentMethodOptionsCustomerBalanceBankTransfer>,

    /// The funding method type to be used when there are not enough funds in the customer balance.
    ///
    /// Permitted values include: `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_type: Option<PaymentMethodOptionsCustomerBalanceFundingType>,

    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<PaymentMethodOptionsCustomerBalanceSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodOptionsCustomerBalanceBankTransfer {

    /// List of address types that should be returned in the financial_addresses response.
    ///
    /// If not specified, all valid types will be returned.  Permitted values include: `zengin`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_address_types: Option<Vec<PaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes>>,

    /// The bank transfer type that this PaymentIntent is allowed to use for funding.
    ///
    /// Permitted values include: `us_bank_account`, `eu_bank_account`, `id_bank_account`, `gb_bank_account`, `jp_bank_account`, `mx_bank_account`, `eu_bank_transfer`, `gb_bank_transfer`, `id_bank_transfer`, `jp_bank_transfer`, `mx_bank_transfer`, or `us_bank_transfer`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<PaymentMethodOptionsCustomerBalanceBankTransferType>,
}

/// An enum representing the possible values of an `PaymentMethodOptionsCustomerBalanceBankTransfer`'s `requested_address_types` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    Zengin,
}

impl PaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes::Zengin => "zengin",
        }
    }
}

impl AsRef<str> for PaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    fn default() -> Self {
        Self::Zengin
    }
}

/// An enum representing the possible values of an `PaymentMethodOptionsCustomerBalanceBankTransfer`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsCustomerBalanceBankTransferType {
    EuBankAccount,
    EuBankTransfer,
    GbBankAccount,
    GbBankTransfer,
    IdBankAccount,
    IdBankTransfer,
    JpBankAccount,
    JpBankTransfer,
    MxBankAccount,
    MxBankTransfer,
    UsBankAccount,
    UsBankTransfer,
}

impl PaymentMethodOptionsCustomerBalanceBankTransferType {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodOptionsCustomerBalanceBankTransferType::EuBankAccount => "eu_bank_account",
            PaymentMethodOptionsCustomerBalanceBankTransferType::EuBankTransfer => "eu_bank_transfer",
            PaymentMethodOptionsCustomerBalanceBankTransferType::GbBankAccount => "gb_bank_account",
            PaymentMethodOptionsCustomerBalanceBankTransferType::GbBankTransfer => "gb_bank_transfer",
            PaymentMethodOptionsCustomerBalanceBankTransferType::IdBankAccount => "id_bank_account",
            PaymentMethodOptionsCustomerBalanceBankTransferType::IdBankTransfer => "id_bank_transfer",
            PaymentMethodOptionsCustomerBalanceBankTransferType::JpBankAccount => "jp_bank_account",
            PaymentMethodOptionsCustomerBalanceBankTransferType::JpBankTransfer => "jp_bank_transfer",
            PaymentMethodOptionsCustomerBalanceBankTransferType::MxBankAccount => "mx_bank_account",
            PaymentMethodOptionsCustomerBalanceBankTransferType::MxBankTransfer => "mx_bank_transfer",
            PaymentMethodOptionsCustomerBalanceBankTransferType::UsBankAccount => "us_bank_account",
            PaymentMethodOptionsCustomerBalanceBankTransferType::UsBankTransfer => "us_bank_transfer",
        }
    }
}

impl AsRef<str> for PaymentMethodOptionsCustomerBalanceBankTransferType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodOptionsCustomerBalanceBankTransferType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodOptionsCustomerBalanceBankTransferType {
    fn default() -> Self {
        Self::EuBankAccount
    }
}

/// An enum representing the possible values of an `PaymentMethodOptionsCustomerBalance`'s `funding_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsCustomerBalanceFundingType {
    BankTransfer,
}

impl PaymentMethodOptionsCustomerBalanceFundingType {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodOptionsCustomerBalanceFundingType::BankTransfer => "bank_transfer",
        }
    }
}

impl AsRef<str> for PaymentMethodOptionsCustomerBalanceFundingType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodOptionsCustomerBalanceFundingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodOptionsCustomerBalanceFundingType {
    fn default() -> Self {
        Self::BankTransfer
    }
}

/// An enum representing the possible values of an `PaymentMethodOptionsCustomerBalance`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    None,
}

impl PaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodOptionsCustomerBalanceSetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for PaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

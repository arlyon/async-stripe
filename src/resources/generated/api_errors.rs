// ======================================
// This file was automatically generated.
// ======================================

use crate::resources::{PaymentIntent, PaymentMethod, PaymentSource, SetupIntent};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "APIErrors".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ApiErrors {
    /// For card errors, the ID of the failed charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<String>,

    /// For some errors that could be handled programmatically, a short string indicating the [error code](https://stripe.com/docs/error-codes) reported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<ApiErrorsCode>,

    /// For card errors resulting from a card issuer decline, a short string indicating the [card issuer's reason for the decline](https://stripe.com/docs/declines#issuer-declines) if they provide one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decline_code: Option<String>,

    /// A URL to more information about the [error code](https://stripe.com/docs/error-codes) reported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_url: Option<String>,

    /// A human-readable message providing more details about the error.
    ///
    /// For card errors, these messages can be shown to your users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// If the error is parameter-specific, the parameter related to the error.
    ///
    /// For example, you can use this to display a message near the correct form field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<PaymentIntent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<PaymentMethod>,

    /// If the error is specific to the type of payment method, the payment method type that had a problem.
    ///
    /// This field is only populated for invoice-related errors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_type: Option<String>,

    /// A URL to the request log entry in your dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_log_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_intent: Option<SetupIntent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<PaymentSource>,

    /// The type of error returned.
    ///
    /// One of `api_error`, `card_error`, `idempotency_error`, or `invalid_request_error`.
    #[serde(rename = "type")]
    pub type_: ApiErrorsType,
}

/// An enum representing the possible values of an `ApiErrors`'s `code` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ApiErrorsCode {
    AccountClosed,
    AccountCountryInvalidAddress,
    AccountErrorCountryChangeRequiresAdditionalSteps,
    AccountInformationMismatch,
    AccountInvalid,
    AccountNumberInvalid,
    AcssDebitSessionIncomplete,
    AlipayUpgradeRequired,
    AmountTooLarge,
    AmountTooSmall,
    ApiKeyExpired,
    ApplicationFeesNotAllowed,
    AuthenticationRequired,
    BalanceInsufficient,
    BalanceInvalidParameter,
    BankAccountBadRoutingNumbers,
    BankAccountDeclined,
    BankAccountExists,
    BankAccountRestricted,
    BankAccountUnusable,
    BankAccountUnverified,
    BankAccountVerificationFailed,
    BillingInvalidMandate,
    BitcoinUpgradeRequired,
    CaptureChargeAuthorizationExpired,
    CaptureUnauthorizedPayment,
    CardDeclineRateLimitExceeded,
    CardDeclined,
    CardholderPhoneNumberRequired,
    ChargeAlreadyCaptured,
    ChargeAlreadyRefunded,
    ChargeDisputed,
    ChargeExceedsSourceLimit,
    ChargeExpiredForCapture,
    ChargeInvalidParameter,
    ChargeNotRefundable,
    ClearingCodeUnsupported,
    CountryCodeInvalid,
    CountryUnsupported,
    CouponExpired,
    CustomerMaxPaymentMethods,
    CustomerMaxSubscriptions,
    CustomerTaxLocationInvalid,
    DebitNotAuthorized,
    EmailInvalid,
    ExpiredCard,
    FinancialConnectionsAccountInactive,
    FinancialConnectionsNoSuccessfulTransactionRefresh,
    IdempotencyKeyInUse,
    IncorrectAddress,
    IncorrectCvc,
    IncorrectNumber,
    IncorrectZip,
    InstantPayoutsConfigDisabled,
    InstantPayoutsCurrencyDisabled,
    InstantPayoutsLimitExceeded,
    InstantPayoutsUnsupported,
    InsufficientFunds,
    IntentInvalidState,
    IntentVerificationMethodMissing,
    InvalidCardType,
    InvalidCharacters,
    InvalidChargeAmount,
    InvalidCvc,
    InvalidExpiryMonth,
    InvalidExpiryYear,
    InvalidNumber,
    InvalidSourceUsage,
    InvalidTaxLocation,
    InvoiceNoCustomerLineItems,
    InvoiceNoPaymentMethodTypes,
    InvoiceNoSubscriptionLineItems,
    InvoiceNotEditable,
    InvoiceOnBehalfOfNotEditable,
    InvoicePaymentIntentRequiresAction,
    InvoiceUpcomingNone,
    LivemodeMismatch,
    LockTimeout,
    Missing,
    NoAccount,
    NotAllowedOnStandardAccount,
    OutOfInventory,
    OwnershipDeclarationNotAllowed,
    ParameterInvalidEmpty,
    ParameterInvalidInteger,
    ParameterInvalidStringBlank,
    ParameterInvalidStringEmpty,
    ParameterMissing,
    ParameterUnknown,
    ParametersExclusive,
    PaymentIntentActionRequired,
    PaymentIntentAuthenticationFailure,
    PaymentIntentIncompatiblePaymentMethod,
    PaymentIntentInvalidParameter,
    PaymentIntentKonbiniRejectedConfirmationNumber,
    PaymentIntentMandateInvalid,
    PaymentIntentPaymentAttemptExpired,
    PaymentIntentPaymentAttemptFailed,
    PaymentIntentUnexpectedState,
    PaymentMethodBankAccountAlreadyVerified,
    PaymentMethodBankAccountBlocked,
    PaymentMethodBillingDetailsAddressMissing,
    PaymentMethodConfigurationFailures,
    PaymentMethodCurrencyMismatch,
    PaymentMethodCustomerDecline,
    PaymentMethodInvalidParameter,
    PaymentMethodInvalidParameterTestmode,
    PaymentMethodMicrodepositFailed,
    PaymentMethodMicrodepositVerificationAmountsInvalid,
    PaymentMethodMicrodepositVerificationAmountsMismatch,
    PaymentMethodMicrodepositVerificationAttemptsExceeded,
    PaymentMethodMicrodepositVerificationDescriptorCodeMismatch,
    PaymentMethodMicrodepositVerificationTimeout,
    PaymentMethodNotAvailable,
    PaymentMethodProviderDecline,
    PaymentMethodProviderTimeout,
    PaymentMethodUnactivated,
    PaymentMethodUnexpectedState,
    PaymentMethodUnsupportedType,
    PayoutReconciliationNotReady,
    PayoutsLimitExceeded,
    PayoutsNotAllowed,
    PlatformAccountRequired,
    PlatformApiKeyExpired,
    PostalCodeInvalid,
    ProcessingError,
    ProductInactive,
    ProgressiveOnboardingLimitExceeded,
    RateLimit,
    ReferToCustomer,
    RefundDisputedPayment,
    ResourceAlreadyExists,
    ResourceMissing,
    ReturnIntentAlreadyProcessed,
    RoutingNumberInvalid,
    SecretKeyRequired,
    SepaUnsupportedAccount,
    SetupAttemptFailed,
    SetupIntentAuthenticationFailure,
    SetupIntentInvalidParameter,
    SetupIntentMandateInvalid,
    SetupIntentSetupAttemptExpired,
    SetupIntentUnexpectedState,
    ShippingCalculationFailed,
    SkuInactive,
    StateUnsupported,
    StatusTransitionInvalid,
    StripeTaxInactive,
    TaxIdInvalid,
    TaxesCalculationFailed,
    TerminalLocationCountryUnsupported,
    TerminalReaderBusy,
    TerminalReaderHardwareFault,
    TerminalReaderOffline,
    TerminalReaderTimeout,
    TestmodeChargesOnly,
    TlsVersionUnsupported,
    TokenAlreadyUsed,
    TokenCardNetworkInvalid,
    TokenInUse,
    TransferSourceBalanceParametersMismatch,
    TransfersNotAllowed,
    UrlInvalid,
}

impl ApiErrorsCode {
    pub fn as_str(self) -> &'static str {
        match self {
            ApiErrorsCode::AccountClosed => "account_closed",
            ApiErrorsCode::AccountCountryInvalidAddress => "account_country_invalid_address",
            ApiErrorsCode::AccountErrorCountryChangeRequiresAdditionalSteps => {
                "account_error_country_change_requires_additional_steps"
            }
            ApiErrorsCode::AccountInformationMismatch => "account_information_mismatch",
            ApiErrorsCode::AccountInvalid => "account_invalid",
            ApiErrorsCode::AccountNumberInvalid => "account_number_invalid",
            ApiErrorsCode::AcssDebitSessionIncomplete => "acss_debit_session_incomplete",
            ApiErrorsCode::AlipayUpgradeRequired => "alipay_upgrade_required",
            ApiErrorsCode::AmountTooLarge => "amount_too_large",
            ApiErrorsCode::AmountTooSmall => "amount_too_small",
            ApiErrorsCode::ApiKeyExpired => "api_key_expired",
            ApiErrorsCode::ApplicationFeesNotAllowed => "application_fees_not_allowed",
            ApiErrorsCode::AuthenticationRequired => "authentication_required",
            ApiErrorsCode::BalanceInsufficient => "balance_insufficient",
            ApiErrorsCode::BalanceInvalidParameter => "balance_invalid_parameter",
            ApiErrorsCode::BankAccountBadRoutingNumbers => "bank_account_bad_routing_numbers",
            ApiErrorsCode::BankAccountDeclined => "bank_account_declined",
            ApiErrorsCode::BankAccountExists => "bank_account_exists",
            ApiErrorsCode::BankAccountRestricted => "bank_account_restricted",
            ApiErrorsCode::BankAccountUnusable => "bank_account_unusable",
            ApiErrorsCode::BankAccountUnverified => "bank_account_unverified",
            ApiErrorsCode::BankAccountVerificationFailed => "bank_account_verification_failed",
            ApiErrorsCode::BillingInvalidMandate => "billing_invalid_mandate",
            ApiErrorsCode::BitcoinUpgradeRequired => "bitcoin_upgrade_required",
            ApiErrorsCode::CaptureChargeAuthorizationExpired => {
                "capture_charge_authorization_expired"
            }
            ApiErrorsCode::CaptureUnauthorizedPayment => "capture_unauthorized_payment",
            ApiErrorsCode::CardDeclineRateLimitExceeded => "card_decline_rate_limit_exceeded",
            ApiErrorsCode::CardDeclined => "card_declined",
            ApiErrorsCode::CardholderPhoneNumberRequired => "cardholder_phone_number_required",
            ApiErrorsCode::ChargeAlreadyCaptured => "charge_already_captured",
            ApiErrorsCode::ChargeAlreadyRefunded => "charge_already_refunded",
            ApiErrorsCode::ChargeDisputed => "charge_disputed",
            ApiErrorsCode::ChargeExceedsSourceLimit => "charge_exceeds_source_limit",
            ApiErrorsCode::ChargeExpiredForCapture => "charge_expired_for_capture",
            ApiErrorsCode::ChargeInvalidParameter => "charge_invalid_parameter",
            ApiErrorsCode::ChargeNotRefundable => "charge_not_refundable",
            ApiErrorsCode::ClearingCodeUnsupported => "clearing_code_unsupported",
            ApiErrorsCode::CountryCodeInvalid => "country_code_invalid",
            ApiErrorsCode::CountryUnsupported => "country_unsupported",
            ApiErrorsCode::CouponExpired => "coupon_expired",
            ApiErrorsCode::CustomerMaxPaymentMethods => "customer_max_payment_methods",
            ApiErrorsCode::CustomerMaxSubscriptions => "customer_max_subscriptions",
            ApiErrorsCode::CustomerTaxLocationInvalid => "customer_tax_location_invalid",
            ApiErrorsCode::DebitNotAuthorized => "debit_not_authorized",
            ApiErrorsCode::EmailInvalid => "email_invalid",
            ApiErrorsCode::ExpiredCard => "expired_card",
            ApiErrorsCode::FinancialConnectionsAccountInactive => {
                "financial_connections_account_inactive"
            }
            ApiErrorsCode::FinancialConnectionsNoSuccessfulTransactionRefresh => {
                "financial_connections_no_successful_transaction_refresh"
            }
            ApiErrorsCode::IdempotencyKeyInUse => "idempotency_key_in_use",
            ApiErrorsCode::IncorrectAddress => "incorrect_address",
            ApiErrorsCode::IncorrectCvc => "incorrect_cvc",
            ApiErrorsCode::IncorrectNumber => "incorrect_number",
            ApiErrorsCode::IncorrectZip => "incorrect_zip",
            ApiErrorsCode::InstantPayoutsConfigDisabled => "instant_payouts_config_disabled",
            ApiErrorsCode::InstantPayoutsCurrencyDisabled => "instant_payouts_currency_disabled",
            ApiErrorsCode::InstantPayoutsLimitExceeded => "instant_payouts_limit_exceeded",
            ApiErrorsCode::InstantPayoutsUnsupported => "instant_payouts_unsupported",
            ApiErrorsCode::InsufficientFunds => "insufficient_funds",
            ApiErrorsCode::IntentInvalidState => "intent_invalid_state",
            ApiErrorsCode::IntentVerificationMethodMissing => "intent_verification_method_missing",
            ApiErrorsCode::InvalidCardType => "invalid_card_type",
            ApiErrorsCode::InvalidCharacters => "invalid_characters",
            ApiErrorsCode::InvalidChargeAmount => "invalid_charge_amount",
            ApiErrorsCode::InvalidCvc => "invalid_cvc",
            ApiErrorsCode::InvalidExpiryMonth => "invalid_expiry_month",
            ApiErrorsCode::InvalidExpiryYear => "invalid_expiry_year",
            ApiErrorsCode::InvalidNumber => "invalid_number",
            ApiErrorsCode::InvalidSourceUsage => "invalid_source_usage",
            ApiErrorsCode::InvalidTaxLocation => "invalid_tax_location",
            ApiErrorsCode::InvoiceNoCustomerLineItems => "invoice_no_customer_line_items",
            ApiErrorsCode::InvoiceNoPaymentMethodTypes => "invoice_no_payment_method_types",
            ApiErrorsCode::InvoiceNoSubscriptionLineItems => "invoice_no_subscription_line_items",
            ApiErrorsCode::InvoiceNotEditable => "invoice_not_editable",
            ApiErrorsCode::InvoiceOnBehalfOfNotEditable => "invoice_on_behalf_of_not_editable",
            ApiErrorsCode::InvoicePaymentIntentRequiresAction => {
                "invoice_payment_intent_requires_action"
            }
            ApiErrorsCode::InvoiceUpcomingNone => "invoice_upcoming_none",
            ApiErrorsCode::LivemodeMismatch => "livemode_mismatch",
            ApiErrorsCode::LockTimeout => "lock_timeout",
            ApiErrorsCode::Missing => "missing",
            ApiErrorsCode::NoAccount => "no_account",
            ApiErrorsCode::NotAllowedOnStandardAccount => "not_allowed_on_standard_account",
            ApiErrorsCode::OutOfInventory => "out_of_inventory",
            ApiErrorsCode::OwnershipDeclarationNotAllowed => "ownership_declaration_not_allowed",
            ApiErrorsCode::ParameterInvalidEmpty => "parameter_invalid_empty",
            ApiErrorsCode::ParameterInvalidInteger => "parameter_invalid_integer",
            ApiErrorsCode::ParameterInvalidStringBlank => "parameter_invalid_string_blank",
            ApiErrorsCode::ParameterInvalidStringEmpty => "parameter_invalid_string_empty",
            ApiErrorsCode::ParameterMissing => "parameter_missing",
            ApiErrorsCode::ParameterUnknown => "parameter_unknown",
            ApiErrorsCode::ParametersExclusive => "parameters_exclusive",
            ApiErrorsCode::PaymentIntentActionRequired => "payment_intent_action_required",
            ApiErrorsCode::PaymentIntentAuthenticationFailure => {
                "payment_intent_authentication_failure"
            }
            ApiErrorsCode::PaymentIntentIncompatiblePaymentMethod => {
                "payment_intent_incompatible_payment_method"
            }
            ApiErrorsCode::PaymentIntentInvalidParameter => "payment_intent_invalid_parameter",
            ApiErrorsCode::PaymentIntentKonbiniRejectedConfirmationNumber => {
                "payment_intent_konbini_rejected_confirmation_number"
            }
            ApiErrorsCode::PaymentIntentMandateInvalid => "payment_intent_mandate_invalid",
            ApiErrorsCode::PaymentIntentPaymentAttemptExpired => {
                "payment_intent_payment_attempt_expired"
            }
            ApiErrorsCode::PaymentIntentPaymentAttemptFailed => {
                "payment_intent_payment_attempt_failed"
            }
            ApiErrorsCode::PaymentIntentUnexpectedState => "payment_intent_unexpected_state",
            ApiErrorsCode::PaymentMethodBankAccountAlreadyVerified => {
                "payment_method_bank_account_already_verified"
            }
            ApiErrorsCode::PaymentMethodBankAccountBlocked => "payment_method_bank_account_blocked",
            ApiErrorsCode::PaymentMethodBillingDetailsAddressMissing => {
                "payment_method_billing_details_address_missing"
            }
            ApiErrorsCode::PaymentMethodConfigurationFailures => {
                "payment_method_configuration_failures"
            }
            ApiErrorsCode::PaymentMethodCurrencyMismatch => "payment_method_currency_mismatch",
            ApiErrorsCode::PaymentMethodCustomerDecline => "payment_method_customer_decline",
            ApiErrorsCode::PaymentMethodInvalidParameter => "payment_method_invalid_parameter",
            ApiErrorsCode::PaymentMethodInvalidParameterTestmode => {
                "payment_method_invalid_parameter_testmode"
            }
            ApiErrorsCode::PaymentMethodMicrodepositFailed => "payment_method_microdeposit_failed",
            ApiErrorsCode::PaymentMethodMicrodepositVerificationAmountsInvalid => {
                "payment_method_microdeposit_verification_amounts_invalid"
            }
            ApiErrorsCode::PaymentMethodMicrodepositVerificationAmountsMismatch => {
                "payment_method_microdeposit_verification_amounts_mismatch"
            }
            ApiErrorsCode::PaymentMethodMicrodepositVerificationAttemptsExceeded => {
                "payment_method_microdeposit_verification_attempts_exceeded"
            }
            ApiErrorsCode::PaymentMethodMicrodepositVerificationDescriptorCodeMismatch => {
                "payment_method_microdeposit_verification_descriptor_code_mismatch"
            }
            ApiErrorsCode::PaymentMethodMicrodepositVerificationTimeout => {
                "payment_method_microdeposit_verification_timeout"
            }
            ApiErrorsCode::PaymentMethodNotAvailable => "payment_method_not_available",
            ApiErrorsCode::PaymentMethodProviderDecline => "payment_method_provider_decline",
            ApiErrorsCode::PaymentMethodProviderTimeout => "payment_method_provider_timeout",
            ApiErrorsCode::PaymentMethodUnactivated => "payment_method_unactivated",
            ApiErrorsCode::PaymentMethodUnexpectedState => "payment_method_unexpected_state",
            ApiErrorsCode::PaymentMethodUnsupportedType => "payment_method_unsupported_type",
            ApiErrorsCode::PayoutReconciliationNotReady => "payout_reconciliation_not_ready",
            ApiErrorsCode::PayoutsLimitExceeded => "payouts_limit_exceeded",
            ApiErrorsCode::PayoutsNotAllowed => "payouts_not_allowed",
            ApiErrorsCode::PlatformAccountRequired => "platform_account_required",
            ApiErrorsCode::PlatformApiKeyExpired => "platform_api_key_expired",
            ApiErrorsCode::PostalCodeInvalid => "postal_code_invalid",
            ApiErrorsCode::ProcessingError => "processing_error",
            ApiErrorsCode::ProductInactive => "product_inactive",
            ApiErrorsCode::ProgressiveOnboardingLimitExceeded => {
                "progressive_onboarding_limit_exceeded"
            }
            ApiErrorsCode::RateLimit => "rate_limit",
            ApiErrorsCode::ReferToCustomer => "refer_to_customer",
            ApiErrorsCode::RefundDisputedPayment => "refund_disputed_payment",
            ApiErrorsCode::ResourceAlreadyExists => "resource_already_exists",
            ApiErrorsCode::ResourceMissing => "resource_missing",
            ApiErrorsCode::ReturnIntentAlreadyProcessed => "return_intent_already_processed",
            ApiErrorsCode::RoutingNumberInvalid => "routing_number_invalid",
            ApiErrorsCode::SecretKeyRequired => "secret_key_required",
            ApiErrorsCode::SepaUnsupportedAccount => "sepa_unsupported_account",
            ApiErrorsCode::SetupAttemptFailed => "setup_attempt_failed",
            ApiErrorsCode::SetupIntentAuthenticationFailure => {
                "setup_intent_authentication_failure"
            }
            ApiErrorsCode::SetupIntentInvalidParameter => "setup_intent_invalid_parameter",
            ApiErrorsCode::SetupIntentMandateInvalid => "setup_intent_mandate_invalid",
            ApiErrorsCode::SetupIntentSetupAttemptExpired => "setup_intent_setup_attempt_expired",
            ApiErrorsCode::SetupIntentUnexpectedState => "setup_intent_unexpected_state",
            ApiErrorsCode::ShippingCalculationFailed => "shipping_calculation_failed",
            ApiErrorsCode::SkuInactive => "sku_inactive",
            ApiErrorsCode::StateUnsupported => "state_unsupported",
            ApiErrorsCode::StatusTransitionInvalid => "status_transition_invalid",
            ApiErrorsCode::StripeTaxInactive => "stripe_tax_inactive",
            ApiErrorsCode::TaxIdInvalid => "tax_id_invalid",
            ApiErrorsCode::TaxesCalculationFailed => "taxes_calculation_failed",
            ApiErrorsCode::TerminalLocationCountryUnsupported => {
                "terminal_location_country_unsupported"
            }
            ApiErrorsCode::TerminalReaderBusy => "terminal_reader_busy",
            ApiErrorsCode::TerminalReaderHardwareFault => "terminal_reader_hardware_fault",
            ApiErrorsCode::TerminalReaderOffline => "terminal_reader_offline",
            ApiErrorsCode::TerminalReaderTimeout => "terminal_reader_timeout",
            ApiErrorsCode::TestmodeChargesOnly => "testmode_charges_only",
            ApiErrorsCode::TlsVersionUnsupported => "tls_version_unsupported",
            ApiErrorsCode::TokenAlreadyUsed => "token_already_used",
            ApiErrorsCode::TokenCardNetworkInvalid => "token_card_network_invalid",
            ApiErrorsCode::TokenInUse => "token_in_use",
            ApiErrorsCode::TransferSourceBalanceParametersMismatch => {
                "transfer_source_balance_parameters_mismatch"
            }
            ApiErrorsCode::TransfersNotAllowed => "transfers_not_allowed",
            ApiErrorsCode::UrlInvalid => "url_invalid",
        }
    }
}

impl AsRef<str> for ApiErrorsCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ApiErrorsCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for ApiErrorsCode {
    fn default() -> Self {
        Self::AccountClosed
    }
}

/// An enum representing the possible values of an `ApiErrors`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ApiErrorsType {
    ApiError,
    CardError,
    IdempotencyError,
    InvalidRequestError,
}

impl ApiErrorsType {
    pub fn as_str(self) -> &'static str {
        match self {
            ApiErrorsType::ApiError => "api_error",
            ApiErrorsType::CardError => "card_error",
            ApiErrorsType::IdempotencyError => "idempotency_error",
            ApiErrorsType::InvalidRequestError => "invalid_request_error",
        }
    }
}

impl AsRef<str> for ApiErrorsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ApiErrorsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for ApiErrorsType {
    fn default() -> Self {
        Self::ApiError
    }
}

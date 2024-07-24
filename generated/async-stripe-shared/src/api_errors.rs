#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ApiErrors {
    /// For card errors, the ID of the failed charge.
    pub charge: Option<String>,
    /// For some errors that could be handled programmatically, a short string indicating the [error code](https://stripe.com/docs/error-codes) reported.
    pub code: Option<ApiErrorsCode>,
    /// For card errors resulting from a card issuer decline, a short string indicating the [card issuer's reason for the decline](https://stripe.com/docs/declines#issuer-declines) if they provide one.
    pub decline_code: Option<String>,
    /// A URL to more information about the [error code](https://stripe.com/docs/error-codes) reported.
    pub doc_url: Option<String>,
    /// A human-readable message providing more details about the error.
    /// For card errors, these messages can be shown to your users.
    pub message: Option<String>,
    /// If the error is parameter-specific, the parameter related to the error.
    /// For example, you can use this to display a message near the correct form field.
    pub param: Option<String>,
    pub payment_intent: Option<stripe_shared::PaymentIntent>,
    pub payment_method: Option<stripe_shared::PaymentMethod>,
    /// If the error is specific to the type of payment method, the payment method type that had a problem.
    /// This field is only populated for invoice-related errors.
    pub payment_method_type: Option<String>,
    /// A URL to the request log entry in your dashboard.
    pub request_log_url: Option<String>,
    pub setup_intent: Option<stripe_shared::SetupIntent>,
    pub source: Option<stripe_shared::PaymentSource>,
    /// The type of error returned.
    /// One of `api_error`, `card_error`, `idempotency_error`, or `invalid_request_error`.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: ApiErrorsType,
}
#[doc(hidden)]
pub struct ApiErrorsBuilder {
    charge: Option<Option<String>>,
    code: Option<Option<ApiErrorsCode>>,
    decline_code: Option<Option<String>>,
    doc_url: Option<Option<String>>,
    message: Option<Option<String>>,
    param: Option<Option<String>>,
    payment_intent: Option<Option<stripe_shared::PaymentIntent>>,
    payment_method: Option<Option<stripe_shared::PaymentMethod>>,
    payment_method_type: Option<Option<String>>,
    request_log_url: Option<Option<String>>,
    setup_intent: Option<Option<stripe_shared::SetupIntent>>,
    source: Option<Option<stripe_shared::PaymentSource>>,
    type_: Option<ApiErrorsType>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for ApiErrors {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ApiErrors>,
        builder: ApiErrorsBuilder,
    }

    impl Visitor for Place<ApiErrors> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: ApiErrorsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for ApiErrorsBuilder {
        type Out = ApiErrors;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "charge" => Deserialize::begin(&mut self.charge),
                "code" => Deserialize::begin(&mut self.code),
                "decline_code" => Deserialize::begin(&mut self.decline_code),
                "doc_url" => Deserialize::begin(&mut self.doc_url),
                "message" => Deserialize::begin(&mut self.message),
                "param" => Deserialize::begin(&mut self.param),
                "payment_intent" => Deserialize::begin(&mut self.payment_intent),
                "payment_method" => Deserialize::begin(&mut self.payment_method),
                "payment_method_type" => Deserialize::begin(&mut self.payment_method_type),
                "request_log_url" => Deserialize::begin(&mut self.request_log_url),
                "setup_intent" => Deserialize::begin(&mut self.setup_intent),
                "source" => Deserialize::begin(&mut self.source),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                charge: Deserialize::default(),
                code: Deserialize::default(),
                decline_code: Deserialize::default(),
                doc_url: Deserialize::default(),
                message: Deserialize::default(),
                param: Deserialize::default(),
                payment_intent: Deserialize::default(),
                payment_method: Deserialize::default(),
                payment_method_type: Deserialize::default(),
                request_log_url: Deserialize::default(),
                setup_intent: Deserialize::default(),
                source: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                charge: self.charge.take()?,
                code: self.code?,
                decline_code: self.decline_code.take()?,
                doc_url: self.doc_url.take()?,
                message: self.message.take()?,
                param: self.param.take()?,
                payment_intent: self.payment_intent.take()?,
                payment_method: self.payment_method.take()?,
                payment_method_type: self.payment_method_type.take()?,
                request_log_url: self.request_log_url.take()?,
                setup_intent: self.setup_intent.take()?,
                source: self.source.take()?,
                type_: self.type_?,
            })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for ApiErrors {
        type Builder = ApiErrorsBuilder;
    }

    impl FromValueOpt for ApiErrors {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ApiErrorsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "charge" => b.charge = Some(FromValueOpt::from_value(v)?),
                    "code" => b.code = Some(FromValueOpt::from_value(v)?),
                    "decline_code" => b.decline_code = Some(FromValueOpt::from_value(v)?),
                    "doc_url" => b.doc_url = Some(FromValueOpt::from_value(v)?),
                    "message" => b.message = Some(FromValueOpt::from_value(v)?),
                    "param" => b.param = Some(FromValueOpt::from_value(v)?),
                    "payment_intent" => b.payment_intent = Some(FromValueOpt::from_value(v)?),
                    "payment_method" => b.payment_method = Some(FromValueOpt::from_value(v)?),
                    "payment_method_type" => {
                        b.payment_method_type = Some(FromValueOpt::from_value(v)?)
                    }
                    "request_log_url" => b.request_log_url = Some(FromValueOpt::from_value(v)?),
                    "setup_intent" => b.setup_intent = Some(FromValueOpt::from_value(v)?),
                    "source" => b.source = Some(FromValueOpt::from_value(v)?),
                    "type" => b.type_ = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// For some errors that could be handled programmatically, a short string indicating the [error code](https://stripe.com/docs/error-codes) reported.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
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
    BillingPolicyRemoteFunctionResponseInvalid,
    BillingPolicyRemoteFunctionTimeout,
    BillingPolicyRemoteFunctionUnexpectedStatusCode,
    BillingPolicyRemoteFunctionUnreachable,
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
    ForwardingApiInactive,
    ForwardingApiInvalidParameter,
    ForwardingApiUpstreamConnectionError,
    ForwardingApiUpstreamConnectionTimeout,
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
    ShippingAddressInvalid,
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
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl ApiErrorsCode {
    pub fn as_str(self) -> &'static str {
        use ApiErrorsCode::*;
        match self {
            AccountClosed => "account_closed",
            AccountCountryInvalidAddress => "account_country_invalid_address",
            AccountErrorCountryChangeRequiresAdditionalSteps => {
                "account_error_country_change_requires_additional_steps"
            }
            AccountInformationMismatch => "account_information_mismatch",
            AccountInvalid => "account_invalid",
            AccountNumberInvalid => "account_number_invalid",
            AcssDebitSessionIncomplete => "acss_debit_session_incomplete",
            AlipayUpgradeRequired => "alipay_upgrade_required",
            AmountTooLarge => "amount_too_large",
            AmountTooSmall => "amount_too_small",
            ApiKeyExpired => "api_key_expired",
            ApplicationFeesNotAllowed => "application_fees_not_allowed",
            AuthenticationRequired => "authentication_required",
            BalanceInsufficient => "balance_insufficient",
            BalanceInvalidParameter => "balance_invalid_parameter",
            BankAccountBadRoutingNumbers => "bank_account_bad_routing_numbers",
            BankAccountDeclined => "bank_account_declined",
            BankAccountExists => "bank_account_exists",
            BankAccountRestricted => "bank_account_restricted",
            BankAccountUnusable => "bank_account_unusable",
            BankAccountUnverified => "bank_account_unverified",
            BankAccountVerificationFailed => "bank_account_verification_failed",
            BillingInvalidMandate => "billing_invalid_mandate",
            BillingPolicyRemoteFunctionResponseInvalid => {
                "billing_policy_remote_function_response_invalid"
            }
            BillingPolicyRemoteFunctionTimeout => "billing_policy_remote_function_timeout",
            BillingPolicyRemoteFunctionUnexpectedStatusCode => {
                "billing_policy_remote_function_unexpected_status_code"
            }
            BillingPolicyRemoteFunctionUnreachable => "billing_policy_remote_function_unreachable",
            BitcoinUpgradeRequired => "bitcoin_upgrade_required",
            CaptureChargeAuthorizationExpired => "capture_charge_authorization_expired",
            CaptureUnauthorizedPayment => "capture_unauthorized_payment",
            CardDeclineRateLimitExceeded => "card_decline_rate_limit_exceeded",
            CardDeclined => "card_declined",
            CardholderPhoneNumberRequired => "cardholder_phone_number_required",
            ChargeAlreadyCaptured => "charge_already_captured",
            ChargeAlreadyRefunded => "charge_already_refunded",
            ChargeDisputed => "charge_disputed",
            ChargeExceedsSourceLimit => "charge_exceeds_source_limit",
            ChargeExpiredForCapture => "charge_expired_for_capture",
            ChargeInvalidParameter => "charge_invalid_parameter",
            ChargeNotRefundable => "charge_not_refundable",
            ClearingCodeUnsupported => "clearing_code_unsupported",
            CountryCodeInvalid => "country_code_invalid",
            CountryUnsupported => "country_unsupported",
            CouponExpired => "coupon_expired",
            CustomerMaxPaymentMethods => "customer_max_payment_methods",
            CustomerMaxSubscriptions => "customer_max_subscriptions",
            CustomerTaxLocationInvalid => "customer_tax_location_invalid",
            DebitNotAuthorized => "debit_not_authorized",
            EmailInvalid => "email_invalid",
            ExpiredCard => "expired_card",
            FinancialConnectionsAccountInactive => "financial_connections_account_inactive",
            FinancialConnectionsNoSuccessfulTransactionRefresh => {
                "financial_connections_no_successful_transaction_refresh"
            }
            ForwardingApiInactive => "forwarding_api_inactive",
            ForwardingApiInvalidParameter => "forwarding_api_invalid_parameter",
            ForwardingApiUpstreamConnectionError => "forwarding_api_upstream_connection_error",
            ForwardingApiUpstreamConnectionTimeout => "forwarding_api_upstream_connection_timeout",
            IdempotencyKeyInUse => "idempotency_key_in_use",
            IncorrectAddress => "incorrect_address",
            IncorrectCvc => "incorrect_cvc",
            IncorrectNumber => "incorrect_number",
            IncorrectZip => "incorrect_zip",
            InstantPayoutsConfigDisabled => "instant_payouts_config_disabled",
            InstantPayoutsCurrencyDisabled => "instant_payouts_currency_disabled",
            InstantPayoutsLimitExceeded => "instant_payouts_limit_exceeded",
            InstantPayoutsUnsupported => "instant_payouts_unsupported",
            InsufficientFunds => "insufficient_funds",
            IntentInvalidState => "intent_invalid_state",
            IntentVerificationMethodMissing => "intent_verification_method_missing",
            InvalidCardType => "invalid_card_type",
            InvalidCharacters => "invalid_characters",
            InvalidChargeAmount => "invalid_charge_amount",
            InvalidCvc => "invalid_cvc",
            InvalidExpiryMonth => "invalid_expiry_month",
            InvalidExpiryYear => "invalid_expiry_year",
            InvalidNumber => "invalid_number",
            InvalidSourceUsage => "invalid_source_usage",
            InvalidTaxLocation => "invalid_tax_location",
            InvoiceNoCustomerLineItems => "invoice_no_customer_line_items",
            InvoiceNoPaymentMethodTypes => "invoice_no_payment_method_types",
            InvoiceNoSubscriptionLineItems => "invoice_no_subscription_line_items",
            InvoiceNotEditable => "invoice_not_editable",
            InvoiceOnBehalfOfNotEditable => "invoice_on_behalf_of_not_editable",
            InvoicePaymentIntentRequiresAction => "invoice_payment_intent_requires_action",
            InvoiceUpcomingNone => "invoice_upcoming_none",
            LivemodeMismatch => "livemode_mismatch",
            LockTimeout => "lock_timeout",
            Missing => "missing",
            NoAccount => "no_account",
            NotAllowedOnStandardAccount => "not_allowed_on_standard_account",
            OutOfInventory => "out_of_inventory",
            OwnershipDeclarationNotAllowed => "ownership_declaration_not_allowed",
            ParameterInvalidEmpty => "parameter_invalid_empty",
            ParameterInvalidInteger => "parameter_invalid_integer",
            ParameterInvalidStringBlank => "parameter_invalid_string_blank",
            ParameterInvalidStringEmpty => "parameter_invalid_string_empty",
            ParameterMissing => "parameter_missing",
            ParameterUnknown => "parameter_unknown",
            ParametersExclusive => "parameters_exclusive",
            PaymentIntentActionRequired => "payment_intent_action_required",
            PaymentIntentAuthenticationFailure => "payment_intent_authentication_failure",
            PaymentIntentIncompatiblePaymentMethod => "payment_intent_incompatible_payment_method",
            PaymentIntentInvalidParameter => "payment_intent_invalid_parameter",
            PaymentIntentKonbiniRejectedConfirmationNumber => {
                "payment_intent_konbini_rejected_confirmation_number"
            }
            PaymentIntentMandateInvalid => "payment_intent_mandate_invalid",
            PaymentIntentPaymentAttemptExpired => "payment_intent_payment_attempt_expired",
            PaymentIntentPaymentAttemptFailed => "payment_intent_payment_attempt_failed",
            PaymentIntentUnexpectedState => "payment_intent_unexpected_state",
            PaymentMethodBankAccountAlreadyVerified => {
                "payment_method_bank_account_already_verified"
            }
            PaymentMethodBankAccountBlocked => "payment_method_bank_account_blocked",
            PaymentMethodBillingDetailsAddressMissing => {
                "payment_method_billing_details_address_missing"
            }
            PaymentMethodConfigurationFailures => "payment_method_configuration_failures",
            PaymentMethodCurrencyMismatch => "payment_method_currency_mismatch",
            PaymentMethodCustomerDecline => "payment_method_customer_decline",
            PaymentMethodInvalidParameter => "payment_method_invalid_parameter",
            PaymentMethodInvalidParameterTestmode => "payment_method_invalid_parameter_testmode",
            PaymentMethodMicrodepositFailed => "payment_method_microdeposit_failed",
            PaymentMethodMicrodepositVerificationAmountsInvalid => {
                "payment_method_microdeposit_verification_amounts_invalid"
            }
            PaymentMethodMicrodepositVerificationAmountsMismatch => {
                "payment_method_microdeposit_verification_amounts_mismatch"
            }
            PaymentMethodMicrodepositVerificationAttemptsExceeded => {
                "payment_method_microdeposit_verification_attempts_exceeded"
            }
            PaymentMethodMicrodepositVerificationDescriptorCodeMismatch => {
                "payment_method_microdeposit_verification_descriptor_code_mismatch"
            }
            PaymentMethodMicrodepositVerificationTimeout => {
                "payment_method_microdeposit_verification_timeout"
            }
            PaymentMethodNotAvailable => "payment_method_not_available",
            PaymentMethodProviderDecline => "payment_method_provider_decline",
            PaymentMethodProviderTimeout => "payment_method_provider_timeout",
            PaymentMethodUnactivated => "payment_method_unactivated",
            PaymentMethodUnexpectedState => "payment_method_unexpected_state",
            PaymentMethodUnsupportedType => "payment_method_unsupported_type",
            PayoutReconciliationNotReady => "payout_reconciliation_not_ready",
            PayoutsLimitExceeded => "payouts_limit_exceeded",
            PayoutsNotAllowed => "payouts_not_allowed",
            PlatformAccountRequired => "platform_account_required",
            PlatformApiKeyExpired => "platform_api_key_expired",
            PostalCodeInvalid => "postal_code_invalid",
            ProcessingError => "processing_error",
            ProductInactive => "product_inactive",
            ProgressiveOnboardingLimitExceeded => "progressive_onboarding_limit_exceeded",
            RateLimit => "rate_limit",
            ReferToCustomer => "refer_to_customer",
            RefundDisputedPayment => "refund_disputed_payment",
            ResourceAlreadyExists => "resource_already_exists",
            ResourceMissing => "resource_missing",
            ReturnIntentAlreadyProcessed => "return_intent_already_processed",
            RoutingNumberInvalid => "routing_number_invalid",
            SecretKeyRequired => "secret_key_required",
            SepaUnsupportedAccount => "sepa_unsupported_account",
            SetupAttemptFailed => "setup_attempt_failed",
            SetupIntentAuthenticationFailure => "setup_intent_authentication_failure",
            SetupIntentInvalidParameter => "setup_intent_invalid_parameter",
            SetupIntentMandateInvalid => "setup_intent_mandate_invalid",
            SetupIntentSetupAttemptExpired => "setup_intent_setup_attempt_expired",
            SetupIntentUnexpectedState => "setup_intent_unexpected_state",
            ShippingAddressInvalid => "shipping_address_invalid",
            ShippingCalculationFailed => "shipping_calculation_failed",
            SkuInactive => "sku_inactive",
            StateUnsupported => "state_unsupported",
            StatusTransitionInvalid => "status_transition_invalid",
            StripeTaxInactive => "stripe_tax_inactive",
            TaxIdInvalid => "tax_id_invalid",
            TaxesCalculationFailed => "taxes_calculation_failed",
            TerminalLocationCountryUnsupported => "terminal_location_country_unsupported",
            TerminalReaderBusy => "terminal_reader_busy",
            TerminalReaderHardwareFault => "terminal_reader_hardware_fault",
            TerminalReaderOffline => "terminal_reader_offline",
            TerminalReaderTimeout => "terminal_reader_timeout",
            TestmodeChargesOnly => "testmode_charges_only",
            TlsVersionUnsupported => "tls_version_unsupported",
            TokenAlreadyUsed => "token_already_used",
            TokenCardNetworkInvalid => "token_card_network_invalid",
            TokenInUse => "token_in_use",
            TransferSourceBalanceParametersMismatch => {
                "transfer_source_balance_parameters_mismatch"
            }
            TransfersNotAllowed => "transfers_not_allowed",
            UrlInvalid => "url_invalid",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for ApiErrorsCode {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ApiErrorsCode::*;
        match s {
            "account_closed" => Ok(AccountClosed),
            "account_country_invalid_address" => Ok(AccountCountryInvalidAddress),
            "account_error_country_change_requires_additional_steps" => {
                Ok(AccountErrorCountryChangeRequiresAdditionalSteps)
            }
            "account_information_mismatch" => Ok(AccountInformationMismatch),
            "account_invalid" => Ok(AccountInvalid),
            "account_number_invalid" => Ok(AccountNumberInvalid),
            "acss_debit_session_incomplete" => Ok(AcssDebitSessionIncomplete),
            "alipay_upgrade_required" => Ok(AlipayUpgradeRequired),
            "amount_too_large" => Ok(AmountTooLarge),
            "amount_too_small" => Ok(AmountTooSmall),
            "api_key_expired" => Ok(ApiKeyExpired),
            "application_fees_not_allowed" => Ok(ApplicationFeesNotAllowed),
            "authentication_required" => Ok(AuthenticationRequired),
            "balance_insufficient" => Ok(BalanceInsufficient),
            "balance_invalid_parameter" => Ok(BalanceInvalidParameter),
            "bank_account_bad_routing_numbers" => Ok(BankAccountBadRoutingNumbers),
            "bank_account_declined" => Ok(BankAccountDeclined),
            "bank_account_exists" => Ok(BankAccountExists),
            "bank_account_restricted" => Ok(BankAccountRestricted),
            "bank_account_unusable" => Ok(BankAccountUnusable),
            "bank_account_unverified" => Ok(BankAccountUnverified),
            "bank_account_verification_failed" => Ok(BankAccountVerificationFailed),
            "billing_invalid_mandate" => Ok(BillingInvalidMandate),
            "billing_policy_remote_function_response_invalid" => {
                Ok(BillingPolicyRemoteFunctionResponseInvalid)
            }
            "billing_policy_remote_function_timeout" => Ok(BillingPolicyRemoteFunctionTimeout),
            "billing_policy_remote_function_unexpected_status_code" => {
                Ok(BillingPolicyRemoteFunctionUnexpectedStatusCode)
            }
            "billing_policy_remote_function_unreachable" => {
                Ok(BillingPolicyRemoteFunctionUnreachable)
            }
            "bitcoin_upgrade_required" => Ok(BitcoinUpgradeRequired),
            "capture_charge_authorization_expired" => Ok(CaptureChargeAuthorizationExpired),
            "capture_unauthorized_payment" => Ok(CaptureUnauthorizedPayment),
            "card_decline_rate_limit_exceeded" => Ok(CardDeclineRateLimitExceeded),
            "card_declined" => Ok(CardDeclined),
            "cardholder_phone_number_required" => Ok(CardholderPhoneNumberRequired),
            "charge_already_captured" => Ok(ChargeAlreadyCaptured),
            "charge_already_refunded" => Ok(ChargeAlreadyRefunded),
            "charge_disputed" => Ok(ChargeDisputed),
            "charge_exceeds_source_limit" => Ok(ChargeExceedsSourceLimit),
            "charge_expired_for_capture" => Ok(ChargeExpiredForCapture),
            "charge_invalid_parameter" => Ok(ChargeInvalidParameter),
            "charge_not_refundable" => Ok(ChargeNotRefundable),
            "clearing_code_unsupported" => Ok(ClearingCodeUnsupported),
            "country_code_invalid" => Ok(CountryCodeInvalid),
            "country_unsupported" => Ok(CountryUnsupported),
            "coupon_expired" => Ok(CouponExpired),
            "customer_max_payment_methods" => Ok(CustomerMaxPaymentMethods),
            "customer_max_subscriptions" => Ok(CustomerMaxSubscriptions),
            "customer_tax_location_invalid" => Ok(CustomerTaxLocationInvalid),
            "debit_not_authorized" => Ok(DebitNotAuthorized),
            "email_invalid" => Ok(EmailInvalid),
            "expired_card" => Ok(ExpiredCard),
            "financial_connections_account_inactive" => Ok(FinancialConnectionsAccountInactive),
            "financial_connections_no_successful_transaction_refresh" => {
                Ok(FinancialConnectionsNoSuccessfulTransactionRefresh)
            }
            "forwarding_api_inactive" => Ok(ForwardingApiInactive),
            "forwarding_api_invalid_parameter" => Ok(ForwardingApiInvalidParameter),
            "forwarding_api_upstream_connection_error" => Ok(ForwardingApiUpstreamConnectionError),
            "forwarding_api_upstream_connection_timeout" => {
                Ok(ForwardingApiUpstreamConnectionTimeout)
            }
            "idempotency_key_in_use" => Ok(IdempotencyKeyInUse),
            "incorrect_address" => Ok(IncorrectAddress),
            "incorrect_cvc" => Ok(IncorrectCvc),
            "incorrect_number" => Ok(IncorrectNumber),
            "incorrect_zip" => Ok(IncorrectZip),
            "instant_payouts_config_disabled" => Ok(InstantPayoutsConfigDisabled),
            "instant_payouts_currency_disabled" => Ok(InstantPayoutsCurrencyDisabled),
            "instant_payouts_limit_exceeded" => Ok(InstantPayoutsLimitExceeded),
            "instant_payouts_unsupported" => Ok(InstantPayoutsUnsupported),
            "insufficient_funds" => Ok(InsufficientFunds),
            "intent_invalid_state" => Ok(IntentInvalidState),
            "intent_verification_method_missing" => Ok(IntentVerificationMethodMissing),
            "invalid_card_type" => Ok(InvalidCardType),
            "invalid_characters" => Ok(InvalidCharacters),
            "invalid_charge_amount" => Ok(InvalidChargeAmount),
            "invalid_cvc" => Ok(InvalidCvc),
            "invalid_expiry_month" => Ok(InvalidExpiryMonth),
            "invalid_expiry_year" => Ok(InvalidExpiryYear),
            "invalid_number" => Ok(InvalidNumber),
            "invalid_source_usage" => Ok(InvalidSourceUsage),
            "invalid_tax_location" => Ok(InvalidTaxLocation),
            "invoice_no_customer_line_items" => Ok(InvoiceNoCustomerLineItems),
            "invoice_no_payment_method_types" => Ok(InvoiceNoPaymentMethodTypes),
            "invoice_no_subscription_line_items" => Ok(InvoiceNoSubscriptionLineItems),
            "invoice_not_editable" => Ok(InvoiceNotEditable),
            "invoice_on_behalf_of_not_editable" => Ok(InvoiceOnBehalfOfNotEditable),
            "invoice_payment_intent_requires_action" => Ok(InvoicePaymentIntentRequiresAction),
            "invoice_upcoming_none" => Ok(InvoiceUpcomingNone),
            "livemode_mismatch" => Ok(LivemodeMismatch),
            "lock_timeout" => Ok(LockTimeout),
            "missing" => Ok(Missing),
            "no_account" => Ok(NoAccount),
            "not_allowed_on_standard_account" => Ok(NotAllowedOnStandardAccount),
            "out_of_inventory" => Ok(OutOfInventory),
            "ownership_declaration_not_allowed" => Ok(OwnershipDeclarationNotAllowed),
            "parameter_invalid_empty" => Ok(ParameterInvalidEmpty),
            "parameter_invalid_integer" => Ok(ParameterInvalidInteger),
            "parameter_invalid_string_blank" => Ok(ParameterInvalidStringBlank),
            "parameter_invalid_string_empty" => Ok(ParameterInvalidStringEmpty),
            "parameter_missing" => Ok(ParameterMissing),
            "parameter_unknown" => Ok(ParameterUnknown),
            "parameters_exclusive" => Ok(ParametersExclusive),
            "payment_intent_action_required" => Ok(PaymentIntentActionRequired),
            "payment_intent_authentication_failure" => Ok(PaymentIntentAuthenticationFailure),
            "payment_intent_incompatible_payment_method" => {
                Ok(PaymentIntentIncompatiblePaymentMethod)
            }
            "payment_intent_invalid_parameter" => Ok(PaymentIntentInvalidParameter),
            "payment_intent_konbini_rejected_confirmation_number" => {
                Ok(PaymentIntentKonbiniRejectedConfirmationNumber)
            }
            "payment_intent_mandate_invalid" => Ok(PaymentIntentMandateInvalid),
            "payment_intent_payment_attempt_expired" => Ok(PaymentIntentPaymentAttemptExpired),
            "payment_intent_payment_attempt_failed" => Ok(PaymentIntentPaymentAttemptFailed),
            "payment_intent_unexpected_state" => Ok(PaymentIntentUnexpectedState),
            "payment_method_bank_account_already_verified" => {
                Ok(PaymentMethodBankAccountAlreadyVerified)
            }
            "payment_method_bank_account_blocked" => Ok(PaymentMethodBankAccountBlocked),
            "payment_method_billing_details_address_missing" => {
                Ok(PaymentMethodBillingDetailsAddressMissing)
            }
            "payment_method_configuration_failures" => Ok(PaymentMethodConfigurationFailures),
            "payment_method_currency_mismatch" => Ok(PaymentMethodCurrencyMismatch),
            "payment_method_customer_decline" => Ok(PaymentMethodCustomerDecline),
            "payment_method_invalid_parameter" => Ok(PaymentMethodInvalidParameter),
            "payment_method_invalid_parameter_testmode" => {
                Ok(PaymentMethodInvalidParameterTestmode)
            }
            "payment_method_microdeposit_failed" => Ok(PaymentMethodMicrodepositFailed),
            "payment_method_microdeposit_verification_amounts_invalid" => {
                Ok(PaymentMethodMicrodepositVerificationAmountsInvalid)
            }
            "payment_method_microdeposit_verification_amounts_mismatch" => {
                Ok(PaymentMethodMicrodepositVerificationAmountsMismatch)
            }
            "payment_method_microdeposit_verification_attempts_exceeded" => {
                Ok(PaymentMethodMicrodepositVerificationAttemptsExceeded)
            }
            "payment_method_microdeposit_verification_descriptor_code_mismatch" => {
                Ok(PaymentMethodMicrodepositVerificationDescriptorCodeMismatch)
            }
            "payment_method_microdeposit_verification_timeout" => {
                Ok(PaymentMethodMicrodepositVerificationTimeout)
            }
            "payment_method_not_available" => Ok(PaymentMethodNotAvailable),
            "payment_method_provider_decline" => Ok(PaymentMethodProviderDecline),
            "payment_method_provider_timeout" => Ok(PaymentMethodProviderTimeout),
            "payment_method_unactivated" => Ok(PaymentMethodUnactivated),
            "payment_method_unexpected_state" => Ok(PaymentMethodUnexpectedState),
            "payment_method_unsupported_type" => Ok(PaymentMethodUnsupportedType),
            "payout_reconciliation_not_ready" => Ok(PayoutReconciliationNotReady),
            "payouts_limit_exceeded" => Ok(PayoutsLimitExceeded),
            "payouts_not_allowed" => Ok(PayoutsNotAllowed),
            "platform_account_required" => Ok(PlatformAccountRequired),
            "platform_api_key_expired" => Ok(PlatformApiKeyExpired),
            "postal_code_invalid" => Ok(PostalCodeInvalid),
            "processing_error" => Ok(ProcessingError),
            "product_inactive" => Ok(ProductInactive),
            "progressive_onboarding_limit_exceeded" => Ok(ProgressiveOnboardingLimitExceeded),
            "rate_limit" => Ok(RateLimit),
            "refer_to_customer" => Ok(ReferToCustomer),
            "refund_disputed_payment" => Ok(RefundDisputedPayment),
            "resource_already_exists" => Ok(ResourceAlreadyExists),
            "resource_missing" => Ok(ResourceMissing),
            "return_intent_already_processed" => Ok(ReturnIntentAlreadyProcessed),
            "routing_number_invalid" => Ok(RoutingNumberInvalid),
            "secret_key_required" => Ok(SecretKeyRequired),
            "sepa_unsupported_account" => Ok(SepaUnsupportedAccount),
            "setup_attempt_failed" => Ok(SetupAttemptFailed),
            "setup_intent_authentication_failure" => Ok(SetupIntentAuthenticationFailure),
            "setup_intent_invalid_parameter" => Ok(SetupIntentInvalidParameter),
            "setup_intent_mandate_invalid" => Ok(SetupIntentMandateInvalid),
            "setup_intent_setup_attempt_expired" => Ok(SetupIntentSetupAttemptExpired),
            "setup_intent_unexpected_state" => Ok(SetupIntentUnexpectedState),
            "shipping_address_invalid" => Ok(ShippingAddressInvalid),
            "shipping_calculation_failed" => Ok(ShippingCalculationFailed),
            "sku_inactive" => Ok(SkuInactive),
            "state_unsupported" => Ok(StateUnsupported),
            "status_transition_invalid" => Ok(StatusTransitionInvalid),
            "stripe_tax_inactive" => Ok(StripeTaxInactive),
            "tax_id_invalid" => Ok(TaxIdInvalid),
            "taxes_calculation_failed" => Ok(TaxesCalculationFailed),
            "terminal_location_country_unsupported" => Ok(TerminalLocationCountryUnsupported),
            "terminal_reader_busy" => Ok(TerminalReaderBusy),
            "terminal_reader_hardware_fault" => Ok(TerminalReaderHardwareFault),
            "terminal_reader_offline" => Ok(TerminalReaderOffline),
            "terminal_reader_timeout" => Ok(TerminalReaderTimeout),
            "testmode_charges_only" => Ok(TestmodeChargesOnly),
            "tls_version_unsupported" => Ok(TlsVersionUnsupported),
            "token_already_used" => Ok(TokenAlreadyUsed),
            "token_card_network_invalid" => Ok(TokenCardNetworkInvalid),
            "token_in_use" => Ok(TokenInUse),
            "transfer_source_balance_parameters_mismatch" => {
                Ok(TransferSourceBalanceParametersMismatch)
            }
            "transfers_not_allowed" => Ok(TransfersNotAllowed),
            "url_invalid" => Ok(UrlInvalid),
            _ => Ok(Self::Unknown),
        }
    }
}
impl std::fmt::Display for ApiErrorsCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ApiErrorsCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ApiErrorsCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ApiErrorsCode {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ApiErrorsCode> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ApiErrorsCode::from_str(s).unwrap());
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(ApiErrorsCode);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ApiErrorsCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// The type of error returned.
/// One of `api_error`, `card_error`, `idempotency_error`, or `invalid_request_error`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ApiErrorsType {
    ApiError,
    CardError,
    IdempotencyError,
    InvalidRequestError,
}
impl ApiErrorsType {
    pub fn as_str(self) -> &'static str {
        use ApiErrorsType::*;
        match self {
            ApiError => "api_error",
            CardError => "card_error",
            IdempotencyError => "idempotency_error",
            InvalidRequestError => "invalid_request_error",
        }
    }
}

impl std::str::FromStr for ApiErrorsType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ApiErrorsType::*;
        match s {
            "api_error" => Ok(ApiError),
            "card_error" => Ok(CardError),
            "idempotency_error" => Ok(IdempotencyError),
            "invalid_request_error" => Ok(InvalidRequestError),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ApiErrorsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ApiErrorsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ApiErrorsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ApiErrorsType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ApiErrorsType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ApiErrorsType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(ApiErrorsType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ApiErrorsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ApiErrorsType"))
    }
}

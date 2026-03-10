use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct CreateTerminalOnboardingLinkBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    link_options: CreateTerminalOnboardingLinkLinkOptions,
    link_type: stripe_terminal::TerminalOnboardingLinkLinkType,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_behalf_of: Option<String>,
}
impl CreateTerminalOnboardingLinkBuilder {
    fn new(
        link_options: impl Into<CreateTerminalOnboardingLinkLinkOptions>,
        link_type: impl Into<stripe_terminal::TerminalOnboardingLinkLinkType>,
    ) -> Self {
        Self {
            expand: None,
            link_options: link_options.into(),
            link_type: link_type.into(),
            on_behalf_of: None,
        }
    }
}
/// Specific fields needed to generate the desired link type.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateTerminalOnboardingLinkLinkOptions {
    /// The options associated with the Apple Terms and Conditions link type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_terms_and_conditions:
        Option<CreateTerminalOnboardingLinkLinkOptionsAppleTermsAndConditions>,
}
impl CreateTerminalOnboardingLinkLinkOptions {
    pub fn new() -> Self {
        Self { apple_terms_and_conditions: None }
    }
}
impl Default for CreateTerminalOnboardingLinkLinkOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// The options associated with the Apple Terms and Conditions link type.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateTerminalOnboardingLinkLinkOptionsAppleTermsAndConditions {
    /// Whether the link should also support users relinking their Apple account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_relinking: Option<bool>,
    /// The business name of the merchant accepting Apple's Terms and Conditions.
    pub merchant_display_name: String,
}
impl CreateTerminalOnboardingLinkLinkOptionsAppleTermsAndConditions {
    pub fn new(merchant_display_name: impl Into<String>) -> Self {
        Self { allow_relinking: None, merchant_display_name: merchant_display_name.into() }
    }
}
/// Creates a new `OnboardingLink` object that contains a redirect_url used for onboarding onto Tap to Pay on iPhone.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTerminalOnboardingLink {
    inner: CreateTerminalOnboardingLinkBuilder,
}
impl CreateTerminalOnboardingLink {
    /// Construct a new `CreateTerminalOnboardingLink`.
    pub fn new(
        link_options: impl Into<CreateTerminalOnboardingLinkLinkOptions>,
        link_type: impl Into<stripe_terminal::TerminalOnboardingLinkLinkType>,
    ) -> Self {
        Self {
            inner: CreateTerminalOnboardingLinkBuilder::new(link_options.into(), link_type.into()),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Stripe account ID to generate the link for.
    pub fn on_behalf_of(mut self, on_behalf_of: impl Into<String>) -> Self {
        self.inner.on_behalf_of = Some(on_behalf_of.into());
        self
    }
}
impl CreateTerminalOnboardingLink {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for CreateTerminalOnboardingLink {
    type Output = stripe_terminal::TerminalOnboardingLink;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/terminal/onboarding_links").form(&self.inner)
    }
}

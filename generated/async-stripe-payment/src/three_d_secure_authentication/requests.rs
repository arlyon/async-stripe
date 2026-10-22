use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct ListThreeDSecureAuthenticationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<stripe_payment::ThreeDSecureAuthenticationStatus>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListThreeDSecureAuthenticationBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListThreeDSecureAuthenticationBuilder").finish_non_exhaustive()
    }
}
impl ListThreeDSecureAuthenticationBuilder {
    fn new() -> Self {
        Self {
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            status: None,
        }
    }
}
/// Returns a list of 3D Secure Authentications.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ListThreeDSecureAuthentication {
    inner: ListThreeDSecureAuthenticationBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListThreeDSecureAuthentication {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListThreeDSecureAuthentication").finish_non_exhaustive()
    }
}
impl ListThreeDSecureAuthentication {
    /// Construct a new `ListThreeDSecureAuthentication`.
    pub fn new() -> Self {
        Self { inner: ListThreeDSecureAuthenticationBuilder::new() }
    }
    /// A filter on the list, based on the object `created` field.
    /// The value can be a string with an integer Unix timestamp or a dictionary with a number of different query options.
    pub fn created(mut self, created: impl Into<stripe_types::RangeQueryTs>) -> Self {
        self.inner.created = Some(created.into());
        self
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: impl Into<String>) -> Self {
        self.inner.ending_before = Some(ending_before.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: impl Into<i64>) -> Self {
        self.inner.limit = Some(limit.into());
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
    /// Only return 3D Secure Authentications for specified status.
    pub fn status(
        mut self,
        status: impl Into<stripe_payment::ThreeDSecureAuthenticationStatus>,
    ) -> Self {
        self.inner.status = Some(status.into());
        self
    }
}
impl Default for ListThreeDSecureAuthentication {
    fn default() -> Self {
        Self::new()
    }
}
impl ListThreeDSecureAuthentication {
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

    pub fn paginate(
        &self,
    ) -> stripe_client_core::ListPaginator<
        stripe_types::List<stripe_payment::ThreeDSecureAuthentication>,
    > {
        stripe_client_core::ListPaginator::new_list("/three_d_secure/authentications", &self.inner)
    }
}

impl StripeRequest for ListThreeDSecureAuthentication {
    type Output = stripe_types::List<stripe_payment::ThreeDSecureAuthentication>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/three_d_secure/authentications").query(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct RetrieveThreeDSecureAuthenticationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveThreeDSecureAuthenticationBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveThreeDSecureAuthenticationBuilder").finish_non_exhaustive()
    }
}
impl RetrieveThreeDSecureAuthenticationBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// This endpoint retrieves a 3DS Authentication.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RetrieveThreeDSecureAuthentication {
    inner: RetrieveThreeDSecureAuthenticationBuilder,
    authentication: stripe_payment::ThreeDSecureAuthenticationId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveThreeDSecureAuthentication {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveThreeDSecureAuthentication").finish_non_exhaustive()
    }
}
impl RetrieveThreeDSecureAuthentication {
    /// Construct a new `RetrieveThreeDSecureAuthentication`.
    pub fn new(authentication: impl Into<stripe_payment::ThreeDSecureAuthenticationId>) -> Self {
        Self {
            authentication: authentication.into(),
            inner: RetrieveThreeDSecureAuthenticationBuilder::new(),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveThreeDSecureAuthentication {
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

impl StripeRequest for RetrieveThreeDSecureAuthentication {
    type Output = stripe_payment::ThreeDSecureAuthentication;

    fn build(&self) -> RequestBuilder {
        let authentication = &self.authentication;
        RequestBuilder::new(
            StripeMethod::Get,
            format!("/three_d_secure/authentications/{authentication}"),
        )
        .query(&self.inner)
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct CreateThreeDSecureAuthenticationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    acquirer_details: Option<CreateThreeDSecureAuthenticationAcquirerDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<i64>,
    channel: CreateThreeDSecureAuthenticationChannel,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<stripe_types::Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    directory_server: Option<stripe_payment::ThreeDSecureAuthenticationDirectoryServer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flow_preference: Option<CreateThreeDSecureAuthenticationFlowPreference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    future_usage: Option<CreateThreeDSecureAuthenticationFutureUsage>,
    message_category: stripe_payment::ThreeDSecureAuthenticationMessageCategory,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method_data: Option<CreateThreeDSecureAuthenticationPaymentMethodData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<stripe_payment::ThreeDSecureAuthenticationReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    submit: Option<CreateThreeDSecureAuthenticationSubmit>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateThreeDSecureAuthenticationBuilder").finish_non_exhaustive()
    }
}
impl CreateThreeDSecureAuthenticationBuilder {
    fn new(
        channel: impl Into<CreateThreeDSecureAuthenticationChannel>,
        message_category: impl Into<stripe_payment::ThreeDSecureAuthenticationMessageCategory>,
    ) -> Self {
        Self {
            acquirer_details: None,
            amount: None,
            channel: channel.into(),
            currency: None,
            directory_server: None,
            expand: None,
            flow_preference: None,
            future_usage: None,
            message_category: message_category.into(),
            metadata: None,
            payment_method: None,
            payment_method_data: None,
            reason: None,
            shipping_address: None,
            submit: None,
        }
    }
}
/// Contains additional details about the acquirer for this 3DS Authentication.
///
/// Refer to the [Pass acquirer details and directory server section of the standalone 3DS guide](/payments/3d-secure/standalone-3d-secure#pass-acquirer-details-and-directory-server) for more information.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateThreeDSecureAuthenticationAcquirerDetails {
    /// The Acquirer BIN (specific to the directory_server).
    pub acquirer_bin: String,
    /// The two-letter country code of the acquirer ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub acquirer_country: String,
    /// The Merchant ID (or Card Acceptor ID) that your acquirer assigned you (specific to the directory_server).
    pub acquirer_merchant_id: String,
    /// The [merchant category code](https://en.wikipedia.org/wiki/Merchant_category_code) as defined by each payment system or directory server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcc: Option<String>,
    /// The merchant name assigned by the acquirer or payment system.
    /// Same name used in the authorization message as defined in [ISO 8583](https://en.wikipedia.org/wiki/ISO_8583).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_name: Option<String>,
    /// Requestor ID if you’re enrolled in the card network’s 3DS program.
    /// Otherwise, you can omit this field because Stripe assigns a Requestor ID with the card networks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requestor_id: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationAcquirerDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateThreeDSecureAuthenticationAcquirerDetails").finish_non_exhaustive()
    }
}
impl CreateThreeDSecureAuthenticationAcquirerDetails {
    pub fn new(
        acquirer_bin: impl Into<String>,
        acquirer_country: impl Into<String>,
        acquirer_merchant_id: impl Into<String>,
    ) -> Self {
        Self {
            acquirer_bin: acquirer_bin.into(),
            acquirer_country: acquirer_country.into(),
            acquirer_merchant_id: acquirer_merchant_id.into(),
            mcc: None,
            merchant_name: None,
            requestor_id: None,
        }
    }
}
/// Contains additional details on the channel used for this 3DS Authentication.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateThreeDSecureAuthenticationChannel {
    /// Contains additional details about the browser details you collected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser: Option<CreateThreeDSecureAuthenticationChannelBrowser>,
    /// Contains additional details about the 3DS Requestor Initiated (3RI) channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_r_i: Option<CreateThreeDSecureAuthenticationChannelThreeRI>,
    /// Type of channel you would prefer to use for this 3DS Authentication.
    #[serde(rename = "type")]
    pub type_: CreateThreeDSecureAuthenticationChannelType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateThreeDSecureAuthenticationChannel").finish_non_exhaustive()
    }
}
impl CreateThreeDSecureAuthenticationChannel {
    pub fn new(type_: impl Into<CreateThreeDSecureAuthenticationChannelType>) -> Self {
        Self { browser: None, three_r_i: None, type_: type_.into() }
    }
}
/// Contains additional details about the browser details you collected.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateThreeDSecureAuthenticationChannelBrowser {
    /// The HTTP accept headers from the cardholder's browser. Collected server-side.
    pub accept_header: String,
    /// The color depth of the cardholder’s screen.
    ///
    /// Returned from the `screen.colorDepth` property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_depth: Option<i64>,
    /// Unique and immutable identifier linked to a device that is consistent across 3DS transactions for the specific user device.
    /// For example: hardware device ID or a platform-calculated device fingerprint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    /// The IP address of the browser.
    /// Included in the HTTP request to your server before you create the 3DS Authentication.
    ///
    /// Collected server-side.
    pub ip_address: String,
    /// The cardholder browser’s ability to execute Java.
    /// Returned from the navigator.javaEnabled property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub java_enabled: Option<bool>,
    /// The cardholder browser’s ability to execute JavaScript.
    pub javascript_enabled: bool,
    /// An IETF BCP 47 language tag representing the browser language.
    /// Typically returned from the `navigator.language` property, but might also be returned from `navigator.languages` or `navigator.browserLanguage`.
    ///
    /// In some cases, this value might be an array.
    /// To cast it to a string or null value, you can use the `getBrowserLanguage()` [example function](/payments/3d-secure/standalone-3d-secure#pass-client-side-collected-channel-information).
    pub language: String,
    /// The total height of the cardholder’s screen in pixels.
    ///
    /// Returned from the `screen.height` property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_height: Option<i64>,
    /// The total width of the cardholder’s screen in pixels.
    ///
    /// Returned from the `screen.width` property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_width: Option<i64>,
    /// The time difference between UTC time and the local time of the cardholder’s browser, in minutes.
    ///
    /// Returned by `new Date().getTimezoneOffset()`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone_offset: Option<i64>,
    /// The browser user agent.
    /// You can retrieve this value on the client side using the `navigator.userAgent` property, or in the HTTP request to your server before you create the 3DS Authentication.
    pub user_agent: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationChannelBrowser {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateThreeDSecureAuthenticationChannelBrowser").finish_non_exhaustive()
    }
}
impl CreateThreeDSecureAuthenticationChannelBrowser {
    pub fn new(
        accept_header: impl Into<String>,
        ip_address: impl Into<String>,
        javascript_enabled: impl Into<bool>,
        language: impl Into<String>,
        user_agent: impl Into<String>,
    ) -> Self {
        Self {
            accept_header: accept_header.into(),
            color_depth: None,
            device_id: None,
            ip_address: ip_address.into(),
            java_enabled: None,
            javascript_enabled: javascript_enabled.into(),
            language: language.into(),
            screen_height: None,
            screen_width: None,
            timezone_offset: None,
            user_agent: user_agent.into(),
        }
    }
}
/// Contains additional details about the 3DS Requestor Initiated (3RI) channel.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateThreeDSecureAuthenticationChannelThreeRI {
    /// ID of a prior `Authentication`.
    /// For example, the first recurring transaction that was authenticated by the cardholder.
    pub previous_authentication: String,
    /// It provides additional information to the ACS to determine the best approach for handling a 3RI request.
    #[serde(rename = "type")]
    pub type_: CreateThreeDSecureAuthenticationChannelThreeRiType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationChannelThreeRI {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateThreeDSecureAuthenticationChannelThreeRI").finish_non_exhaustive()
    }
}
impl CreateThreeDSecureAuthenticationChannelThreeRI {
    pub fn new(
        previous_authentication: impl Into<String>,
        type_: impl Into<CreateThreeDSecureAuthenticationChannelThreeRiType>,
    ) -> Self {
        Self { previous_authentication: previous_authentication.into(), type_: type_.into() }
    }
}
/// It provides additional information to the ACS to determine the best approach for handling a 3RI request.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateThreeDSecureAuthenticationChannelThreeRiType {
    DelayedShipment,
    OtherPayment,
    Recurring,
    SplitShipment,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateThreeDSecureAuthenticationChannelThreeRiType {
    pub fn as_str(&self) -> &str {
        use CreateThreeDSecureAuthenticationChannelThreeRiType::*;
        match self {
            DelayedShipment => "delayed_shipment",
            OtherPayment => "other_payment",
            Recurring => "recurring",
            SplitShipment => "split_shipment",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateThreeDSecureAuthenticationChannelThreeRiType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateThreeDSecureAuthenticationChannelThreeRiType::*;
        match s {
            "delayed_shipment" => Ok(DelayedShipment),
            "other_payment" => Ok(OtherPayment),
            "recurring" => Ok(Recurring),
            "split_shipment" => Ok(SplitShipment),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateThreeDSecureAuthenticationChannelThreeRiType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateThreeDSecureAuthenticationChannelThreeRiType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationChannelThreeRiType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationChannelThreeRiType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateThreeDSecureAuthenticationChannelThreeRiType))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateThreeDSecureAuthenticationChannelThreeRiType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateThreeDSecureAuthenticationChannelThreeRiType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Type of channel you would prefer to use for this 3DS Authentication.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateThreeDSecureAuthenticationChannelType {
    Browser,
    ThreeRI,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateThreeDSecureAuthenticationChannelType {
    pub fn as_str(&self) -> &str {
        use CreateThreeDSecureAuthenticationChannelType::*;
        match self {
            Browser => "browser",
            ThreeRI => "three_r_i",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateThreeDSecureAuthenticationChannelType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateThreeDSecureAuthenticationChannelType::*;
        match s {
            "browser" => Ok(Browser),
            "three_r_i" => Ok(ThreeRI),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateThreeDSecureAuthenticationChannelType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateThreeDSecureAuthenticationChannelType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationChannelType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationChannelType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateThreeDSecureAuthenticationChannelType))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateThreeDSecureAuthenticationChannelType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateThreeDSecureAuthenticationChannelType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Contains additional details on your flow preference for this 3DS Authentication.
///
/// Refer to the [Specify a flow preference section of the standalone 3DS guide](/payments/3d-secure/standalone-3d-secure#specify-a-flow-preference) for more information.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateThreeDSecureAuthenticationFlowPreference {
    /// Contains additional details about your challenge flow preference for this 3DS Authentication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge: Option<CreateThreeDSecureAuthenticationFlowPreferenceChallenge>,
    /// Contains additional details about your data share only flow preference for this 3DS Authentication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_share: Option<CreateThreeDSecureAuthenticationFlowPreferenceDataShare>,
    /// Contains additional details about your frictionless flow preference for this 3DS Authentication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frictionless: Option<CreateThreeDSecureAuthenticationFlowPreferenceFrictionless>,
    /// Type of flow you requested for this 3DS Authentication.
    #[serde(rename = "type")]
    pub type_: CreateThreeDSecureAuthenticationFlowPreferenceType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationFlowPreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateThreeDSecureAuthenticationFlowPreference").finish_non_exhaustive()
    }
}
impl CreateThreeDSecureAuthenticationFlowPreference {
    pub fn new(type_: impl Into<CreateThreeDSecureAuthenticationFlowPreferenceType>) -> Self {
        Self { challenge: None, data_share: None, frictionless: None, type_: type_.into() }
    }
}
/// Contains additional details about your challenge flow preference for this 3DS Authentication.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateThreeDSecureAuthenticationFlowPreferenceChallenge {
    /// Type of challenge flow you requested for this 3DS Authentication.
    #[serde(rename = "type")]
    pub type_: CreateThreeDSecureAuthenticationFlowPreferenceChallengeType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationFlowPreferenceChallenge {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateThreeDSecureAuthenticationFlowPreferenceChallenge")
            .finish_non_exhaustive()
    }
}
impl CreateThreeDSecureAuthenticationFlowPreferenceChallenge {
    pub fn new(
        type_: impl Into<CreateThreeDSecureAuthenticationFlowPreferenceChallengeType>,
    ) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of challenge flow you requested for this 3DS Authentication.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateThreeDSecureAuthenticationFlowPreferenceChallengeType {
    Mandated,
    Preferred,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateThreeDSecureAuthenticationFlowPreferenceChallengeType {
    pub fn as_str(&self) -> &str {
        use CreateThreeDSecureAuthenticationFlowPreferenceChallengeType::*;
        match self {
            Mandated => "mandated",
            Preferred => "preferred",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateThreeDSecureAuthenticationFlowPreferenceChallengeType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateThreeDSecureAuthenticationFlowPreferenceChallengeType::*;
        match s {
            "mandated" => Ok(Mandated),
            "preferred" => Ok(Preferred),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateThreeDSecureAuthenticationFlowPreferenceChallengeType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateThreeDSecureAuthenticationFlowPreferenceChallengeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationFlowPreferenceChallengeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationFlowPreferenceChallengeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateThreeDSecureAuthenticationFlowPreferenceChallengeType))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateThreeDSecureAuthenticationFlowPreferenceChallengeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateThreeDSecureAuthenticationFlowPreferenceChallengeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Contains additional details about your data share only flow preference for this 3DS Authentication.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateThreeDSecureAuthenticationFlowPreferenceDataShare {
    /// Type of data share only flow you requested for this 3DS Authentication.
    #[serde(rename = "type")]
    pub type_: CreateThreeDSecureAuthenticationFlowPreferenceDataShareType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationFlowPreferenceDataShare {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateThreeDSecureAuthenticationFlowPreferenceDataShare")
            .finish_non_exhaustive()
    }
}
impl CreateThreeDSecureAuthenticationFlowPreferenceDataShare {
    pub fn new(
        type_: impl Into<CreateThreeDSecureAuthenticationFlowPreferenceDataShareType>,
    ) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of data share only flow you requested for this 3DS Authentication.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateThreeDSecureAuthenticationFlowPreferenceDataShareType {
    DsSpecific,
    EmvStandard,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateThreeDSecureAuthenticationFlowPreferenceDataShareType {
    pub fn as_str(&self) -> &str {
        use CreateThreeDSecureAuthenticationFlowPreferenceDataShareType::*;
        match self {
            DsSpecific => "ds_specific",
            EmvStandard => "emv_standard",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateThreeDSecureAuthenticationFlowPreferenceDataShareType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateThreeDSecureAuthenticationFlowPreferenceDataShareType::*;
        match s {
            "ds_specific" => Ok(DsSpecific),
            "emv_standard" => Ok(EmvStandard),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateThreeDSecureAuthenticationFlowPreferenceDataShareType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateThreeDSecureAuthenticationFlowPreferenceDataShareType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationFlowPreferenceDataShareType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationFlowPreferenceDataShareType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateThreeDSecureAuthenticationFlowPreferenceDataShareType))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateThreeDSecureAuthenticationFlowPreferenceDataShareType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateThreeDSecureAuthenticationFlowPreferenceDataShareType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Contains additional details about your frictionless flow preference for this 3DS Authentication.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateThreeDSecureAuthenticationFlowPreferenceFrictionless {
    /// Type of frictionless flow you requested for this 3DS Authentication.
    #[serde(rename = "type")]
    pub type_: CreateThreeDSecureAuthenticationFlowPreferenceFrictionlessType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationFlowPreferenceFrictionless {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateThreeDSecureAuthenticationFlowPreferenceFrictionless")
            .finish_non_exhaustive()
    }
}
impl CreateThreeDSecureAuthenticationFlowPreferenceFrictionless {
    pub fn new(
        type_: impl Into<CreateThreeDSecureAuthenticationFlowPreferenceFrictionlessType>,
    ) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of frictionless flow you requested for this 3DS Authentication.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateThreeDSecureAuthenticationFlowPreferenceFrictionlessType {
    LowRisk,
    None,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateThreeDSecureAuthenticationFlowPreferenceFrictionlessType {
    pub fn as_str(&self) -> &str {
        use CreateThreeDSecureAuthenticationFlowPreferenceFrictionlessType::*;
        match self {
            LowRisk => "low_risk",
            None => "none",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateThreeDSecureAuthenticationFlowPreferenceFrictionlessType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateThreeDSecureAuthenticationFlowPreferenceFrictionlessType::*;
        match s {
            "low_risk" => Ok(LowRisk),
            "none" => Ok(None),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateThreeDSecureAuthenticationFlowPreferenceFrictionlessType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateThreeDSecureAuthenticationFlowPreferenceFrictionlessType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationFlowPreferenceFrictionlessType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationFlowPreferenceFrictionlessType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateThreeDSecureAuthenticationFlowPreferenceFrictionlessType))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateThreeDSecureAuthenticationFlowPreferenceFrictionlessType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateThreeDSecureAuthenticationFlowPreferenceFrictionlessType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Type of flow you requested for this 3DS Authentication.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateThreeDSecureAuthenticationFlowPreferenceType {
    Challenge,
    DataShare,
    Frictionless,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateThreeDSecureAuthenticationFlowPreferenceType {
    pub fn as_str(&self) -> &str {
        use CreateThreeDSecureAuthenticationFlowPreferenceType::*;
        match self {
            Challenge => "challenge",
            DataShare => "data_share",
            Frictionless => "frictionless",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateThreeDSecureAuthenticationFlowPreferenceType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateThreeDSecureAuthenticationFlowPreferenceType::*;
        match s {
            "challenge" => Ok(Challenge),
            "data_share" => Ok(DataShare),
            "frictionless" => Ok(Frictionless),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateThreeDSecureAuthenticationFlowPreferenceType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateThreeDSecureAuthenticationFlowPreferenceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationFlowPreferenceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationFlowPreferenceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateThreeDSecureAuthenticationFlowPreferenceType))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateThreeDSecureAuthenticationFlowPreferenceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateThreeDSecureAuthenticationFlowPreferenceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Contains information about future usage of this 3DS Authentication
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateThreeDSecureAuthenticationFutureUsage {
    /// Parameters related to an installment payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installment: Option<CreateThreeDSecureAuthenticationFutureUsageInstallment>,
    /// Parameters related to a recurring payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<CreateThreeDSecureAuthenticationFutureUsageRecurring>,
    /// The type of future usage declared for this 3DS Authentication
    #[serde(rename = "type")]
    pub type_: CreateThreeDSecureAuthenticationFutureUsageType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateThreeDSecureAuthenticationFutureUsage").finish_non_exhaustive()
    }
}
impl CreateThreeDSecureAuthenticationFutureUsage {
    pub fn new(type_: impl Into<CreateThreeDSecureAuthenticationFutureUsageType>) -> Self {
        Self { installment: None, recurring: None, type_: type_.into() }
    }
}
/// Parameters related to an installment payment.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateThreeDSecureAuthenticationFutureUsageInstallment {
    /// A non-negative integer representing the future authorizations' amount in the [smallest currency unit](/currencies#zero-decimal).
    pub amount: i64,
    /// Information about the expiry of the future usage of this authentication.
    pub expiry: CreateThreeDSecureAuthenticationFutureUsageInstallmentExpiry,
    /// The unit of time for `interval_count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<CreateThreeDSecureAuthenticationFutureUsageInstallmentInterval>,
    /// The minimum number of time intervals between authorizations.
    /// Must be greater than 0, and defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
    /// The maximum number of installments. Must be greater than 1.
    pub number: i64,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationFutureUsageInstallment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateThreeDSecureAuthenticationFutureUsageInstallment")
            .finish_non_exhaustive()
    }
}
impl CreateThreeDSecureAuthenticationFutureUsageInstallment {
    pub fn new(
        amount: impl Into<i64>,
        expiry: impl Into<CreateThreeDSecureAuthenticationFutureUsageInstallmentExpiry>,
        number: impl Into<i64>,
    ) -> Self {
        Self {
            amount: amount.into(),
            expiry: expiry.into(),
            interval: None,
            interval_count: None,
            number: number.into(),
        }
    }
}
/// Information about the expiry of the future usage of this authentication.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateThreeDSecureAuthenticationFutureUsageInstallmentExpiry {
    /// The date before which the last authorization related to this authentication will occur.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// The type of expiry for the future use of this authentication.
    #[serde(rename = "type")]
    pub type_: CreateThreeDSecureAuthenticationFutureUsageInstallmentExpiryType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationFutureUsageInstallmentExpiry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateThreeDSecureAuthenticationFutureUsageInstallmentExpiry")
            .finish_non_exhaustive()
    }
}
impl CreateThreeDSecureAuthenticationFutureUsageInstallmentExpiry {
    pub fn new(
        type_: impl Into<CreateThreeDSecureAuthenticationFutureUsageInstallmentExpiryType>,
    ) -> Self {
        Self { date: None, type_: type_.into() }
    }
}
/// The type of expiry for the future use of this authentication.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateThreeDSecureAuthenticationFutureUsageInstallmentExpiryType {
    Date,
    Never,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateThreeDSecureAuthenticationFutureUsageInstallmentExpiryType {
    pub fn as_str(&self) -> &str {
        use CreateThreeDSecureAuthenticationFutureUsageInstallmentExpiryType::*;
        match self {
            Date => "date",
            Never => "never",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateThreeDSecureAuthenticationFutureUsageInstallmentExpiryType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateThreeDSecureAuthenticationFutureUsageInstallmentExpiryType::*;
        match s {
            "date" => Ok(Date),
            "never" => Ok(Never),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateThreeDSecureAuthenticationFutureUsageInstallmentExpiryType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateThreeDSecureAuthenticationFutureUsageInstallmentExpiryType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationFutureUsageInstallmentExpiryType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationFutureUsageInstallmentExpiryType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateThreeDSecureAuthenticationFutureUsageInstallmentExpiryType))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateThreeDSecureAuthenticationFutureUsageInstallmentExpiryType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateThreeDSecureAuthenticationFutureUsageInstallmentExpiryType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The unit of time for `interval_count`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateThreeDSecureAuthenticationFutureUsageInstallmentInterval {
    Day,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateThreeDSecureAuthenticationFutureUsageInstallmentInterval {
    pub fn as_str(&self) -> &str {
        use CreateThreeDSecureAuthenticationFutureUsageInstallmentInterval::*;
        match self {
            Day => "day",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateThreeDSecureAuthenticationFutureUsageInstallmentInterval {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateThreeDSecureAuthenticationFutureUsageInstallmentInterval::*;
        match s {
            "day" => Ok(Day),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateThreeDSecureAuthenticationFutureUsageInstallmentInterval"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateThreeDSecureAuthenticationFutureUsageInstallmentInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationFutureUsageInstallmentInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationFutureUsageInstallmentInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateThreeDSecureAuthenticationFutureUsageInstallmentInterval))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateThreeDSecureAuthenticationFutureUsageInstallmentInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateThreeDSecureAuthenticationFutureUsageInstallmentInterval
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Parameters related to a recurring payment.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateThreeDSecureAuthenticationFutureUsageRecurring {
    /// A non-negative integer representing the future authorizations' amount in the [smallest currency unit](/currencies#zero-decimal).
    pub amount: i64,
    /// Information about the expiry of the future usage of this authentication.
    pub expiry: CreateThreeDSecureAuthenticationFutureUsageRecurringExpiry,
    /// The unit of time for `interval_count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<CreateThreeDSecureAuthenticationFutureUsageRecurringInterval>,
    /// The minimum number of time intervals between authorizations.
    /// Must be greater than 0, and defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationFutureUsageRecurring {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateThreeDSecureAuthenticationFutureUsageRecurring")
            .finish_non_exhaustive()
    }
}
impl CreateThreeDSecureAuthenticationFutureUsageRecurring {
    pub fn new(
        amount: impl Into<i64>,
        expiry: impl Into<CreateThreeDSecureAuthenticationFutureUsageRecurringExpiry>,
    ) -> Self {
        Self { amount: amount.into(), expiry: expiry.into(), interval: None, interval_count: None }
    }
}
/// Information about the expiry of the future usage of this authentication.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateThreeDSecureAuthenticationFutureUsageRecurringExpiry {
    /// The date before which the last authorization related to this authentication will occur.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// The type of expiry for the future use of this authentication.
    #[serde(rename = "type")]
    pub type_: CreateThreeDSecureAuthenticationFutureUsageRecurringExpiryType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationFutureUsageRecurringExpiry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateThreeDSecureAuthenticationFutureUsageRecurringExpiry")
            .finish_non_exhaustive()
    }
}
impl CreateThreeDSecureAuthenticationFutureUsageRecurringExpiry {
    pub fn new(
        type_: impl Into<CreateThreeDSecureAuthenticationFutureUsageRecurringExpiryType>,
    ) -> Self {
        Self { date: None, type_: type_.into() }
    }
}
/// The type of expiry for the future use of this authentication.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateThreeDSecureAuthenticationFutureUsageRecurringExpiryType {
    Date,
    Never,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateThreeDSecureAuthenticationFutureUsageRecurringExpiryType {
    pub fn as_str(&self) -> &str {
        use CreateThreeDSecureAuthenticationFutureUsageRecurringExpiryType::*;
        match self {
            Date => "date",
            Never => "never",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateThreeDSecureAuthenticationFutureUsageRecurringExpiryType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateThreeDSecureAuthenticationFutureUsageRecurringExpiryType::*;
        match s {
            "date" => Ok(Date),
            "never" => Ok(Never),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateThreeDSecureAuthenticationFutureUsageRecurringExpiryType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateThreeDSecureAuthenticationFutureUsageRecurringExpiryType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationFutureUsageRecurringExpiryType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationFutureUsageRecurringExpiryType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateThreeDSecureAuthenticationFutureUsageRecurringExpiryType))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateThreeDSecureAuthenticationFutureUsageRecurringExpiryType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateThreeDSecureAuthenticationFutureUsageRecurringExpiryType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The unit of time for `interval_count`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateThreeDSecureAuthenticationFutureUsageRecurringInterval {
    Day,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateThreeDSecureAuthenticationFutureUsageRecurringInterval {
    pub fn as_str(&self) -> &str {
        use CreateThreeDSecureAuthenticationFutureUsageRecurringInterval::*;
        match self {
            Day => "day",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateThreeDSecureAuthenticationFutureUsageRecurringInterval {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateThreeDSecureAuthenticationFutureUsageRecurringInterval::*;
        match s {
            "day" => Ok(Day),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateThreeDSecureAuthenticationFutureUsageRecurringInterval"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateThreeDSecureAuthenticationFutureUsageRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationFutureUsageRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationFutureUsageRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateThreeDSecureAuthenticationFutureUsageRecurringInterval))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateThreeDSecureAuthenticationFutureUsageRecurringInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateThreeDSecureAuthenticationFutureUsageRecurringInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The type of future usage declared for this 3DS Authentication
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateThreeDSecureAuthenticationFutureUsageType {
    CardOnFile,
    Installment,
    Recurring,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateThreeDSecureAuthenticationFutureUsageType {
    pub fn as_str(&self) -> &str {
        use CreateThreeDSecureAuthenticationFutureUsageType::*;
        match self {
            CardOnFile => "card_on_file",
            Installment => "installment",
            Recurring => "recurring",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateThreeDSecureAuthenticationFutureUsageType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateThreeDSecureAuthenticationFutureUsageType::*;
        match s {
            "card_on_file" => Ok(CardOnFile),
            "installment" => Ok(Installment),
            "recurring" => Ok(Recurring),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateThreeDSecureAuthenticationFutureUsageType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateThreeDSecureAuthenticationFutureUsageType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationFutureUsageType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationFutureUsageType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateThreeDSecureAuthenticationFutureUsageType))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateThreeDSecureAuthenticationFutureUsageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateThreeDSecureAuthenticationFutureUsageType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Hash used to generate the PaymentMethod to be used for this Authentication.
/// This is mutually exclusive with the `payment_method` parameter.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateThreeDSecureAuthenticationPaymentMethodData {
    /// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<CreateThreeDSecureAuthenticationPaymentMethodDataBillingDetails>,
    pub card: CreateThreeDSecureAuthenticationPaymentMethodDataCard,
    /// The type of the PaymentMethod.
    /// An additional hash is included on the PaymentMethod with a name matching this value.
    /// It contains additional information specific to the PaymentMethod type.
    #[serde(rename = "type")]
    pub type_: CreateThreeDSecureAuthenticationPaymentMethodDataType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationPaymentMethodData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateThreeDSecureAuthenticationPaymentMethodData").finish_non_exhaustive()
    }
}
impl CreateThreeDSecureAuthenticationPaymentMethodData {
    pub fn new(
        card: impl Into<CreateThreeDSecureAuthenticationPaymentMethodDataCard>,
        type_: impl Into<CreateThreeDSecureAuthenticationPaymentMethodDataType>,
    ) -> Self {
        Self { billing_details: None, card: card.into(), type_: type_.into() }
    }
}
/// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateThreeDSecureAuthenticationPaymentMethodDataBillingDetails {
    /// Billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Billing phone number (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationPaymentMethodDataBillingDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateThreeDSecureAuthenticationPaymentMethodDataBillingDetails")
            .finish_non_exhaustive()
    }
}
impl CreateThreeDSecureAuthenticationPaymentMethodDataBillingDetails {
    pub fn new() -> Self {
        Self { address: None, email: None, name: None, phone: None }
    }
}
impl Default for CreateThreeDSecureAuthenticationPaymentMethodDataBillingDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateThreeDSecureAuthenticationPaymentMethodDataCard {
    #[serde(untagged)]
    Card(CreateThreeDSecureAuthenticationPaymentMethodDataCard),
    #[serde(untagged)]
    CardToken(CreateThreeDSecureAuthenticationPaymentMethodDataCardToken),
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationPaymentMethodDataCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateThreeDSecureAuthenticationPaymentMethodDataCard")
            .finish_non_exhaustive()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateThreeDSecureAuthenticationPaymentMethodDataCard {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc: Option<String>,
    pub exp_month: i64,
    pub exp_year: i64,
    pub number: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationPaymentMethodDataCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateThreeDSecureAuthenticationPaymentMethodDataCard")
            .finish_non_exhaustive()
    }
}
impl CreateThreeDSecureAuthenticationPaymentMethodDataCard {
    pub fn new(
        exp_month: impl Into<i64>,
        exp_year: impl Into<i64>,
        number: impl Into<String>,
    ) -> Self {
        Self {
            cvc: None,
            exp_month: exp_month.into(),
            exp_year: exp_year.into(),
            number: number.into(),
        }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateThreeDSecureAuthenticationPaymentMethodDataCardToken {
    pub token: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationPaymentMethodDataCardToken {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateThreeDSecureAuthenticationPaymentMethodDataCardToken")
            .finish_non_exhaustive()
    }
}
impl CreateThreeDSecureAuthenticationPaymentMethodDataCardToken {
    pub fn new(token: impl Into<String>) -> Self {
        Self { token: token.into() }
    }
}
/// The type of the PaymentMethod.
/// An additional hash is included on the PaymentMethod with a name matching this value.
/// It contains additional information specific to the PaymentMethod type.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateThreeDSecureAuthenticationPaymentMethodDataType {
    Card,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateThreeDSecureAuthenticationPaymentMethodDataType {
    pub fn as_str(&self) -> &str {
        use CreateThreeDSecureAuthenticationPaymentMethodDataType::*;
        match self {
            Card => "card",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateThreeDSecureAuthenticationPaymentMethodDataType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateThreeDSecureAuthenticationPaymentMethodDataType::*;
        match s {
            "card" => Ok(Card),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateThreeDSecureAuthenticationPaymentMethodDataType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateThreeDSecureAuthenticationPaymentMethodDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationPaymentMethodDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationPaymentMethodDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateThreeDSecureAuthenticationPaymentMethodDataType))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateThreeDSecureAuthenticationPaymentMethodDataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateThreeDSecureAuthenticationPaymentMethodDataType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Set to `always` to skip the fingerprinting step and submit this Authentication immediately or `if_fingerprinting_not_supported` to submit this Authentication only if fingerprinting is not available.
/// This parameter defaults to `never`.
///
/// Refer to the [Submit at creation section of the standalone 3DS guide](/payments/3d-secure/standalone-3d-secure#submit-at-creation) for more information.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateThreeDSecureAuthenticationSubmit {
    Always,
    IfFingerprintingNotSupported,
    Never,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateThreeDSecureAuthenticationSubmit {
    pub fn as_str(&self) -> &str {
        use CreateThreeDSecureAuthenticationSubmit::*;
        match self {
            Always => "always",
            IfFingerprintingNotSupported => "if_fingerprinting_not_supported",
            Never => "never",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateThreeDSecureAuthenticationSubmit {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateThreeDSecureAuthenticationSubmit::*;
        match s {
            "always" => Ok(Always),
            "if_fingerprinting_not_supported" => Ok(IfFingerprintingNotSupported),
            "never" => Ok(Never),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateThreeDSecureAuthenticationSubmit"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateThreeDSecureAuthenticationSubmit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationSubmit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateThreeDSecureAuthenticationSubmit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateThreeDSecureAuthenticationSubmit)).finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateThreeDSecureAuthenticationSubmit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateThreeDSecureAuthenticationSubmit {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// This endpoint creates a 3DS Authentication.
/// Refer to the <a href="/payments/3d-secure/standalone-3d-secure#create-a-3ds-authentication-object">Create a 3DS Authentication object section of the Standalone 3DS guide</a> for more information.
///
/// You can pass the submit parameter to automatically submit the 3DS Authentication object when you create it.
/// Refer to the <a href="/payments/3d-secure/standalone-3d-secure#submit-at-creation">Submit at creation section of the Standalone 3DS guide</a> for more information.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateThreeDSecureAuthentication {
    inner: CreateThreeDSecureAuthenticationBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateThreeDSecureAuthentication {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateThreeDSecureAuthentication").finish_non_exhaustive()
    }
}
impl CreateThreeDSecureAuthentication {
    /// Construct a new `CreateThreeDSecureAuthentication`.
    pub fn new(
        channel: impl Into<CreateThreeDSecureAuthenticationChannel>,
        message_category: impl Into<stripe_payment::ThreeDSecureAuthenticationMessageCategory>,
    ) -> Self {
        Self {
            inner: CreateThreeDSecureAuthenticationBuilder::new(
                channel.into(),
                message_category.into(),
            ),
        }
    }
    /// Contains additional details about the acquirer for this 3DS Authentication.
    ///
    /// Refer to the [Pass acquirer details and directory server section of the standalone 3DS guide](/payments/3d-secure/standalone-3d-secure#pass-acquirer-details-and-directory-server) for more information.
    pub fn acquirer_details(
        mut self,
        acquirer_details: impl Into<CreateThreeDSecureAuthenticationAcquirerDetails>,
    ) -> Self {
        self.inner.acquirer_details = Some(acquirer_details.into());
        self
    }
    /// A non-negative integer representing the amount in the [smallest currency unit](/currencies#zero-decimal).
    /// You can't include this parameter if `message_category` is `non_payment_authentication`.
    pub fn amount(mut self, amount: impl Into<i64>) -> Self {
        self.inner.amount = Some(amount.into());
        self
    }
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub fn currency(mut self, currency: impl Into<stripe_types::Currency>) -> Self {
        self.inner.currency = Some(currency.into());
        self
    }
    /// The 3DS directory server with which this 3DS Authentication was processed.
    pub fn directory_server(
        mut self,
        directory_server: impl Into<stripe_payment::ThreeDSecureAuthenticationDirectoryServer>,
    ) -> Self {
        self.inner.directory_server = Some(directory_server.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Contains additional details on your flow preference for this 3DS Authentication.
    ///
    /// Refer to the [Specify a flow preference section of the standalone 3DS guide](/payments/3d-secure/standalone-3d-secure#specify-a-flow-preference) for more information.
    pub fn flow_preference(
        mut self,
        flow_preference: impl Into<CreateThreeDSecureAuthenticationFlowPreference>,
    ) -> Self {
        self.inner.flow_preference = Some(flow_preference.into());
        self
    }
    /// Contains information about future usage of this 3DS Authentication
    pub fn future_usage(
        mut self,
        future_usage: impl Into<CreateThreeDSecureAuthenticationFutureUsage>,
    ) -> Self {
        self.inner.future_usage = Some(future_usage.into());
        self
    }
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
    /// ID of the payment method (a PaymentMethod object) to attach to this 3DS Authentication.
    pub fn payment_method(mut self, payment_method: impl Into<String>) -> Self {
        self.inner.payment_method = Some(payment_method.into());
        self
    }
    /// Hash used to generate the PaymentMethod to be used for this Authentication.
    /// This is mutually exclusive with the `payment_method` parameter.
    pub fn payment_method_data(
        mut self,
        payment_method_data: impl Into<CreateThreeDSecureAuthenticationPaymentMethodData>,
    ) -> Self {
        self.inner.payment_method_data = Some(payment_method_data.into());
        self
    }
    /// The reason for invoking standalone 3DS.
    /// This is tailored specifically for cases when you want Stripe to help determine the standalone 3DS flow to fit your use case instead of needing to select a specific 3DS flow.
    ///
    /// This parameter is exclusive with `flow_preference`.
    /// You can either use `reason` for controlling 3DS according to your business requirements, or use `flow_preference` for having fine-grained control over your 3DS flow preference.
    pub fn reason(
        mut self,
        reason: impl Into<stripe_payment::ThreeDSecureAuthenticationReason>,
    ) -> Self {
        self.inner.reason = Some(reason.into());
        self
    }
    /// The shipping address requested by the cardholder.
    /// You should try to include as complete address information as possible.
    pub fn shipping_address(mut self, shipping_address: impl Into<Address>) -> Self {
        self.inner.shipping_address = Some(shipping_address.into());
        self
    }
    /// Set to `always` to skip the fingerprinting step and submit this Authentication immediately or `if_fingerprinting_not_supported` to submit this Authentication only if fingerprinting is not available.
    /// This parameter defaults to `never`.
    ///
    /// Refer to the [Submit at creation section of the standalone 3DS guide](/payments/3d-secure/standalone-3d-secure#submit-at-creation) for more information.
    pub fn submit(mut self, submit: impl Into<CreateThreeDSecureAuthenticationSubmit>) -> Self {
        self.inner.submit = Some(submit.into());
        self
    }
}
impl CreateThreeDSecureAuthentication {
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

impl StripeRequest for CreateThreeDSecureAuthentication {
    type Output = stripe_payment::ThreeDSecureAuthentication;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/three_d_secure/authentications").form(&self.inner)
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct CancelThreeDSecureAuthenticationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CancelThreeDSecureAuthenticationBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CancelThreeDSecureAuthenticationBuilder").finish_non_exhaustive()
    }
}
impl CancelThreeDSecureAuthenticationBuilder {
    fn new() -> Self {
        Self { expand: None, metadata: None }
    }
}
/// This endpoint cancels a 3DS Authentication.
/// You can cancel a 3DS Authentication object when it’s in a non-final status:.
/// `requires_submission` or `requires_challenge`.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CancelThreeDSecureAuthentication {
    inner: CancelThreeDSecureAuthenticationBuilder,
    authentication: stripe_payment::ThreeDSecureAuthenticationId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CancelThreeDSecureAuthentication {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CancelThreeDSecureAuthentication").finish_non_exhaustive()
    }
}
impl CancelThreeDSecureAuthentication {
    /// Construct a new `CancelThreeDSecureAuthentication`.
    pub fn new(authentication: impl Into<stripe_payment::ThreeDSecureAuthenticationId>) -> Self {
        Self {
            authentication: authentication.into(),
            inner: CancelThreeDSecureAuthenticationBuilder::new(),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
}
impl CancelThreeDSecureAuthentication {
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

impl StripeRequest for CancelThreeDSecureAuthentication {
    type Output = stripe_payment::ThreeDSecureAuthentication;

    fn build(&self) -> RequestBuilder {
        let authentication = &self.authentication;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/three_d_secure/authentications/{authentication}/cancel"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct SubmitThreeDSecureAuthenticationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fingerprinting_result: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubmitThreeDSecureAuthenticationBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubmitThreeDSecureAuthenticationBuilder").finish_non_exhaustive()
    }
}
impl SubmitThreeDSecureAuthenticationBuilder {
    fn new() -> Self {
        Self { expand: None, fingerprinting_result: None, metadata: None }
    }
}
/// This endpoint submits a 3DS Authentication.
/// You can submit a 3DS Authentication object when it has status `requires_submission`.
/// Refer to the <a href="/payments/3d-secure/standalone-3d-secure#submit-the-3ds-authentication-object">Submit the 3DS Authentication object section of the Standalone 3DS guide</a> for more information.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct SubmitThreeDSecureAuthentication {
    inner: SubmitThreeDSecureAuthenticationBuilder,
    authentication: stripe_payment::ThreeDSecureAuthenticationId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubmitThreeDSecureAuthentication {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubmitThreeDSecureAuthentication").finish_non_exhaustive()
    }
}
impl SubmitThreeDSecureAuthentication {
    /// Construct a new `SubmitThreeDSecureAuthentication`.
    pub fn new(authentication: impl Into<stripe_payment::ThreeDSecureAuthenticationId>) -> Self {
        Self {
            authentication: authentication.into(),
            inner: SubmitThreeDSecureAuthenticationBuilder::new(),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// The fingerprinting result of the issuer fingerprinting step.
    ///
    /// Refer to the [Issuer fingerprinting section of the Standalone 3DS guide](/payments/3d-secure/standalone-3d-secure#issuer-fingerprinting) for more information.
    pub fn fingerprinting_result(mut self, fingerprinting_result: impl Into<String>) -> Self {
        self.inner.fingerprinting_result = Some(fingerprinting_result.into());
        self
    }
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
}
impl SubmitThreeDSecureAuthentication {
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

impl StripeRequest for SubmitThreeDSecureAuthentication {
    type Output = stripe_payment::ThreeDSecureAuthentication;

    fn build(&self) -> RequestBuilder {
        let authentication = &self.authentication;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/three_d_secure/authentications/{authentication}/submit"),
        )
        .form(&self.inner)
    }
}

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct Address {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Address line 1, such as the street, PO Box, or company name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Address line 2, such as the apartment, suite, unit, or building.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// Country subdivision code defined in ISO 3166-2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Address").finish_non_exhaustive()
    }
}
impl Address {
    pub fn new() -> Self {
        Self { city: None, country: None, line1: None, line2: None, postal_code: None, state: None }
    }
}
impl Default for Address {
    fn default() -> Self {
        Self::new()
    }
}

use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

/// Deletes a `Configuration` object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct DeleteTerminalConfiguration {
    configuration: stripe_terminal::TerminalConfigurationId,
}
impl DeleteTerminalConfiguration {
    /// Construct a new `DeleteTerminalConfiguration`.
    pub fn new(configuration: impl Into<stripe_terminal::TerminalConfigurationId>) -> Self {
        Self { configuration: configuration.into() }
    }
}
impl DeleteTerminalConfiguration {
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

impl StripeRequest for DeleteTerminalConfiguration {
    type Output = stripe_terminal::DeletedTerminalConfiguration;

    fn build(&self) -> RequestBuilder {
        let configuration = &self.configuration;
        RequestBuilder::new(
            StripeMethod::Delete,
            format!("/terminal/configurations/{configuration}"),
        )
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct ListTerminalConfigurationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_account_default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
impl ListTerminalConfigurationBuilder {
    fn new() -> Self {
        Self {
            ending_before: None,
            expand: None,
            is_account_default: None,
            limit: None,
            starting_after: None,
        }
    }
}
/// Returns a list of `Configuration` objects.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListTerminalConfiguration {
    inner: ListTerminalConfigurationBuilder,
}
impl ListTerminalConfiguration {
    /// Construct a new `ListTerminalConfiguration`.
    pub fn new() -> Self {
        Self { inner: ListTerminalConfigurationBuilder::new() }
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
    /// if present, only return the account default or non-default configurations.
    pub fn is_account_default(mut self, is_account_default: impl Into<bool>) -> Self {
        self.inner.is_account_default = Some(is_account_default.into());
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
}
impl Default for ListTerminalConfiguration {
    fn default() -> Self {
        Self::new()
    }
}
impl ListTerminalConfiguration {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_terminal::TerminalConfiguration>>
    {
        stripe_client_core::ListPaginator::new_list("/terminal/configurations", &self.inner)
    }
}

impl StripeRequest for ListTerminalConfiguration {
    type Output = stripe_types::List<stripe_terminal::TerminalConfiguration>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/terminal/configurations").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveTerminalConfigurationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveTerminalConfigurationBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a `Configuration` object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveTerminalConfiguration {
    inner: RetrieveTerminalConfigurationBuilder,
    configuration: stripe_terminal::TerminalConfigurationId,
}
impl RetrieveTerminalConfiguration {
    /// Construct a new `RetrieveTerminalConfiguration`.
    pub fn new(configuration: impl Into<stripe_terminal::TerminalConfigurationId>) -> Self {
        Self {
            configuration: configuration.into(),
            inner: RetrieveTerminalConfigurationBuilder::new(),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveTerminalConfiguration {
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

impl StripeRequest for RetrieveTerminalConfiguration {
    type Output = RetrieveTerminalConfigurationReturned;

    fn build(&self) -> RequestBuilder {
        let configuration = &self.configuration;
        RequestBuilder::new(StripeMethod::Get, format!("/terminal/configurations/{configuration}"))
            .query(&self.inner)
    }
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(untagged))]
pub enum RetrieveTerminalConfigurationReturned {
    TerminalConfiguration(stripe_terminal::TerminalConfiguration),
    DeletedTerminalConfiguration(stripe_terminal::DeletedTerminalConfiguration),
}

#[derive(Default)]
pub struct RetrieveTerminalConfigurationReturnedBuilder {
    inner: stripe_types::miniserde_helpers::MaybeDeletedBuilderInner,
}

const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::MapBuilder;

    use super::*;

    make_place!(Place);

    struct Builder<'a> {
        out: &'a mut Option<RetrieveTerminalConfigurationReturned>,
        builder: RetrieveTerminalConfigurationReturnedBuilder,
    }

    impl Deserialize for RetrieveTerminalConfigurationReturned {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    impl Visitor for Place<RetrieveTerminalConfigurationReturned> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: Default::default() }))
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

    impl MapBuilder for RetrieveTerminalConfigurationReturnedBuilder {
        type Out = RetrieveTerminalConfigurationReturned;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.inner.key_inner(k)
        }

        fn deser_default() -> Self {
            Self::default()
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (deleted, o) = self.inner.finish_inner()?;
            Some(if deleted {
                RetrieveTerminalConfigurationReturned::DeletedTerminalConfiguration(
                    FromValueOpt::from_value(Value::Object(o))?,
                )
            } else {
                RetrieveTerminalConfigurationReturned::TerminalConfiguration(
                    FromValueOpt::from_value(Value::Object(o))?,
                )
            })
        }
    }

    impl stripe_types::ObjectDeser for RetrieveTerminalConfigurationReturned {
        type Builder = RetrieveTerminalConfigurationReturnedBuilder;
    }
};

#[derive(Clone, Debug, serde::Serialize)]
struct CreateTerminalConfigurationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    bbpos_wisepos_e: Option<BbposWisePose>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offline: Option<Offline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reboot_window: Option<RebootWindow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stripe_s700: Option<CreateTerminalConfigurationStripeS700>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping: Option<Tipping>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verifone_p400: Option<CreateTerminalConfigurationVerifoneP400>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wifi: Option<CreateTerminalConfigurationWifi>,
}
impl CreateTerminalConfigurationBuilder {
    fn new() -> Self {
        Self {
            bbpos_wisepos_e: None,
            expand: None,
            name: None,
            offline: None,
            reboot_window: None,
            stripe_s700: None,
            tipping: None,
            verifone_p400: None,
            wifi: None,
        }
    }
}
/// An object containing device type specific settings for Stripe S700 readers
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTerminalConfigurationStripeS700 {
    /// A File ID representing an image you would like displayed on the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splashscreen: Option<String>,
}
impl CreateTerminalConfigurationStripeS700 {
    pub fn new() -> Self {
        Self { splashscreen: None }
    }
}
impl Default for CreateTerminalConfigurationStripeS700 {
    fn default() -> Self {
        Self::new()
    }
}
/// An object containing device type specific settings for Verifone P400 readers
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTerminalConfigurationVerifoneP400 {
    /// A File ID representing an image you would like displayed on the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splashscreen: Option<String>,
}
impl CreateTerminalConfigurationVerifoneP400 {
    pub fn new() -> Self {
        Self { splashscreen: None }
    }
}
impl Default for CreateTerminalConfigurationVerifoneP400 {
    fn default() -> Self {
        Self::new()
    }
}
/// Configurations for connecting to a WiFi network.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTerminalConfigurationWifi {
    /// Credentials for a WPA-Enterprise WiFi network using the EAP-PEAP authentication method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_eap_peap: Option<EnterprisePeapConfig>,
    /// Credentials for a WPA-Enterprise WiFi network using the EAP-TLS authentication method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_eap_tls: Option<EnterpriseTlsConfig>,
    /// Credentials for a WPA-Personal WiFi network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_psk: Option<PersonalPskConfig>,
    /// Security type of the WiFi network.
    /// Fill out the hash with the corresponding name to provide the set of credentials for this security type.
    #[serde(rename = "type")]
    pub type_: CreateTerminalConfigurationWifiType,
}
impl CreateTerminalConfigurationWifi {
    pub fn new(type_: impl Into<CreateTerminalConfigurationWifiType>) -> Self {
        Self {
            enterprise_eap_peap: None,
            enterprise_eap_tls: None,
            personal_psk: None,
            type_: type_.into(),
        }
    }
}
/// Security type of the WiFi network.
/// Fill out the hash with the corresponding name to provide the set of credentials for this security type.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTerminalConfigurationWifiType {
    EnterpriseEapPeap,
    EnterpriseEapTls,
    PersonalPsk,
}
impl CreateTerminalConfigurationWifiType {
    pub fn as_str(self) -> &'static str {
        use CreateTerminalConfigurationWifiType::*;
        match self {
            EnterpriseEapPeap => "enterprise_eap_peap",
            EnterpriseEapTls => "enterprise_eap_tls",
            PersonalPsk => "personal_psk",
        }
    }
}

impl std::str::FromStr for CreateTerminalConfigurationWifiType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTerminalConfigurationWifiType::*;
        match s {
            "enterprise_eap_peap" => Ok(EnterpriseEapPeap),
            "enterprise_eap_tls" => Ok(EnterpriseEapTls),
            "personal_psk" => Ok(PersonalPsk),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTerminalConfigurationWifiType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTerminalConfigurationWifiType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTerminalConfigurationWifiType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTerminalConfigurationWifiType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTerminalConfigurationWifiType")
        })
    }
}
/// Creates a new `Configuration` object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTerminalConfiguration {
    inner: CreateTerminalConfigurationBuilder,
}
impl CreateTerminalConfiguration {
    /// Construct a new `CreateTerminalConfiguration`.
    pub fn new() -> Self {
        Self { inner: CreateTerminalConfigurationBuilder::new() }
    }
    /// An object containing device type specific settings for BBPOS WisePOS E readers
    pub fn bbpos_wisepos_e(mut self, bbpos_wisepos_e: impl Into<BbposWisePose>) -> Self {
        self.inner.bbpos_wisepos_e = Some(bbpos_wisepos_e.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Name of the configuration
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.inner.name = Some(name.into());
        self
    }
    /// Configurations for collecting transactions offline.
    pub fn offline(mut self, offline: impl Into<Offline>) -> Self {
        self.inner.offline = Some(offline.into());
        self
    }
    /// Reboot time settings for readers that support customized reboot time configuration.
    pub fn reboot_window(mut self, reboot_window: impl Into<RebootWindow>) -> Self {
        self.inner.reboot_window = Some(reboot_window.into());
        self
    }
    /// An object containing device type specific settings for Stripe S700 readers
    pub fn stripe_s700(
        mut self,
        stripe_s700: impl Into<CreateTerminalConfigurationStripeS700>,
    ) -> Self {
        self.inner.stripe_s700 = Some(stripe_s700.into());
        self
    }
    /// Tipping configurations for readers supporting on-reader tips
    pub fn tipping(mut self, tipping: impl Into<Tipping>) -> Self {
        self.inner.tipping = Some(tipping.into());
        self
    }
    /// An object containing device type specific settings for Verifone P400 readers
    pub fn verifone_p400(
        mut self,
        verifone_p400: impl Into<CreateTerminalConfigurationVerifoneP400>,
    ) -> Self {
        self.inner.verifone_p400 = Some(verifone_p400.into());
        self
    }
    /// Configurations for connecting to a WiFi network.
    pub fn wifi(mut self, wifi: impl Into<CreateTerminalConfigurationWifi>) -> Self {
        self.inner.wifi = Some(wifi.into());
        self
    }
}
impl Default for CreateTerminalConfiguration {
    fn default() -> Self {
        Self::new()
    }
}
impl CreateTerminalConfiguration {
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

impl StripeRequest for CreateTerminalConfiguration {
    type Output = stripe_terminal::TerminalConfiguration;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/terminal/configurations").form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct UpdateTerminalConfigurationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    bbpos_wisepos_e: Option<BbposWisePose>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offline: Option<Offline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reboot_window: Option<RebootWindow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stripe_s700: Option<UpdateTerminalConfigurationStripeS700>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping: Option<Tipping>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verifone_p400: Option<UpdateTerminalConfigurationVerifoneP400>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wifi: Option<UpdateTerminalConfigurationWifi>,
}
impl UpdateTerminalConfigurationBuilder {
    fn new() -> Self {
        Self {
            bbpos_wisepos_e: None,
            expand: None,
            name: None,
            offline: None,
            reboot_window: None,
            stripe_s700: None,
            tipping: None,
            verifone_p400: None,
            wifi: None,
        }
    }
}
/// An object containing device type specific settings for Stripe S700 readers
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateTerminalConfigurationStripeS700 {
    /// A File ID representing an image you would like displayed on the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splashscreen: Option<String>,
}
impl UpdateTerminalConfigurationStripeS700 {
    pub fn new() -> Self {
        Self { splashscreen: None }
    }
}
impl Default for UpdateTerminalConfigurationStripeS700 {
    fn default() -> Self {
        Self::new()
    }
}
/// An object containing device type specific settings for Verifone P400 readers
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateTerminalConfigurationVerifoneP400 {
    /// A File ID representing an image you would like displayed on the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splashscreen: Option<String>,
}
impl UpdateTerminalConfigurationVerifoneP400 {
    pub fn new() -> Self {
        Self { splashscreen: None }
    }
}
impl Default for UpdateTerminalConfigurationVerifoneP400 {
    fn default() -> Self {
        Self::new()
    }
}
/// Configurations for connecting to a WiFi network.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateTerminalConfigurationWifi {
    /// Credentials for a WPA-Enterprise WiFi network using the EAP-PEAP authentication method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_eap_peap: Option<EnterprisePeapConfig>,
    /// Credentials for a WPA-Enterprise WiFi network using the EAP-TLS authentication method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_eap_tls: Option<EnterpriseTlsConfig>,
    /// Credentials for a WPA-Personal WiFi network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_psk: Option<PersonalPskConfig>,
    /// Security type of the WiFi network.
    /// Fill out the hash with the corresponding name to provide the set of credentials for this security type.
    #[serde(rename = "type")]
    pub type_: UpdateTerminalConfigurationWifiType,
}
impl UpdateTerminalConfigurationWifi {
    pub fn new(type_: impl Into<UpdateTerminalConfigurationWifiType>) -> Self {
        Self {
            enterprise_eap_peap: None,
            enterprise_eap_tls: None,
            personal_psk: None,
            type_: type_.into(),
        }
    }
}
/// Security type of the WiFi network.
/// Fill out the hash with the corresponding name to provide the set of credentials for this security type.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateTerminalConfigurationWifiType {
    EnterpriseEapPeap,
    EnterpriseEapTls,
    PersonalPsk,
}
impl UpdateTerminalConfigurationWifiType {
    pub fn as_str(self) -> &'static str {
        use UpdateTerminalConfigurationWifiType::*;
        match self {
            EnterpriseEapPeap => "enterprise_eap_peap",
            EnterpriseEapTls => "enterprise_eap_tls",
            PersonalPsk => "personal_psk",
        }
    }
}

impl std::str::FromStr for UpdateTerminalConfigurationWifiType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateTerminalConfigurationWifiType::*;
        match s {
            "enterprise_eap_peap" => Ok(EnterpriseEapPeap),
            "enterprise_eap_tls" => Ok(EnterpriseEapTls),
            "personal_psk" => Ok(PersonalPsk),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateTerminalConfigurationWifiType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateTerminalConfigurationWifiType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateTerminalConfigurationWifiType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateTerminalConfigurationWifiType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpdateTerminalConfigurationWifiType")
        })
    }
}
/// Updates a new `Configuration` object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateTerminalConfiguration {
    inner: UpdateTerminalConfigurationBuilder,
    configuration: stripe_terminal::TerminalConfigurationId,
}
impl UpdateTerminalConfiguration {
    /// Construct a new `UpdateTerminalConfiguration`.
    pub fn new(configuration: impl Into<stripe_terminal::TerminalConfigurationId>) -> Self {
        Self {
            configuration: configuration.into(),
            inner: UpdateTerminalConfigurationBuilder::new(),
        }
    }
    /// An object containing device type specific settings for BBPOS WisePOS E readers
    pub fn bbpos_wisepos_e(mut self, bbpos_wisepos_e: impl Into<BbposWisePose>) -> Self {
        self.inner.bbpos_wisepos_e = Some(bbpos_wisepos_e.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Name of the configuration
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.inner.name = Some(name.into());
        self
    }
    /// Configurations for collecting transactions offline.
    pub fn offline(mut self, offline: impl Into<Offline>) -> Self {
        self.inner.offline = Some(offline.into());
        self
    }
    /// Reboot time settings for readers that support customized reboot time configuration.
    pub fn reboot_window(mut self, reboot_window: impl Into<RebootWindow>) -> Self {
        self.inner.reboot_window = Some(reboot_window.into());
        self
    }
    /// An object containing device type specific settings for Stripe S700 readers
    pub fn stripe_s700(
        mut self,
        stripe_s700: impl Into<UpdateTerminalConfigurationStripeS700>,
    ) -> Self {
        self.inner.stripe_s700 = Some(stripe_s700.into());
        self
    }
    /// Tipping configurations for readers supporting on-reader tips
    pub fn tipping(mut self, tipping: impl Into<Tipping>) -> Self {
        self.inner.tipping = Some(tipping.into());
        self
    }
    /// An object containing device type specific settings for Verifone P400 readers
    pub fn verifone_p400(
        mut self,
        verifone_p400: impl Into<UpdateTerminalConfigurationVerifoneP400>,
    ) -> Self {
        self.inner.verifone_p400 = Some(verifone_p400.into());
        self
    }
    /// Configurations for connecting to a WiFi network.
    pub fn wifi(mut self, wifi: impl Into<UpdateTerminalConfigurationWifi>) -> Self {
        self.inner.wifi = Some(wifi.into());
        self
    }
}
impl UpdateTerminalConfiguration {
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

impl StripeRequest for UpdateTerminalConfiguration {
    type Output = UpdateTerminalConfigurationReturned;

    fn build(&self) -> RequestBuilder {
        let configuration = &self.configuration;
        RequestBuilder::new(StripeMethod::Post, format!("/terminal/configurations/{configuration}"))
            .form(&self.inner)
    }
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(untagged))]
pub enum UpdateTerminalConfigurationReturned {
    TerminalConfiguration(stripe_terminal::TerminalConfiguration),
    DeletedTerminalConfiguration(stripe_terminal::DeletedTerminalConfiguration),
}

#[derive(Default)]
pub struct UpdateTerminalConfigurationReturnedBuilder {
    inner: stripe_types::miniserde_helpers::MaybeDeletedBuilderInner,
}

const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::MapBuilder;

    use super::*;

    make_place!(Place);

    struct Builder<'a> {
        out: &'a mut Option<UpdateTerminalConfigurationReturned>,
        builder: UpdateTerminalConfigurationReturnedBuilder,
    }

    impl Deserialize for UpdateTerminalConfigurationReturned {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    impl Visitor for Place<UpdateTerminalConfigurationReturned> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: Default::default() }))
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

    impl MapBuilder for UpdateTerminalConfigurationReturnedBuilder {
        type Out = UpdateTerminalConfigurationReturned;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.inner.key_inner(k)
        }

        fn deser_default() -> Self {
            Self::default()
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (deleted, o) = self.inner.finish_inner()?;
            Some(if deleted {
                UpdateTerminalConfigurationReturned::DeletedTerminalConfiguration(
                    FromValueOpt::from_value(Value::Object(o))?,
                )
            } else {
                UpdateTerminalConfigurationReturned::TerminalConfiguration(
                    FromValueOpt::from_value(Value::Object(o))?,
                )
            })
        }
    }

    impl stripe_types::ObjectDeser for UpdateTerminalConfigurationReturned {
        type Builder = UpdateTerminalConfigurationReturnedBuilder;
    }
};

#[derive(Clone, Debug, serde::Serialize)]
pub struct BbposWisePose {
    /// A File ID representing an image to display on the reader
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splashscreen: Option<String>,
}
impl BbposWisePose {
    pub fn new() -> Self {
        Self { splashscreen: None }
    }
}
impl Default for BbposWisePose {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct Offline {
    /// Determines whether to allow transactions to be collected while reader is offline.
    /// Defaults to false.
    pub enabled: bool,
}
impl Offline {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into() }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct RebootWindow {
    /// Integer between 0 to 23 that represents the end hour of the reboot time window.
    /// The value must be different than the start_hour.
    pub end_hour: i64,
    /// Integer between 0 to 23 that represents the start hour of the reboot time window.
    pub start_hour: i64,
}
impl RebootWindow {
    pub fn new(end_hour: impl Into<i64>, start_hour: impl Into<i64>) -> Self {
        Self { end_hour: end_hour.into(), start_hour: start_hour.into() }
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct CurrencySpecificConfig {
    /// Fixed amounts displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,
    /// Percentages displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,
    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}
impl CurrencySpecificConfig {
    pub fn new() -> Self {
        Self { fixed_amounts: None, percentages: None, smart_tip_threshold: None }
    }
}
impl Default for CurrencySpecificConfig {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct EnterprisePeapConfig {
    /// A File ID representing a PEM file containing the server certificate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_certificate_file: Option<String>,
    /// Password for connecting to the WiFi network
    pub password: String,
    /// Name of the WiFi network
    pub ssid: String,
    /// Username for connecting to the WiFi network
    pub username: String,
}
impl EnterprisePeapConfig {
    pub fn new(
        password: impl Into<String>,
        ssid: impl Into<String>,
        username: impl Into<String>,
    ) -> Self {
        Self {
            ca_certificate_file: None,
            password: password.into(),
            ssid: ssid.into(),
            username: username.into(),
        }
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct EnterpriseTlsConfig {
    /// A File ID representing a PEM file containing the server certificate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_certificate_file: Option<String>,
    /// A File ID representing a PEM file containing the client certificate
    pub client_certificate_file: String,
    /// A File ID representing a PEM file containing the client RSA private key
    pub private_key_file: String,
    /// Password for the private key file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key_file_password: Option<String>,
    /// Name of the WiFi network
    pub ssid: String,
}
impl EnterpriseTlsConfig {
    pub fn new(
        client_certificate_file: impl Into<String>,
        private_key_file: impl Into<String>,
        ssid: impl Into<String>,
    ) -> Self {
        Self {
            ca_certificate_file: None,
            client_certificate_file: client_certificate_file.into(),
            private_key_file: private_key_file.into(),
            private_key_file_password: None,
            ssid: ssid.into(),
        }
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct PersonalPskConfig {
    /// Password for connecting to the WiFi network
    pub password: String,
    /// Name of the WiFi network
    pub ssid: String,
}
impl PersonalPskConfig {
    pub fn new(password: impl Into<String>, ssid: impl Into<String>) -> Self {
        Self { password: password.into(), ssid: ssid.into() }
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct Tipping {
    /// Tipping configuration for AUD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aud: Option<CurrencySpecificConfig>,
    /// Tipping configuration for CAD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cad: Option<CurrencySpecificConfig>,
    /// Tipping configuration for CHF
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chf: Option<CurrencySpecificConfig>,
    /// Tipping configuration for CZK
    #[serde(skip_serializing_if = "Option::is_none")]
    pub czk: Option<CurrencySpecificConfig>,
    /// Tipping configuration for DKK
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkk: Option<CurrencySpecificConfig>,
    /// Tipping configuration for EUR
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eur: Option<CurrencySpecificConfig>,
    /// Tipping configuration for GBP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gbp: Option<CurrencySpecificConfig>,
    /// Tipping configuration for HKD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hkd: Option<CurrencySpecificConfig>,
    /// Tipping configuration for JPY
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jpy: Option<CurrencySpecificConfig>,
    /// Tipping configuration for MYR
    #[serde(skip_serializing_if = "Option::is_none")]
    pub myr: Option<CurrencySpecificConfig>,
    /// Tipping configuration for NOK
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nok: Option<CurrencySpecificConfig>,
    /// Tipping configuration for NZD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nzd: Option<CurrencySpecificConfig>,
    /// Tipping configuration for PLN
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pln: Option<CurrencySpecificConfig>,
    /// Tipping configuration for SEK
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sek: Option<CurrencySpecificConfig>,
    /// Tipping configuration for SGD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sgd: Option<CurrencySpecificConfig>,
    /// Tipping configuration for USD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usd: Option<CurrencySpecificConfig>,
}
impl Tipping {
    pub fn new() -> Self {
        Self {
            aud: None,
            cad: None,
            chf: None,
            czk: None,
            dkk: None,
            eur: None,
            gbp: None,
            hkd: None,
            jpy: None,
            myr: None,
            nok: None,
            nzd: None,
            pln: None,
            sek: None,
            sgd: None,
            usd: None,
        }
    }
}
impl Default for Tipping {
    fn default() -> Self {
        Self::new()
    }
}

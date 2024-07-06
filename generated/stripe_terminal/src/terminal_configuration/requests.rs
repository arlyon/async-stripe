use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

/// Deletes a `Configuration` object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct DeleteTerminalConfiguration<'a> {
    configuration: &'a stripe_terminal::TerminalConfigurationId,
}
impl<'a> DeleteTerminalConfiguration<'a> {
    /// Construct a new `DeleteTerminalConfiguration`.
    pub fn new(configuration: &'a stripe_terminal::TerminalConfigurationId) -> Self {
        Self { configuration }
    }
}
impl DeleteTerminalConfiguration<'_> {
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

impl StripeRequest for DeleteTerminalConfiguration<'_> {
    type Output = stripe_terminal::DeletedTerminalConfiguration;

    fn build(&self) -> RequestBuilder {
        let configuration = self.configuration;
        RequestBuilder::new(
            StripeMethod::Delete,
            format!("/terminal/configurations/{configuration}"),
        )
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListTerminalConfigurationBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_account_default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListTerminalConfigurationBuilder<'a> {
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
pub struct ListTerminalConfiguration<'a> {
    inner: ListTerminalConfigurationBuilder<'a>,
}
impl<'a> ListTerminalConfiguration<'a> {
    /// Construct a new `ListTerminalConfiguration`.
    pub fn new() -> Self {
        Self { inner: ListTerminalConfigurationBuilder::new() }
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: &'a str) -> Self {
        self.inner.ending_before = Some(ending_before);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// if present, only return the account default or non-default configurations.
    pub fn is_account_default(mut self, is_account_default: bool) -> Self {
        self.inner.is_account_default = Some(is_account_default);
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: i64) -> Self {
        self.inner.limit = Some(limit);
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
}
impl<'a> Default for ListTerminalConfiguration<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl ListTerminalConfiguration<'_> {
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
        stripe_client_core::ListPaginator::new_list("/terminal/configurations", self.inner)
    }
}

impl StripeRequest for ListTerminalConfiguration<'_> {
    type Output = stripe_types::List<stripe_terminal::TerminalConfiguration>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/terminal/configurations").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveTerminalConfigurationBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTerminalConfigurationBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a `Configuration` object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveTerminalConfiguration<'a> {
    inner: RetrieveTerminalConfigurationBuilder<'a>,
    configuration: &'a stripe_terminal::TerminalConfigurationId,
}
impl<'a> RetrieveTerminalConfiguration<'a> {
    /// Construct a new `RetrieveTerminalConfiguration`.
    pub fn new(configuration: &'a stripe_terminal::TerminalConfigurationId) -> Self {
        Self { configuration, inner: RetrieveTerminalConfigurationBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveTerminalConfiguration<'_> {
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

impl StripeRequest for RetrieveTerminalConfiguration<'_> {
    type Output = RetrieveTerminalConfigurationReturned;

    fn build(&self) -> RequestBuilder {
        let configuration = self.configuration;
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

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateTerminalConfigurationBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    bbpos_wisepos_e: Option<CreateTerminalConfigurationBbposWiseposE<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offline: Option<Offline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping: Option<Tipping<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verifone_p400: Option<CreateTerminalConfigurationVerifoneP400<'a>>,
}
impl<'a> CreateTerminalConfigurationBuilder<'a> {
    fn new() -> Self {
        Self {
            bbpos_wisepos_e: None,
            expand: None,
            name: None,
            offline: None,
            tipping: None,
            verifone_p400: None,
        }
    }
}
/// An object containing device type specific settings for BBPOS WisePOS E readers
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTerminalConfigurationBbposWiseposE<'a> {
    /// A File ID representing an image you would like displayed on the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splashscreen: Option<&'a str>,
}
impl<'a> CreateTerminalConfigurationBbposWiseposE<'a> {
    pub fn new() -> Self {
        Self { splashscreen: None }
    }
}
impl<'a> Default for CreateTerminalConfigurationBbposWiseposE<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// An object containing device type specific settings for Verifone P400 readers
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTerminalConfigurationVerifoneP400<'a> {
    /// A File ID representing an image you would like displayed on the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splashscreen: Option<&'a str>,
}
impl<'a> CreateTerminalConfigurationVerifoneP400<'a> {
    pub fn new() -> Self {
        Self { splashscreen: None }
    }
}
impl<'a> Default for CreateTerminalConfigurationVerifoneP400<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Creates a new `Configuration` object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTerminalConfiguration<'a> {
    inner: CreateTerminalConfigurationBuilder<'a>,
}
impl<'a> CreateTerminalConfiguration<'a> {
    /// Construct a new `CreateTerminalConfiguration`.
    pub fn new() -> Self {
        Self { inner: CreateTerminalConfigurationBuilder::new() }
    }
    /// An object containing device type specific settings for BBPOS WisePOS E readers
    pub fn bbpos_wisepos_e(
        mut self,
        bbpos_wisepos_e: CreateTerminalConfigurationBbposWiseposE<'a>,
    ) -> Self {
        self.inner.bbpos_wisepos_e = Some(bbpos_wisepos_e);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Name of the configuration
    pub fn name(mut self, name: &'a str) -> Self {
        self.inner.name = Some(name);
        self
    }
    /// Configurations for collecting transactions offline.
    pub fn offline(mut self, offline: Offline) -> Self {
        self.inner.offline = Some(offline);
        self
    }
    /// Tipping configurations for readers supporting on-reader tips
    pub fn tipping(mut self, tipping: Tipping<'a>) -> Self {
        self.inner.tipping = Some(tipping);
        self
    }
    /// An object containing device type specific settings for Verifone P400 readers
    pub fn verifone_p400(
        mut self,
        verifone_p400: CreateTerminalConfigurationVerifoneP400<'a>,
    ) -> Self {
        self.inner.verifone_p400 = Some(verifone_p400);
        self
    }
}
impl<'a> Default for CreateTerminalConfiguration<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl CreateTerminalConfiguration<'_> {
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

impl StripeRequest for CreateTerminalConfiguration<'_> {
    type Output = stripe_terminal::TerminalConfiguration;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/terminal/configurations").form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct UpdateTerminalConfigurationBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    bbpos_wisepos_e: Option<UpdateTerminalConfigurationBbposWiseposE<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offline: Option<Offline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping: Option<Tipping<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verifone_p400: Option<UpdateTerminalConfigurationVerifoneP400<'a>>,
}
impl<'a> UpdateTerminalConfigurationBuilder<'a> {
    fn new() -> Self {
        Self {
            bbpos_wisepos_e: None,
            expand: None,
            name: None,
            offline: None,
            tipping: None,
            verifone_p400: None,
        }
    }
}
/// An object containing device type specific settings for BBPOS WisePOS E readers
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateTerminalConfigurationBbposWiseposE<'a> {
    /// A File ID representing an image you would like displayed on the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splashscreen: Option<&'a str>,
}
impl<'a> UpdateTerminalConfigurationBbposWiseposE<'a> {
    pub fn new() -> Self {
        Self { splashscreen: None }
    }
}
impl<'a> Default for UpdateTerminalConfigurationBbposWiseposE<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// An object containing device type specific settings for Verifone P400 readers
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateTerminalConfigurationVerifoneP400<'a> {
    /// A File ID representing an image you would like displayed on the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splashscreen: Option<&'a str>,
}
impl<'a> UpdateTerminalConfigurationVerifoneP400<'a> {
    pub fn new() -> Self {
        Self { splashscreen: None }
    }
}
impl<'a> Default for UpdateTerminalConfigurationVerifoneP400<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Updates a new `Configuration` object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateTerminalConfiguration<'a> {
    inner: UpdateTerminalConfigurationBuilder<'a>,
    configuration: &'a stripe_terminal::TerminalConfigurationId,
}
impl<'a> UpdateTerminalConfiguration<'a> {
    /// Construct a new `UpdateTerminalConfiguration`.
    pub fn new(configuration: &'a stripe_terminal::TerminalConfigurationId) -> Self {
        Self { configuration, inner: UpdateTerminalConfigurationBuilder::new() }
    }
    /// An object containing device type specific settings for BBPOS WisePOS E readers
    pub fn bbpos_wisepos_e(
        mut self,
        bbpos_wisepos_e: UpdateTerminalConfigurationBbposWiseposE<'a>,
    ) -> Self {
        self.inner.bbpos_wisepos_e = Some(bbpos_wisepos_e);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Name of the configuration
    pub fn name(mut self, name: &'a str) -> Self {
        self.inner.name = Some(name);
        self
    }
    /// Configurations for collecting transactions offline.
    pub fn offline(mut self, offline: Offline) -> Self {
        self.inner.offline = Some(offline);
        self
    }
    /// Tipping configurations for readers supporting on-reader tips
    pub fn tipping(mut self, tipping: Tipping<'a>) -> Self {
        self.inner.tipping = Some(tipping);
        self
    }
    /// An object containing device type specific settings for Verifone P400 readers
    pub fn verifone_p400(
        mut self,
        verifone_p400: UpdateTerminalConfigurationVerifoneP400<'a>,
    ) -> Self {
        self.inner.verifone_p400 = Some(verifone_p400);
        self
    }
}
impl UpdateTerminalConfiguration<'_> {
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

impl StripeRequest for UpdateTerminalConfiguration<'_> {
    type Output = UpdateTerminalConfigurationReturned;

    fn build(&self) -> RequestBuilder {
        let configuration = self.configuration;
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

#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct Offline {
    /// Determines whether to allow transactions to be collected while reader is offline.
    /// Defaults to false.
    pub enabled: bool,
}
impl Offline {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CurrencySpecificConfig<'a> {
    /// Fixed amounts displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<&'a [i64]>,
    /// Percentages displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<&'a [i64]>,
    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}
impl<'a> CurrencySpecificConfig<'a> {
    pub fn new() -> Self {
        Self { fixed_amounts: None, percentages: None, smart_tip_threshold: None }
    }
}
impl<'a> Default for CurrencySpecificConfig<'a> {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct Tipping<'a> {
    /// Tipping configuration for AUD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aud: Option<CurrencySpecificConfig<'a>>,
    /// Tipping configuration for CAD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cad: Option<CurrencySpecificConfig<'a>>,
    /// Tipping configuration for CHF
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chf: Option<CurrencySpecificConfig<'a>>,
    /// Tipping configuration for CZK
    #[serde(skip_serializing_if = "Option::is_none")]
    pub czk: Option<CurrencySpecificConfig<'a>>,
    /// Tipping configuration for DKK
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkk: Option<CurrencySpecificConfig<'a>>,
    /// Tipping configuration for EUR
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eur: Option<CurrencySpecificConfig<'a>>,
    /// Tipping configuration for GBP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gbp: Option<CurrencySpecificConfig<'a>>,
    /// Tipping configuration for HKD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hkd: Option<CurrencySpecificConfig<'a>>,
    /// Tipping configuration for MYR
    #[serde(skip_serializing_if = "Option::is_none")]
    pub myr: Option<CurrencySpecificConfig<'a>>,
    /// Tipping configuration for NOK
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nok: Option<CurrencySpecificConfig<'a>>,
    /// Tipping configuration for NZD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nzd: Option<CurrencySpecificConfig<'a>>,
    /// Tipping configuration for SEK
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sek: Option<CurrencySpecificConfig<'a>>,
    /// Tipping configuration for SGD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sgd: Option<CurrencySpecificConfig<'a>>,
    /// Tipping configuration for USD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usd: Option<CurrencySpecificConfig<'a>>,
}
impl<'a> Tipping<'a> {
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
            myr: None,
            nok: None,
            nzd: None,
            sek: None,
            sgd: None,
            usd: None,
        }
    }
}
impl<'a> Default for Tipping<'a> {
    fn default() -> Self {
        Self::new()
    }
}

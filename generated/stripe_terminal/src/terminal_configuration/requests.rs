#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct DeleteTerminalConfiguration {}
impl DeleteTerminalConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
impl DeleteTerminalConfiguration {
    /// Deletes a `Configuration` object.
    pub fn send(
        &self,
        client: &stripe::Client,
        configuration: &stripe_terminal::TerminalConfigurationId,
    ) -> stripe::Response<stripe_terminal::DeletedTerminalConfiguration> {
        client.send_form(
            &format!("/terminal/configurations/{configuration}"),
            self,
            http_types::Method::Delete,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListTerminalConfiguration<'a> {
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// if present, only return the account default or non-default configurations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_account_default: Option<bool>,
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}
impl<'a> ListTerminalConfiguration<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ListTerminalConfiguration<'a> {
    /// Returns a list of `Configuration` objects.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_terminal::TerminalConfiguration>> {
        client.get_query("/terminal/configurations", self)
    }
    pub fn paginate(
        self,
    ) -> stripe::ListPaginator<stripe_types::List<stripe_terminal::TerminalConfiguration>> {
        stripe::ListPaginator::from_list_params("/terminal/configurations", self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveTerminalConfiguration<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTerminalConfiguration<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveTerminalConfiguration<'a> {
    /// Retrieves a `Configuration` object.
    pub fn send(
        &self,
        client: &stripe::Client,
        configuration: &stripe_terminal::TerminalConfigurationId,
    ) -> stripe::Response<RetrieveTerminalConfigurationReturned> {
        client.get_query(&format!("/terminal/configurations/{configuration}"), self)
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum RetrieveTerminalConfigurationReturned {
    TerminalConfiguration(stripe_terminal::TerminalConfiguration),
    DeletedTerminalConfiguration(stripe_terminal::DeletedTerminalConfiguration),
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTerminalConfiguration<'a> {
    /// An object containing device type specific settings for BBPOS WisePOS E readers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbpos_wisepos_e: Option<CreateTerminalConfigurationBbposWiseposE<'a>>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Configurations for collecting transactions offline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<Offline>,
    /// Tipping configurations for readers supporting on-reader tips
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipping: Option<Tipping<'a>>,
    /// An object containing device type specific settings for Verifone P400 readers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verifone_p400: Option<CreateTerminalConfigurationVerifoneP400<'a>>,
}
impl<'a> CreateTerminalConfiguration<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// An object containing device type specific settings for BBPOS WisePOS E readers
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTerminalConfigurationBbposWiseposE<'a> {
    /// A File ID representing an image you would like displayed on the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splashscreen: Option<&'a str>,
}
impl<'a> CreateTerminalConfigurationBbposWiseposE<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// An object containing device type specific settings for Verifone P400 readers
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTerminalConfigurationVerifoneP400<'a> {
    /// A File ID representing an image you would like displayed on the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splashscreen: Option<&'a str>,
}
impl<'a> CreateTerminalConfigurationVerifoneP400<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> CreateTerminalConfiguration<'a> {
    /// Creates a new `Configuration` object.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_terminal::TerminalConfiguration> {
        client.send_form("/terminal/configurations", self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTerminalConfiguration<'a> {
    /// An object containing device type specific settings for BBPOS WisePOS E readers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbpos_wisepos_e: Option<UpdateTerminalConfigurationBbposWiseposE<'a>>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Configurations for collecting transactions offline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<Offline>,
    /// Tipping configurations for readers supporting on-reader tips
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipping: Option<Tipping<'a>>,
    /// An object containing device type specific settings for Verifone P400 readers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verifone_p400: Option<UpdateTerminalConfigurationVerifoneP400<'a>>,
}
impl<'a> UpdateTerminalConfiguration<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// An object containing device type specific settings for BBPOS WisePOS E readers
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTerminalConfigurationBbposWiseposE<'a> {
    /// A File ID representing an image you would like displayed on the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splashscreen: Option<&'a str>,
}
impl<'a> UpdateTerminalConfigurationBbposWiseposE<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// An object containing device type specific settings for Verifone P400 readers
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTerminalConfigurationVerifoneP400<'a> {
    /// A File ID representing an image you would like displayed on the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splashscreen: Option<&'a str>,
}
impl<'a> UpdateTerminalConfigurationVerifoneP400<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> UpdateTerminalConfiguration<'a> {
    /// Updates a new `Configuration` object.
    pub fn send(
        &self,
        client: &stripe::Client,
        configuration: &stripe_terminal::TerminalConfigurationId,
    ) -> stripe::Response<UpdateTerminalConfigurationReturned> {
        client.send_form(
            &format!("/terminal/configurations/{configuration}"),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum UpdateTerminalConfigurationReturned {
    TerminalConfiguration(stripe_terminal::TerminalConfiguration),
    DeletedTerminalConfiguration(stripe_terminal::DeletedTerminalConfiguration),
}
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
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
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
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
        Self::default()
    }
}

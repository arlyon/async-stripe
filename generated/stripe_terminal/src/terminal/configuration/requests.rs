
/// Creates a new `Configuration` object.
pub fn create(
    client: &stripe::Client,
    params: CreateConfiguration,
) -> stripe::Response<stripe_terminal::terminal::configuration::Configuration> {
    client.send_form("/terminal/configurations", params, http_types::Method::Post)
}
/// Returns a list of `Configuration` objects.
pub fn list(
    client: &stripe::Client,
    params: ListConfiguration,
) -> stripe::Response<stripe_types::List<stripe_terminal::terminal::configuration::Configuration>> {
    client.get_query("/terminal/configurations", params)
}
/// Retrieves a `Configuration` object.
pub fn retrieve(
    client: &stripe::Client,
    configuration: &stripe_terminal::terminal::configuration::TerminalConfigurationId,
    params: RetrieveConfiguration,
) -> stripe::Response<RetrieveReturned> {
    client.get_query(
        &format!("/terminal/configurations/{configuration}", configuration = configuration),
        params,
    )
}
/// Updates a new `Configuration` object.
pub fn update(
    client: &stripe::Client,
    configuration: &stripe_terminal::terminal::configuration::TerminalConfigurationId,
    params: UpdateConfiguration,
) -> stripe::Response<UpdateReturned> {
    client.send_form(
        &format!("/terminal/configurations/{configuration}", configuration = configuration),
        params,
        http_types::Method::Post,
    )
}
/// Deletes a `Configuration` object.
pub fn delete(
    client: &stripe::Client,
    configuration: &stripe_terminal::terminal::configuration::TerminalConfigurationId,
) -> stripe::Response<stripe_terminal::terminal::configuration::DeletedConfiguration> {
    client.send(
        &format!("/terminal/configurations/{configuration}", configuration = configuration),
        http_types::Method::Delete,
    )
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateConfiguration<'a> {
    /// An object containing device type specific settings for BBPOS WisePOS E readers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbpos_wisepos_e: Option<CreateConfigurationBbposWiseposE<'a>>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Tipping configurations for readers supporting on-reader tips.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipping: Option<Tipping<'a>>,
    /// An object containing device type specific settings for Verifone P400 readers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verifone_p400: Option<CreateConfigurationVerifoneP400<'a>>,
}
impl<'a> CreateConfiguration<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// An object containing device type specific settings for BBPOS WisePOS E readers.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateConfigurationBbposWiseposE<'a> {
    /// A File ID representing an image you would like displayed on the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splashscreen: Option<&'a str>,
}
impl<'a> CreateConfigurationBbposWiseposE<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// An object containing device type specific settings for Verifone P400 readers.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateConfigurationVerifoneP400<'a> {
    /// A File ID representing an image you would like displayed on the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splashscreen: Option<&'a str>,
}
impl<'a> CreateConfigurationVerifoneP400<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListConfiguration<'a> {
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// if present, only return the account default or non-default configurations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_account_default: Option<bool>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}
impl<'a> ListConfiguration<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum RetrieveReturned {
    TerminalConfiguration(stripe_terminal::terminal::configuration::Configuration),
    DeletedTerminalConfiguration(stripe_terminal::terminal::configuration::DeletedConfiguration),
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveConfiguration<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveConfiguration<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum UpdateReturned {
    TerminalConfiguration(stripe_terminal::terminal::configuration::Configuration),
    DeletedTerminalConfiguration(stripe_terminal::terminal::configuration::DeletedConfiguration),
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateConfiguration<'a> {
    /// An object containing device type specific settings for BBPOS WisePOS E readers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbpos_wisepos_e: Option<UpdateConfigurationBbposWiseposE<'a>>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Tipping configurations for readers supporting on-reader tips.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipping: Option<Tipping<'a>>,
    /// An object containing device type specific settings for Verifone P400 readers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verifone_p400: Option<UpdateConfigurationVerifoneP400<'a>>,
}
impl<'a> UpdateConfiguration<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// An object containing device type specific settings for BBPOS WisePOS E readers.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateConfigurationBbposWiseposE<'a> {
    /// A File ID representing an image you would like displayed on the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splashscreen: Option<&'a str>,
}
impl<'a> UpdateConfigurationBbposWiseposE<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// An object containing device type specific settings for Verifone P400 readers.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateConfigurationVerifoneP400<'a> {
    /// A File ID representing an image you would like displayed on the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splashscreen: Option<&'a str>,
}
impl<'a> UpdateConfigurationVerifoneP400<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CurrencySpecificConfig<'a> {
    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<&'a [i64]>,
    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<&'a [i64]>,
    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
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
    /// Tipping configuration for AUD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aud: Option<CurrencySpecificConfig<'a>>,
    /// Tipping configuration for CAD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cad: Option<CurrencySpecificConfig<'a>>,
    /// Tipping configuration for CHF.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chf: Option<CurrencySpecificConfig<'a>>,
    /// Tipping configuration for CZK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub czk: Option<CurrencySpecificConfig<'a>>,
    /// Tipping configuration for DKK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkk: Option<CurrencySpecificConfig<'a>>,
    /// Tipping configuration for EUR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eur: Option<CurrencySpecificConfig<'a>>,
    /// Tipping configuration for GBP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gbp: Option<CurrencySpecificConfig<'a>>,
    /// Tipping configuration for HKD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hkd: Option<CurrencySpecificConfig<'a>>,
    /// Tipping configuration for MYR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub myr: Option<CurrencySpecificConfig<'a>>,
    /// Tipping configuration for NOK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nok: Option<CurrencySpecificConfig<'a>>,
    /// Tipping configuration for NZD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nzd: Option<CurrencySpecificConfig<'a>>,
    /// Tipping configuration for SEK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sek: Option<CurrencySpecificConfig<'a>>,
    /// Tipping configuration for SGD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sgd: Option<CurrencySpecificConfig<'a>>,
    /// Tipping configuration for USD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usd: Option<CurrencySpecificConfig<'a>>,
}
impl<'a> Tipping<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}

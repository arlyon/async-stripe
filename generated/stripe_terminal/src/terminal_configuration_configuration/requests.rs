#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTerminalConfigurationConfiguration<'a> {
    /// An object containing device type specific settings for BBPOS WisePOS E readers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbpos_wisepos_e: Option<CreateTerminalConfigurationConfigurationBbposWiseposE<'a>>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Configurations for collecting transactions offline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<CreateTerminalConfigurationConfigurationOffline>,
    /// Tipping configurations for readers supporting on-reader tips.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipping: Option<CreateTerminalConfigurationConfigurationTipping<'a>>,
    /// An object containing device type specific settings for Verifone P400 readers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verifone_p400: Option<CreateTerminalConfigurationConfigurationVerifoneP400<'a>>,
}
impl<'a> CreateTerminalConfigurationConfiguration<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// An object containing device type specific settings for BBPOS WisePOS E readers.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTerminalConfigurationConfigurationBbposWiseposE<'a> {
    /// A File ID representing an image you would like displayed on the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splashscreen: Option<&'a str>,
}
impl<'a> CreateTerminalConfigurationConfigurationBbposWiseposE<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Configurations for collecting transactions offline.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTerminalConfigurationConfigurationOffline {
    /// Determines whether to allow transactions to be collected while reader is offline.
    ///
    /// Defaults to false.
    pub enabled: bool,
}
impl CreateTerminalConfigurationConfigurationOffline {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// Tipping configurations for readers supporting on-reader tips.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTerminalConfigurationConfigurationTipping<'a> {
    /// Tipping configuration for AUD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aud: Option<CreateTerminalConfigurationConfigurationTippingAud<'a>>,
    /// Tipping configuration for CAD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cad: Option<CreateTerminalConfigurationConfigurationTippingCad<'a>>,
    /// Tipping configuration for CHF.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chf: Option<CreateTerminalConfigurationConfigurationTippingChf<'a>>,
    /// Tipping configuration for CZK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub czk: Option<CreateTerminalConfigurationConfigurationTippingCzk<'a>>,
    /// Tipping configuration for DKK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkk: Option<CreateTerminalConfigurationConfigurationTippingDkk<'a>>,
    /// Tipping configuration for EUR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eur: Option<CreateTerminalConfigurationConfigurationTippingEur<'a>>,
    /// Tipping configuration for GBP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gbp: Option<CreateTerminalConfigurationConfigurationTippingGbp<'a>>,
    /// Tipping configuration for HKD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hkd: Option<CreateTerminalConfigurationConfigurationTippingHkd<'a>>,
    /// Tipping configuration for MYR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub myr: Option<CreateTerminalConfigurationConfigurationTippingMyr<'a>>,
    /// Tipping configuration for NOK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nok: Option<CreateTerminalConfigurationConfigurationTippingNok<'a>>,
    /// Tipping configuration for NZD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nzd: Option<CreateTerminalConfigurationConfigurationTippingNzd<'a>>,
    /// Tipping configuration for SEK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sek: Option<CreateTerminalConfigurationConfigurationTippingSek<'a>>,
    /// Tipping configuration for SGD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sgd: Option<CreateTerminalConfigurationConfigurationTippingSgd<'a>>,
    /// Tipping configuration for USD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usd: Option<CreateTerminalConfigurationConfigurationTippingUsd<'a>>,
}
impl<'a> CreateTerminalConfigurationConfigurationTipping<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for AUD.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTerminalConfigurationConfigurationTippingAud<'a> {
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
impl<'a> CreateTerminalConfigurationConfigurationTippingAud<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for CAD.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTerminalConfigurationConfigurationTippingCad<'a> {
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
impl<'a> CreateTerminalConfigurationConfigurationTippingCad<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for CHF.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTerminalConfigurationConfigurationTippingChf<'a> {
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
impl<'a> CreateTerminalConfigurationConfigurationTippingChf<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for CZK.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTerminalConfigurationConfigurationTippingCzk<'a> {
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
impl<'a> CreateTerminalConfigurationConfigurationTippingCzk<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for DKK.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTerminalConfigurationConfigurationTippingDkk<'a> {
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
impl<'a> CreateTerminalConfigurationConfigurationTippingDkk<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for EUR.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTerminalConfigurationConfigurationTippingEur<'a> {
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
impl<'a> CreateTerminalConfigurationConfigurationTippingEur<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for GBP.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTerminalConfigurationConfigurationTippingGbp<'a> {
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
impl<'a> CreateTerminalConfigurationConfigurationTippingGbp<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for HKD.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTerminalConfigurationConfigurationTippingHkd<'a> {
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
impl<'a> CreateTerminalConfigurationConfigurationTippingHkd<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for MYR.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTerminalConfigurationConfigurationTippingMyr<'a> {
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
impl<'a> CreateTerminalConfigurationConfigurationTippingMyr<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for NOK.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTerminalConfigurationConfigurationTippingNok<'a> {
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
impl<'a> CreateTerminalConfigurationConfigurationTippingNok<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for NZD.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTerminalConfigurationConfigurationTippingNzd<'a> {
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
impl<'a> CreateTerminalConfigurationConfigurationTippingNzd<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for SEK.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTerminalConfigurationConfigurationTippingSek<'a> {
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
impl<'a> CreateTerminalConfigurationConfigurationTippingSek<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for SGD.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTerminalConfigurationConfigurationTippingSgd<'a> {
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
impl<'a> CreateTerminalConfigurationConfigurationTippingSgd<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for USD.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTerminalConfigurationConfigurationTippingUsd<'a> {
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
impl<'a> CreateTerminalConfigurationConfigurationTippingUsd<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// An object containing device type specific settings for Verifone P400 readers.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTerminalConfigurationConfigurationVerifoneP400<'a> {
    /// A File ID representing an image you would like displayed on the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splashscreen: Option<&'a str>,
}
impl<'a> CreateTerminalConfigurationConfigurationVerifoneP400<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> CreateTerminalConfigurationConfiguration<'a> {
    /// Creates a new `Configuration` object.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_terminal::TerminalConfigurationConfiguration> {
        client.send_form("/terminal/configurations", self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListTerminalConfigurationConfiguration<'a> {
    /// A cursor for use in pagination.
    ///
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
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}
impl<'a> ListTerminalConfigurationConfiguration<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ListTerminalConfigurationConfiguration<'a> {
    /// Returns a list of `Configuration` objects.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_terminal::TerminalConfigurationConfiguration>>
    {
        client.get_query("/terminal/configurations", self)
    }
    pub fn paginate(
        self,
    ) -> stripe::ListPaginator<stripe_terminal::TerminalConfigurationConfiguration> {
        stripe::ListPaginator::from_params("/terminal/configurations", self)
    }
}
impl<'a> stripe::PaginationParams for ListTerminalConfigurationConfiguration<'a> {}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveTerminalConfigurationConfiguration<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTerminalConfigurationConfiguration<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveTerminalConfigurationConfiguration<'a> {
    /// Retrieves a `Configuration` object.
    pub fn send(
        &self,
        client: &stripe::Client,
        configuration: &str,
    ) -> stripe::Response<RetrieveReturned> {
        client.get_query(&format!("/terminal/configurations/{configuration}"), self)
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "object")]
pub enum RetrieveReturned {
    #[serde(rename = "terminal.configuration")]
    TerminalConfigurationDeletedConfiguration(
        stripe_terminal::TerminalConfigurationDeletedConfiguration,
    ),
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTerminalConfigurationConfiguration<'a> {
    /// An object containing device type specific settings for BBPOS WisePOS E readers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbpos_wisepos_e: Option<UpdateTerminalConfigurationConfigurationBbposWiseposE<'a>>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Configurations for collecting transactions offline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<UpdateTerminalConfigurationConfigurationOffline>,
    /// Tipping configurations for readers supporting on-reader tips.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipping: Option<UpdateTerminalConfigurationConfigurationTipping<'a>>,
    /// An object containing device type specific settings for Verifone P400 readers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verifone_p400: Option<UpdateTerminalConfigurationConfigurationVerifoneP400<'a>>,
}
impl<'a> UpdateTerminalConfigurationConfiguration<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// An object containing device type specific settings for BBPOS WisePOS E readers.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTerminalConfigurationConfigurationBbposWiseposE<'a> {
    /// A File ID representing an image you would like displayed on the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splashscreen: Option<&'a str>,
}
impl<'a> UpdateTerminalConfigurationConfigurationBbposWiseposE<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Configurations for collecting transactions offline.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateTerminalConfigurationConfigurationOffline {
    /// Determines whether to allow transactions to be collected while reader is offline.
    ///
    /// Defaults to false.
    pub enabled: bool,
}
impl UpdateTerminalConfigurationConfigurationOffline {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// Tipping configurations for readers supporting on-reader tips.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTerminalConfigurationConfigurationTipping<'a> {
    /// Tipping configuration for AUD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aud: Option<UpdateTerminalConfigurationConfigurationTippingAud<'a>>,
    /// Tipping configuration for CAD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cad: Option<UpdateTerminalConfigurationConfigurationTippingCad<'a>>,
    /// Tipping configuration for CHF.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chf: Option<UpdateTerminalConfigurationConfigurationTippingChf<'a>>,
    /// Tipping configuration for CZK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub czk: Option<UpdateTerminalConfigurationConfigurationTippingCzk<'a>>,
    /// Tipping configuration for DKK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkk: Option<UpdateTerminalConfigurationConfigurationTippingDkk<'a>>,
    /// Tipping configuration for EUR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eur: Option<UpdateTerminalConfigurationConfigurationTippingEur<'a>>,
    /// Tipping configuration for GBP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gbp: Option<UpdateTerminalConfigurationConfigurationTippingGbp<'a>>,
    /// Tipping configuration for HKD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hkd: Option<UpdateTerminalConfigurationConfigurationTippingHkd<'a>>,
    /// Tipping configuration for MYR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub myr: Option<UpdateTerminalConfigurationConfigurationTippingMyr<'a>>,
    /// Tipping configuration for NOK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nok: Option<UpdateTerminalConfigurationConfigurationTippingNok<'a>>,
    /// Tipping configuration for NZD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nzd: Option<UpdateTerminalConfigurationConfigurationTippingNzd<'a>>,
    /// Tipping configuration for SEK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sek: Option<UpdateTerminalConfigurationConfigurationTippingSek<'a>>,
    /// Tipping configuration for SGD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sgd: Option<UpdateTerminalConfigurationConfigurationTippingSgd<'a>>,
    /// Tipping configuration for USD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usd: Option<UpdateTerminalConfigurationConfigurationTippingUsd<'a>>,
}
impl<'a> UpdateTerminalConfigurationConfigurationTipping<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for AUD.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTerminalConfigurationConfigurationTippingAud<'a> {
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
impl<'a> UpdateTerminalConfigurationConfigurationTippingAud<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for CAD.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTerminalConfigurationConfigurationTippingCad<'a> {
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
impl<'a> UpdateTerminalConfigurationConfigurationTippingCad<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for CHF.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTerminalConfigurationConfigurationTippingChf<'a> {
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
impl<'a> UpdateTerminalConfigurationConfigurationTippingChf<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for CZK.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTerminalConfigurationConfigurationTippingCzk<'a> {
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
impl<'a> UpdateTerminalConfigurationConfigurationTippingCzk<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for DKK.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTerminalConfigurationConfigurationTippingDkk<'a> {
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
impl<'a> UpdateTerminalConfigurationConfigurationTippingDkk<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for EUR.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTerminalConfigurationConfigurationTippingEur<'a> {
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
impl<'a> UpdateTerminalConfigurationConfigurationTippingEur<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for GBP.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTerminalConfigurationConfigurationTippingGbp<'a> {
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
impl<'a> UpdateTerminalConfigurationConfigurationTippingGbp<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for HKD.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTerminalConfigurationConfigurationTippingHkd<'a> {
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
impl<'a> UpdateTerminalConfigurationConfigurationTippingHkd<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for MYR.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTerminalConfigurationConfigurationTippingMyr<'a> {
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
impl<'a> UpdateTerminalConfigurationConfigurationTippingMyr<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for NOK.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTerminalConfigurationConfigurationTippingNok<'a> {
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
impl<'a> UpdateTerminalConfigurationConfigurationTippingNok<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for NZD.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTerminalConfigurationConfigurationTippingNzd<'a> {
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
impl<'a> UpdateTerminalConfigurationConfigurationTippingNzd<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for SEK.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTerminalConfigurationConfigurationTippingSek<'a> {
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
impl<'a> UpdateTerminalConfigurationConfigurationTippingSek<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for SGD.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTerminalConfigurationConfigurationTippingSgd<'a> {
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
impl<'a> UpdateTerminalConfigurationConfigurationTippingSgd<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for USD.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTerminalConfigurationConfigurationTippingUsd<'a> {
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
impl<'a> UpdateTerminalConfigurationConfigurationTippingUsd<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// An object containing device type specific settings for Verifone P400 readers.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTerminalConfigurationConfigurationVerifoneP400<'a> {
    /// A File ID representing an image you would like displayed on the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splashscreen: Option<&'a str>,
}
impl<'a> UpdateTerminalConfigurationConfigurationVerifoneP400<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> UpdateTerminalConfigurationConfiguration<'a> {
    /// Updates a new `Configuration` object.
    pub fn send(
        &self,
        client: &stripe::Client,
        configuration: &str,
    ) -> stripe::Response<UpdateReturned> {
        client.send_form(
            &format!("/terminal/configurations/{configuration}"),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "object")]
pub enum UpdateReturned {
    #[serde(rename = "terminal.configuration")]
    TerminalConfigurationDeletedConfiguration(
        stripe_terminal::TerminalConfigurationDeletedConfiguration,
    ),
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct DeleteTerminalConfigurationConfiguration {}
impl DeleteTerminalConfigurationConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
impl DeleteTerminalConfigurationConfiguration {
    /// Deletes a `Configuration` object.
    pub fn send(
        &self,
        client: &stripe::Client,
        configuration: &str,
    ) -> stripe::Response<stripe_terminal::TerminalConfigurationDeletedConfiguration> {
        client.send_form(
            &format!("/terminal/configurations/{configuration}"),
            self,
            http_types::Method::Delete,
        )
    }
}

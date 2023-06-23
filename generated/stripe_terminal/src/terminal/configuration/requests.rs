use stripe::{Client, Response};

impl stripe_terminal::terminal::configuration::Configuration {
    /// Creates a new `Configuration` object.
    pub fn create(
        client: &Client,
        params: CreateConfiguration,
    ) -> Response<stripe_terminal::terminal::configuration::Configuration> {
        client.send_form("/terminal/configurations", params, http_types::Method::Post)
    }
    /// Returns a list of `Configuration` objects.
    pub fn list(
        client: &Client,
        params: ListConfiguration,
    ) -> Response<stripe_types::List<stripe_terminal::terminal::configuration::Configuration>> {
        client.get_query("/terminal/configurations", params)
    }
    /// Retrieves a `Configuration` object.
    pub fn retrieve(
        client: &Client,
        configuration: &str,
        params: RetrieveConfiguration,
    ) -> Response<RetrieveReturned> {
        client.get_query(
            &format!("/terminal/configurations/{configuration}", configuration = configuration),
            params,
        )
    }
    /// Updates a new `Configuration` object.
    pub fn update(
        client: &Client,
        configuration: &str,
        params: UpdateConfiguration,
    ) -> Response<UpdateReturned> {
        client.send_form(
            &format!("/terminal/configurations/{configuration}", configuration = configuration),
            params,
            http_types::Method::Post,
        )
    }
    /// Deletes a `Configuration` object.
    pub fn delete(
        client: &Client,
        configuration: &str,
    ) -> Response<stripe_terminal::terminal::configuration::DeletedConfiguration> {
        client.send(
            &format!("/terminal/configurations/{configuration}", configuration = configuration),
            http_types::Method::Delete,
        )
    }
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
    pub tipping: Option<CreateConfigurationTipping<'a>>,
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
/// Tipping configurations for readers supporting on-reader tips.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateConfigurationTipping<'a> {
    /// Tipping configuration for AUD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aud: Option<CreateConfigurationTippingAud<'a>>,
    /// Tipping configuration for CAD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cad: Option<CreateConfigurationTippingCad<'a>>,
    /// Tipping configuration for CHF.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chf: Option<CreateConfigurationTippingChf<'a>>,
    /// Tipping configuration for CZK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub czk: Option<CreateConfigurationTippingCzk<'a>>,
    /// Tipping configuration for DKK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkk: Option<CreateConfigurationTippingDkk<'a>>,
    /// Tipping configuration for EUR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eur: Option<CreateConfigurationTippingEur<'a>>,
    /// Tipping configuration for GBP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gbp: Option<CreateConfigurationTippingGbp<'a>>,
    /// Tipping configuration for HKD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hkd: Option<CreateConfigurationTippingHkd<'a>>,
    /// Tipping configuration for MYR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub myr: Option<CreateConfigurationTippingMyr<'a>>,
    /// Tipping configuration for NOK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nok: Option<CreateConfigurationTippingNok<'a>>,
    /// Tipping configuration for NZD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nzd: Option<CreateConfigurationTippingNzd<'a>>,
    /// Tipping configuration for SEK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sek: Option<CreateConfigurationTippingSek<'a>>,
    /// Tipping configuration for SGD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sgd: Option<CreateConfigurationTippingSgd<'a>>,
    /// Tipping configuration for USD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usd: Option<CreateConfigurationTippingUsd<'a>>,
}
impl<'a> CreateConfigurationTipping<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for AUD.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateConfigurationTippingAud<'a> {
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
impl<'a> CreateConfigurationTippingAud<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for CAD.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateConfigurationTippingCad<'a> {
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
impl<'a> CreateConfigurationTippingCad<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for CHF.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateConfigurationTippingChf<'a> {
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
impl<'a> CreateConfigurationTippingChf<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for CZK.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateConfigurationTippingCzk<'a> {
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
impl<'a> CreateConfigurationTippingCzk<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for DKK.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateConfigurationTippingDkk<'a> {
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
impl<'a> CreateConfigurationTippingDkk<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for EUR.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateConfigurationTippingEur<'a> {
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
impl<'a> CreateConfigurationTippingEur<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for GBP.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateConfigurationTippingGbp<'a> {
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
impl<'a> CreateConfigurationTippingGbp<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for HKD.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateConfigurationTippingHkd<'a> {
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
impl<'a> CreateConfigurationTippingHkd<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for MYR.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateConfigurationTippingMyr<'a> {
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
impl<'a> CreateConfigurationTippingMyr<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for NOK.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateConfigurationTippingNok<'a> {
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
impl<'a> CreateConfigurationTippingNok<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for NZD.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateConfigurationTippingNzd<'a> {
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
impl<'a> CreateConfigurationTippingNzd<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for SEK.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateConfigurationTippingSek<'a> {
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
impl<'a> CreateConfigurationTippingSek<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for SGD.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateConfigurationTippingSgd<'a> {
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
impl<'a> CreateConfigurationTippingSgd<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for USD.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateConfigurationTippingUsd<'a> {
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
impl<'a> CreateConfigurationTippingUsd<'a> {
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
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[serde(untagged, rename_all = "snake_case")]
pub enum RetrieveReturned {
    TerminalConfiguration(stripe_terminal::terminal::configuration::Configuration),
    DeletedTerminalConfiguration(stripe_terminal::terminal::configuration::DeletedConfiguration),
}
#[cfg(feature = "min-ser")]
impl crate::deser::StripeDeserialize for RetrieveReturned {
    fn deserialize(str: &str) -> Result<Self, crate::StripeError> {
        use miniserde::json::from_str;

        use crate::deser::StripeDeserialize;
        let maybe_deleted: crate::deser::MaybeDeleted = from_str(str)?;
        let deleted = maybe_deleted.deleted.unwrap_or(false);
        if deleted {
            Ok(Self::DeletedTerminalConfiguration(StripeDeserialize::deserialize(str)?))
        } else {
            Ok(Self::TerminalConfiguration(StripeDeserialize::deserialize(str)?))
        }
    }
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
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[serde(untagged, rename_all = "snake_case")]
pub enum UpdateReturned {
    TerminalConfiguration(stripe_terminal::terminal::configuration::Configuration),
    DeletedTerminalConfiguration(stripe_terminal::terminal::configuration::DeletedConfiguration),
}
#[cfg(feature = "min-ser")]
impl crate::deser::StripeDeserialize for UpdateReturned {
    fn deserialize(str: &str) -> Result<Self, crate::StripeError> {
        use miniserde::json::from_str;

        use crate::deser::StripeDeserialize;
        let maybe_deleted: crate::deser::MaybeDeleted = from_str(str)?;
        let deleted = maybe_deleted.deleted.unwrap_or(false);
        if deleted {
            Ok(Self::DeletedTerminalConfiguration(StripeDeserialize::deserialize(str)?))
        } else {
            Ok(Self::TerminalConfiguration(StripeDeserialize::deserialize(str)?))
        }
    }
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
    pub tipping: Option<UpdateConfigurationTipping<'a>>,
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
/// Tipping configurations for readers supporting on-reader tips.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateConfigurationTipping<'a> {
    /// Tipping configuration for AUD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aud: Option<UpdateConfigurationTippingAud<'a>>,
    /// Tipping configuration for CAD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cad: Option<UpdateConfigurationTippingCad<'a>>,
    /// Tipping configuration for CHF.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chf: Option<UpdateConfigurationTippingChf<'a>>,
    /// Tipping configuration for CZK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub czk: Option<UpdateConfigurationTippingCzk<'a>>,
    /// Tipping configuration for DKK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkk: Option<UpdateConfigurationTippingDkk<'a>>,
    /// Tipping configuration for EUR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eur: Option<UpdateConfigurationTippingEur<'a>>,
    /// Tipping configuration for GBP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gbp: Option<UpdateConfigurationTippingGbp<'a>>,
    /// Tipping configuration for HKD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hkd: Option<UpdateConfigurationTippingHkd<'a>>,
    /// Tipping configuration for MYR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub myr: Option<UpdateConfigurationTippingMyr<'a>>,
    /// Tipping configuration for NOK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nok: Option<UpdateConfigurationTippingNok<'a>>,
    /// Tipping configuration for NZD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nzd: Option<UpdateConfigurationTippingNzd<'a>>,
    /// Tipping configuration for SEK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sek: Option<UpdateConfigurationTippingSek<'a>>,
    /// Tipping configuration for SGD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sgd: Option<UpdateConfigurationTippingSgd<'a>>,
    /// Tipping configuration for USD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usd: Option<UpdateConfigurationTippingUsd<'a>>,
}
impl<'a> UpdateConfigurationTipping<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for AUD.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateConfigurationTippingAud<'a> {
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
impl<'a> UpdateConfigurationTippingAud<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for CAD.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateConfigurationTippingCad<'a> {
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
impl<'a> UpdateConfigurationTippingCad<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for CHF.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateConfigurationTippingChf<'a> {
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
impl<'a> UpdateConfigurationTippingChf<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for CZK.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateConfigurationTippingCzk<'a> {
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
impl<'a> UpdateConfigurationTippingCzk<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for DKK.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateConfigurationTippingDkk<'a> {
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
impl<'a> UpdateConfigurationTippingDkk<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for EUR.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateConfigurationTippingEur<'a> {
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
impl<'a> UpdateConfigurationTippingEur<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for GBP.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateConfigurationTippingGbp<'a> {
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
impl<'a> UpdateConfigurationTippingGbp<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for HKD.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateConfigurationTippingHkd<'a> {
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
impl<'a> UpdateConfigurationTippingHkd<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for MYR.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateConfigurationTippingMyr<'a> {
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
impl<'a> UpdateConfigurationTippingMyr<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for NOK.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateConfigurationTippingNok<'a> {
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
impl<'a> UpdateConfigurationTippingNok<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for NZD.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateConfigurationTippingNzd<'a> {
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
impl<'a> UpdateConfigurationTippingNzd<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for SEK.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateConfigurationTippingSek<'a> {
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
impl<'a> UpdateConfigurationTippingSek<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for SGD.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateConfigurationTippingSgd<'a> {
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
impl<'a> UpdateConfigurationTippingSgd<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for USD.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateConfigurationTippingUsd<'a> {
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
impl<'a> UpdateConfigurationTippingUsd<'a> {
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

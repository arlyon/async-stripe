#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct BusinessProfile {
    /// [The merchant category code for the account](https://stripe.com/docs/connect/setting-mcc).
    ///
    /// MCCs are used to classify businesses based on the goods or services they provide.
    pub mcc: Option<String>,
    /// The customer-facing business name.
    pub name: Option<String>,
    /// Internal-only description of the product sold or service provided by the business.
    ///
    /// It's used by Stripe for risk and underwriting purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,
    /// A publicly available mailing address for sending support issues to.
    pub support_address: Option<crate::address::Address>,
    /// A publicly available email address for sending support issues to.
    pub support_email: Option<String>,
    /// A publicly available phone number to call with support issues.
    pub support_phone: Option<String>,
    /// A publicly available website for handling support issues.
    pub support_url: Option<String>,
    /// The business's publicly available website.
    pub url: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for BusinessProfile {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

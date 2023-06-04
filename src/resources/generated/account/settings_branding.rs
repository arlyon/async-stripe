#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SettingsBranding {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) An icon for the account.
    ///
    /// Must be square and at least 128px x 128px.
    pub icon: Option<crate::Expandable<crate::file::File>>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) A logo for the account that will be used in Checkout instead of the icon and without the account's name next to it if provided.
    ///
    /// Must be at least 128px x 128px.
    pub logo: Option<crate::Expandable<crate::file::File>>,
    /// A CSS hex color value representing the primary branding color for this account.
    pub primary_color: Option<String>,
    /// A CSS hex color value representing the secondary branding color for this account.
    pub secondary_color: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SettingsBranding {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct LoginPage {
    /// If `true`, a shareable `url` will be generated that will take your customers to a hosted login page for the customer portal.
    ///
    /// If `false`, the previously generated `url`, if any, will be deactivated.
    pub enabled: bool,
    /// A shareable URL to the hosted portal login page.
    ///
    /// Your customers will be able to log in with their [email](https://stripe.com/docs/api/customers/object#customer_object-email) and receive a link to their customer portal.
    pub url: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for LoginPage {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
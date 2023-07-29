#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct OwnershipDeclaration {
    /// The Unix timestamp marking when the beneficial owner attestation was made.
    pub date: Option<stripe_types::Timestamp>,
    /// The IP address from which the beneficial owner attestation was made.
    pub ip: Option<String>,
    /// The user-agent string from the browser where the beneficial owner attestation was made.
    pub user_agent: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for OwnershipDeclaration {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

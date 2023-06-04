#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct AfterExpiration {
    /// When set, configuration used to recover the Checkout Session on expiry.
    pub recovery: Option<crate::checkout::session::after_expiration::recovery::Recovery>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AfterExpiration {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

pub mod recovery;
pub use recovery::Recovery;

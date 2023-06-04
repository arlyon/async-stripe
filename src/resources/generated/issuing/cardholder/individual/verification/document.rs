#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Document {
    /// The back of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    pub back: Option<crate::Expandable<crate::file::File>>,
    /// The front of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    pub front: Option<crate::Expandable<crate::file::File>>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Document {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Document {
    /// The back of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    pub back: Option<stripe_types::Expandable<stripe_types::file::File>>,
    /// The front of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    pub front: Option<stripe_types::Expandable<stripe_types::file::File>>,
}

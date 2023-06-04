/// The Pause Collection settings determine how we will pause collection for this subscription and for how long the subscription
/// should be paused.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PauseCollection {
    /// The payment collection behavior for this subscription while paused.
    ///
    /// One of `keep_as_draft`, `mark_uncollectible`, or `void`.
    pub behavior: PauseCollectionBehavior,
    /// The time after which the subscription will resume collecting payments.
    pub resumes_at: Option<crate::Timestamp>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PauseCollection {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The payment collection behavior for this subscription while paused.
///
/// One of `keep_as_draft`, `mark_uncollectible`, or `void`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum PauseCollectionBehavior {
    KeepAsDraft,
    MarkUncollectible,
    Void,
}

impl PauseCollectionBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::KeepAsDraft => "keep_as_draft",
            Self::MarkUncollectible => "mark_uncollectible",
            Self::Void => "void",
        }
    }
}

impl AsRef<str> for PauseCollectionBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PauseCollectionBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// The Pause Collection settings determine how we will pause collection for this subscription and for how long the subscription
/// should be paused.
#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionsResourcePauseCollection {
    /// The payment collection behavior for this subscription while paused.
    ///
    /// One of `keep_as_draft`, `mark_uncollectible`, or `void`.
    pub behavior: SubscriptionsResourcePauseCollectionBehavior,
    /// The time after which the subscription will resume collecting payments.
    pub resumes_at: Option<stripe_types::Timestamp>,
}
/// The payment collection behavior for this subscription while paused.
///
/// One of `keep_as_draft`, `mark_uncollectible`, or `void`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SubscriptionsResourcePauseCollectionBehavior {
    KeepAsDraft,
    MarkUncollectible,
    Void,
}

impl SubscriptionsResourcePauseCollectionBehavior {
    pub fn as_str(self) -> &'static str {
        use SubscriptionsResourcePauseCollectionBehavior::*;
        match self {
            KeepAsDraft => "keep_as_draft",
            MarkUncollectible => "mark_uncollectible",
            Void => "void",
        }
    }
}

impl std::str::FromStr for SubscriptionsResourcePauseCollectionBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionsResourcePauseCollectionBehavior::*;
        match s {
            "keep_as_draft" => Ok(KeepAsDraft),
            "mark_uncollectible" => Ok(MarkUncollectible),
            "void" => Ok(Void),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for SubscriptionsResourcePauseCollectionBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionsResourcePauseCollectionBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SubscriptionsResourcePauseCollectionBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SubscriptionsResourcePauseCollectionBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SubscriptionsResourcePauseCollectionBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for SubscriptionsResourcePauseCollectionBehavior",
            )
        })
    }
}

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
    pub resumes_at: Option<stripe_types::Timestamp>,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for PauseCollectionBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "keep_as_draft" => Ok(Self::KeepAsDraft),
            "mark_uncollectible" => Ok(Self::MarkUncollectible),
            "void" => Ok(Self::Void),

            _ => Err(()),
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
impl serde::Serialize for PauseCollectionBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PauseCollectionBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for PauseCollectionBehavior"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PauseCollectionBehavior {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PauseCollectionBehavior> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PauseCollectionBehavior::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

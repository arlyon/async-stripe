use crate::StripeError;

pub trait StripeDeserialize {
    fn deserialize(str: &str) -> Result<Self, StripeError>
    where
        Self: Sized;
}

#[cfg(feature = "min-ser")]
impl<T: miniserde::Deserialize> StripeDeserialize for T {
    fn deserialize(str: &str) -> Result<Self, StripeError> {
        miniserde::json::from_str(str)
            .map_err(|_| StripeError::JSONDeserialize("Could not deserialize".into()))
    }
}

#[cfg(not(feature = "min-ser"))]
impl<T: serde::de::DeserializeOwned> StripeDeserialize for T {
    fn deserialize(str: &str) -> Result<Self, StripeError> {
        let json_deserializer = &mut serde_json::Deserializer::from_str(str);
        serde_path_to_error::deserialize(json_deserializer).map_err(StripeError::from)
    }
}

#[cfg(feature = "min-ser")]
#[derive(miniserde::Deserialize)]
pub struct ObjectName {
    pub object: String,
}

#[cfg(feature = "min-ser")]
#[derive(miniserde::Deserialize)]
pub struct MaybeDeleted {
    pub deleted: Option<bool>,
}

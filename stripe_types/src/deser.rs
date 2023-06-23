pub trait StripeDeserialize {
    fn deserialize(str: &str) -> Result<Self, String>
    where
        Self: Sized;
}

#[cfg(feature = "min-ser")]
impl<T: miniserde::Deserialize> StripeDeserialize for T {
    fn deserialize(str: &str) -> Result<Self, String> {
        miniserde::json::from_str(str).map_err(|err| err.to_string())
    }
}

#[cfg(not(feature = "min-ser"))]
impl<T: serde::de::DeserializeOwned> StripeDeserialize for T {
    fn deserialize(str: &str) -> Result<Self, String> {
        let json_deserializer = &mut serde_json::Deserializer::from_str(str);
        serde_path_to_error::deserialize(json_deserializer).map_err(|err| err.to_string())
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

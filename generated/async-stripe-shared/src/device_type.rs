#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum DeviceType {
    BbposChipper2x,
    BbposWisepad3,
    BbposWiseposE,
    MobilePhoneReader,
    SimulatedStripeS700,
    SimulatedStripeS710,
    SimulatedVerifoneM425,
    SimulatedVerifoneP630,
    SimulatedVerifoneUx700,
    SimulatedVerifoneV660p,
    SimulatedWiseposE,
    StripeM2,
    StripeS700,
    StripeS710,
    VerifoneP400,
    VerifoneM425,
    VerifoneP630,
    VerifoneUx700,
    VerifoneV660p,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl DeviceType {
    pub fn as_str(&self) -> &str {
        use DeviceType::*;
        match self {
            BbposChipper2x => "bbpos_chipper2x",
            BbposWisepad3 => "bbpos_wisepad3",
            BbposWiseposE => "bbpos_wisepos_e",
            MobilePhoneReader => "mobile_phone_reader",
            SimulatedStripeS700 => "simulated_stripe_s700",
            SimulatedStripeS710 => "simulated_stripe_s710",
            SimulatedVerifoneM425 => "simulated_verifone_m425",
            SimulatedVerifoneP630 => "simulated_verifone_p630",
            SimulatedVerifoneUx700 => "simulated_verifone_ux700",
            SimulatedVerifoneV660p => "simulated_verifone_v660p",
            SimulatedWiseposE => "simulated_wisepos_e",
            StripeM2 => "stripe_m2",
            StripeS700 => "stripe_s700",
            StripeS710 => "stripe_s710",
            VerifoneP400 => "verifone_P400",
            VerifoneM425 => "verifone_m425",
            VerifoneP630 => "verifone_p630",
            VerifoneUx700 => "verifone_ux700",
            VerifoneV660p => "verifone_v660p",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for DeviceType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DeviceType::*;
        match s {
            "bbpos_chipper2x" => Ok(BbposChipper2x),
            "bbpos_wisepad3" => Ok(BbposWisepad3),
            "bbpos_wisepos_e" => Ok(BbposWiseposE),
            "mobile_phone_reader" => Ok(MobilePhoneReader),
            "simulated_stripe_s700" => Ok(SimulatedStripeS700),
            "simulated_stripe_s710" => Ok(SimulatedStripeS710),
            "simulated_verifone_m425" => Ok(SimulatedVerifoneM425),
            "simulated_verifone_p630" => Ok(SimulatedVerifoneP630),
            "simulated_verifone_ux700" => Ok(SimulatedVerifoneUx700),
            "simulated_verifone_v660p" => Ok(SimulatedVerifoneV660p),
            "simulated_wisepos_e" => Ok(SimulatedWiseposE),
            "stripe_m2" => Ok(StripeM2),
            "stripe_s700" => Ok(StripeS700),
            "stripe_s710" => Ok(StripeS710),
            "verifone_P400" => Ok(VerifoneP400),
            "verifone_m425" => Ok(VerifoneM425),
            "verifone_p630" => Ok(VerifoneP630),
            "verifone_ux700" => Ok(VerifoneUx700),
            "verifone_v660p" => Ok(VerifoneV660p),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "DeviceType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for DeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for DeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(DeviceType)).finish_non_exhaustive()
    }
}
impl serde::Serialize for DeviceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for DeviceType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<DeviceType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(DeviceType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(DeviceType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for DeviceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}

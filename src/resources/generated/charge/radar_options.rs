/// Options to configure Radar.
///
/// See [Radar Session](https://stripe.com/docs/radar/radar-session) for more information.
#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct RadarOptions {
    /// A [Radar Session](https://stripe.com/docs/radar/radar-session) is a snapshot of the browser metadata and device details that help Radar make more accurate predictions on your payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for RadarOptions {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

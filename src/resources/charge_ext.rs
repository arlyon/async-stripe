use crate::config::{Client, Response};
use crate::error::ErrorCode;
use crate::ids::ChargeId;
use crate::params::Object;
use crate::resources::{Charge, Rule};

impl Charge {
    /// Capture captures a previously created charge with capture set to false.
    ///
    /// For more details see [https://stripe.com/docs/api#charge_capture](https://stripe.com/docs/api#charge_capture).
    pub fn capture(
        client: &Client,
        charge_id: &ChargeId,
        params: CaptureParams<'_>,
    ) -> Response<Charge> {
        client.post_form(&format!("/charges/{}/capture", charge_id), params)
    }
}

impl Object for Rule {
    type Id = String;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        ""
    }
}

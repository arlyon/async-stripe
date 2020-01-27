use crate::config::{Client, Response};
use crate::ids::{CustomerId, PaymentMethodId};
use crate::resources::{PaymentMethod};
use serde_derive::{Deserialize, Serialize};

/// Attach a payment method to a customer
///
/// For more details see [https://stripe.com/docs/api/payment_methods/attach](https://stripe.com/docs/api/payment_methods/attach).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AttachPaymentMethod {
    pub customer: CustomerId,
}

impl PaymentMethod {
    /// Attach a payment method to a customer
    ///
    /// For more details see [https://stripe.com/docs/api/payment_methods/attach](https://stripe.com/docs/api/payment_methods/attach).
    pub fn attach(
        client: &Client,
        payment_method_id: &PaymentMethodId,
        attach: AttachPaymentMethod,
    ) -> Response<PaymentMethod> {
        client.post_form(&format!("/payment_methods/{}/attach", payment_method_id), attach)
    }
}

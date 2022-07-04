use serde::{Deserialize, Serialize};

use crate::client::{Client, Response};
use crate::ids::{CustomerId, PaymentMethodId};
use crate::resources::PaymentMethod;

/// The parameters for `PaymentMethod::attach`
///
/// For more details see <https://stripe.com/docs/api/payment_methods/attach>.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AttachPaymentMethod {
    pub customer: CustomerId,
}

impl PaymentMethod {
    /// Attach a payment method to a customer
    ///
    /// For more details see <https://stripe.com/docs/api/payment_methods/attach>.
    pub fn attach<'a>(
        client: &'a Client,
        payment_method_id: &'_ PaymentMethodId,
        params: AttachPaymentMethod,
    ) -> Response<'a, PaymentMethod> {
        client.post_form(&format!("/payment_methods/{}/attach", payment_method_id), params)
    }

    /// Detach a PaymentMethod from a Customer
    ///
    /// For more details see <https://stripe.com/docs/api/payment_methods/detach>.
    pub fn detach<'a>(
        client: &'a Client,
        payment_method_id: &'_ PaymentMethodId,
    ) -> Response<'a, PaymentMethod> {
        client.post(&format!("/payment_methods/{}/detach", payment_method_id))
    }
}

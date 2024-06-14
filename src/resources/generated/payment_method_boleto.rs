// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "payment_method_boleto".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodBoleto {

    /// Uniquely identifies the customer tax id (CNPJ or CPF).
    pub tax_id: String,
}

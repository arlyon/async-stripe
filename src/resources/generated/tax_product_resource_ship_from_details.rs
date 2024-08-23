// ======================================
// This file was automatically generated.
// ======================================

use crate::resources::{TaxProductResourcePostalAddress};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "TaxProductResourceShipFromDetails".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxProductResourceShipFromDetails {

    pub address: TaxProductResourcePostalAddress,
}

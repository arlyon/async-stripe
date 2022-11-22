// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "InvoiceSettingRenderingOptions".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InvoiceSettingRenderingOptions {
    /// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
    pub amount_tax_display: Option<String>,
}

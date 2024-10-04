// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{MarginId};
use crate::params::{Metadata, Object, Timestamp};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "Margin".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Margin {
    /// Unique identifier for the object.
    pub id: MarginId,

    /// Whether the margin can be applied to invoices, invoice items, or invoice line items.
    ///
    /// Defaults to `true`.
    pub active: bool,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<Metadata>,

    /// Name of the margin that's displayed on, for example, invoices.
    pub name: Option<String>,

    /// Percent that will be taken off the subtotal before tax (after all other discounts and promotions) of any invoice to which the margin is applied.
    pub percent_off: f64,

    /// Time at which the object was last updated.
    ///
    /// Measured in seconds since the Unix epoch.
    pub updated: Timestamp,
}

impl Object for Margin {
    type Id = MarginId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "margin"
    }
}

use tabled::settings::Style;
use tabled::{Table, Tabled};

use crate::components::Components;
use crate::stripe_object::StripeObject;
use crate::utils::write_to_file;

/// Write a table describing where all generated requests live
pub fn write_crate_table(components: &Components) -> anyhow::Result<()> {
    #[derive(Tabled)]
    struct CrateDisplay {
        #[tabled(rename = "Name")]
        name_cell: String,
        #[tabled(rename = "Crate")]
        krate: &'static str,
        #[tabled(rename = "Feature Gate")]
        feature_gate: String,
    }

    let mut comps = vec![];
    for obj in components.components.values() {
        if obj.requests.is_empty() {
            continue;
        }
        comps.push(CrateDisplay {
            krate: obj.krate_unwrapped().base().name(),
            feature_gate: obj.mod_path(),
            name_cell: name_cell(obj),
        })
    }
    // Cloning when sorting definitely inefficient, but not a hot path at all
    comps.sort_unstable_by_key(|c| (c.krate, c.name_cell.clone()));
    let mut table = Table::new(comps);
    table.with(Style::markdown());
    let display = table.to_string();
    write_to_file(display, "crate_info.md")?;
    Ok(())
}

fn name_cell(obj: &StripeObject) -> String {
    let ident = obj.ident();
    if let Some(doc_url) = &obj.stripe_doc_url {
        format!("[{ident}]({doc_url})")
    } else {
        ident.to_string()
    }
}

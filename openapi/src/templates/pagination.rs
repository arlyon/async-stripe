use std::fmt::Write;

use indoc::writedoc;

pub fn _write_impl_paginable(out: &mut String, impl_for: &str, obj_path: &str, id_is_option: bool) {
    let starting_after_eq =
        if id_is_option { "item.id().to_string()" } else { "Some(item.id().to_string())" };
    let _ = writedoc!(
        out,
        r#"
        impl crate::Paginable for {impl_for}<'_> {{
            type O = {obj_path};
            fn set_last(&mut self, item: &Self::O) {{
                use crate::pagination::Object;
                self.starting_after = {starting_after_eq};
            }}
        }}
        "#
    );
}

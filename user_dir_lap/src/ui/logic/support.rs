use leptos::prelude::*;
use leptos_struct_table::CellValue;

use crate::domain::model::Id;

impl CellValue for Id {
    type RenderOptions = ();

    fn render_value(self, _options: Self::RenderOptions) -> impl leptos::IntoView {
        view! { {self.0} }
    }
}

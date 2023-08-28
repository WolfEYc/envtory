use std::fmt::Display;

use maud::{Markup, html};

pub fn error_html(err: impl Display) -> Markup {
    html! {
        div class="error-message" { (err) }
    }
}
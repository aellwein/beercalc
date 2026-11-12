use dioxus::prelude::*;

pub const INPUT_CLASS: &str = "border-gray-300 p-1 border-solid border focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300";

pub fn number_field(
    label: String,
    value: f64,
    (min, max, step): (f64, f64, f64),
    unit: &str,
    on_change: impl Fn(f64) + 'static,
) -> Element {
    let unit = unit.to_string();
    rsx! {
        div { class: "flex flex-row gap-4 items-baseline flex-wrap",
            div { {label} }
            input {
                class: INPUT_CLASS,
                r#type: "number",
                min: "{min}",
                max: "{max}",
                step: "{step}",
                value: format!("{:.1}", value),
                onchange: move |evt| {
                    if let Ok(parsed) = evt.value().parse::<f64>() {
                        on_change(parsed);
                    }
                },
            }
            div { {unit} }
        }
    }
}

use crate::common::prelude::*;
use yew::prelude::*;
use yewdux::prelude::*;

const BASE_URL: &str = "https://github.com/aellwein/beercalc";
const LICENSE_SUFFIX: &str = "/blob/main/LICENSE";

#[function_component]
pub fn Footer() -> Html {
    let (state, _) = use_store::<CalcState>();
    let t = use_context::<Translator>().unwrap();
    let commit_hash_long: String =
        option_env!("VERGEN_GIT_SHA").map_or("unknown".into(), String::from);
    let commit_hash = commit_hash_long.chars().take(7).collect::<String>();

    html! {
        <footer class="my-6 text-center dark:text-gray-400">
            <a class="text-indigo-600 hover:underline" href={BASE_URL}>{format!(r#" {}"#, t.t("project on github", &state.language))}</a>{" | "}
            <a class="text-indigo-600 hover:underline" href={format!(r#"{}docs/beercalc/index.html"#, get_base_href())}>{"󰈙 Docs"}</a>{" | "}
            <span>{" "}
            <a class="text-indigo-600 hover:underline" href={format!(r#"{}/commit/{}"#, BASE_URL, commit_hash_long.as_str())}>{commit_hash.as_str()}</a></span>{" | "}
            {"󰿃 "}{t.t("licensed under", &state.language)}{" "}<a class="text-indigo-600 hover:underline" href={String::from(BASE_URL) + LICENSE_SUFFIX}>{t.t("mit license", &state.language)}</a>
            {format!(r#" | 🛠️ {}"#, t.t("made with love", &state.language))}
        </footer>
    }
}

use std::str::FromStr;

use chrono::{TimeZone, Utc};
use fraction::Fraction;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let start = Utc.ymd(2022, 05, 05);
    let end = Utc.ymd(2024, 02, 05);

    let now = Utc::today();
    let elapsed = (now - start).num_days();
    let all = (end - start).num_days();

    let decimal = elapsed as f32 / all as f32;
    let fraction = Fraction::from_str(&format!("{:.2}", decimal)).unwrap();
    let percent = format!("({:.0}%)", decimal * 100.0);

    let day_of = format!("Day {} of {}", elapsed, all);

    html! {
        <main class="grid items-center justify-items-center gap-5">
            <h2 class="font-semibold text-3xl">{ "Progress:" }</h2>
            <h1 class="font-semibold text-9xl">{ fraction }</h1>
            <div class="flex gap-5 items-center justify-center font-medium mt-3">
                <div>{ percent }</div>
                <div>{ day_of }</div>
            </div>
        </main>
    }
}

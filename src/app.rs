use chrono::{TimeZone, Utc};
use fraction::Fraction;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    // Actually, 2022-05-05, but this way total number of days is 640.
    let start = Utc.ymd(2022, 05, 06);
    let end = Utc.ymd(2024, 02, 05);

    let now = Utc::today();
    let elapsed = (now - start).num_days();
    let all = (end - start).num_days();

    let fraction = Fraction::new(elapsed as u64, all as u64);
    let percent = format!("({:.0}%)", (elapsed as f32 / all as f32) * 100.0);
    let day_of = format!("Day {} of {}", elapsed, all);

    html! {
        <main class="grid items-center justify-items-center md:gap-5 gap-3">
            <h2 class="font-semibold text-3xl">{ "Progress:" }</h2>
            <h1 class="font-semibold md:text-9xl text-8xl">{ fraction }</h1>
            <div class="flex gap-5 items-center justify-center font-medium md:mt-3 mt-2">
                <div>{ percent }</div>
                <div>{ day_of }</div>
            </div>
        </main>
    }
}

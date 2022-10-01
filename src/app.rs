use chrono::{Date, TimeZone, Utc};
use fraction::GenericFraction;
use yew::prelude::*;

fn get_month_diff(from: &Date<Utc>, to: &Date<Utc>) -> isize {
    let component =
        date_component::date_component::calculate(&from.and_hms(0, 0, 0), &to.and_hms(0, 0, 0));
    component.year * 12 + component.month + 1
}

#[function_component(App)]
pub fn app() -> Html {
    // Actually, 2022-05-05, but this way total number of days is even (640).
    let start = Utc.ymd(2022, 05, 06);
    let end = Utc.ymd(2024, 02, 05);
    let now = Utc::today();

    let elapsed = (now - start).num_days();
    let all = (end - start).num_days();

    let percent = format!("{:.1}%", (elapsed as f32 / all as f32) * 100.0);
    let fraction = GenericFraction::<i64>::new(elapsed, all);
    let day_of = format!("Day {elapsed} of {all}");
    let month_of = format!(
        "Month {} of {}",
        get_month_diff(&start, &now),
        get_month_diff(&start, &end)
    );

    html! {
        <main class="grid items-center justify-items-center md:gap-5 gap-3">
            <h2 class="font-semibold text-3xl">{ "Progress:" }</h2>
            <h1 class="font-semibold md:text-9xl text-8xl">{ percent }</h1>
            <div class="flex gap-5 items-center justify-center font-medium md:mt-3 mt-2">
                <div>{ fraction }</div>
                <div>{ day_of }</div>
                <div>{ month_of }</div>
            </div>
        </main>
    }
}

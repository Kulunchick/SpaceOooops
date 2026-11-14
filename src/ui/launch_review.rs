//! Launch Review screen: every violation and warning with its reason; Launch only when nothing blocks.
use leptos::prelude::*;
use spaceooops::sim::Finding;

use super::cost_table::CostTable;
use super::gauges::Gauges;
use super::state::{AppState, Screen};

#[component]
pub fn LaunchReviewScreen() -> impl IntoView {
    let state = expect_context::<AppState>();
    let review = state.review;
    let can_launch = move || review.with(|r| r.can_launch);
    view! {
        <section class="screen launch-review">
            <h2>"Launch Review"</h2>
            <Gauges />
            <div class="panel">
                <h3 class="finding-head violated">
                    "\u{2716} Hard constraints broken: " {move || review.with(|r| r.violations.len())}
                </h3>
                {move || findings(review.with(|r| r.violations.clone()), "finding violated")}
            </div>
            <div class="panel">
                <h3 class="finding-head thin">
                    "\u{26A0} Warnings: " {move || review.with(|r| r.warnings.len())}
                </h3>
                {move || findings(review.with(|r| r.warnings.clone()), "finding thin")}
            </div>
            <div class="panel">
                <h3>"Cost breakdown"</h3>
                <CostTable />
            </div>
            <div class="actions">
                <button class="btn" on:click=move |_| state.go(Screen::Design)>"Modify design"</button>
                <button
                    class="btn btn-primary"
                    disabled=move || !can_launch()
                    on:click=move |_| {
                        if can_launch() {
                            state.launch();
                        }
                    }
                >
                    "Launch"
                </button>
            </div>
        </section>
    }
}

fn findings(list: Vec<Finding>, class: &'static str) -> impl IntoView {
    if list.is_empty() {
        return view! { <p class="muted">"None."</p> }.into_any();
    }
    view! {
        <ul class="findings">
            {list
                .into_iter()
                .map(|f| view! {
                    <li class=class><span class="finding-role">{f.role.name()}</span> {f.reason}</li>
                })
                .collect_view()}
        </ul>
    }
    .into_any()
}

//! Launch Review screen: every violation and warning with its reason; Launch only when nothing blocks.
use leptos::prelude::*;
use spaceooops::sim::{Finding, Role};

use super::cost_table::CostTable;
use super::gauges::Gauges;
use super::go_no_go::GoNoGo;
use super::state::{AppState, Screen};

#[component]
pub fn LaunchReviewScreen() -> impl IntoView {
    let state = expect_context::<AppState>();
    let review = state.review;
    let can_launch = move || review.with(|r| r.can_launch) && state.all_roles_go();
    // Why Launch is disabled, next to the button: broken hard constraints first, then missing Go votes.
    let blocked_reason = move || {
        let broken = review.with(|r| r.violations.len());
        if broken > 0 {
            let s = if broken == 1 { "" } else { "s" };
            return format!("Launch is blocked: {broken} hard constraint{s} broken (listed above).");
        }
        if !state.all_roles_go() {
            let waiting: Vec<&str> = state.votes.with(|v| {
                Role::ALL.into_iter().zip(v.iter()).filter(|(_, vote)| **vote != Some(true)).map(|(r, _)| r.name()).collect()
            });
            return format!("Waiting for a Go vote from: {}.", waiting.join(", "));
        }
        String::new()
    };
    view! {
        <section class="screen launch-review" class:launching=move || state.launching.get()>
            <div class="launch-sequence" hidden=move || !state.launching.get()>
                <p id="launch-count" class="launch-count lp-display" role="status" aria-live="polite"></p>
                <button class="lp-btn launch-skip" on:click=move |_| state.finish_launch()>"Skip launch"</button>
            </div>
            <h2>"Launch Review"</h2>
            <Gauges />
            <div class="panel">
                {move || {
                    let n = review.with(|r| r.violations.len());
                    if n == 0 {
                        view! { <h3 class="finding-head ok">"\u{2714} Hard constraints broken: 0"</h3> }.into_any()
                    } else {
                        view! { <h3 class="finding-head violated">{format!("\u{2716} Hard constraints broken: {n}")}</h3> }.into_any()
                    }
                }}
                {move || findings(review.with(|r| r.violations.clone()), "finding violated")}
            </div>
            <div class="panel">
                <h3 class="finding-head thin">
                    "\u{26A0} Warnings: " {move || review.with(|r| r.warnings.len())}
                </h3>
                {move || findings(review.with(|r| r.warnings.clone()), "finding thin")}
            </div>
            <GoNoGo />
            <div class="panel">
                <h3>"Cost breakdown"</h3>
                <CostTable />
            </div>
            <div class="actions">
                <button class="btn" on:click=move |_| state.go(Screen::Design)>"Modify design"</button>
                <button
                    class="btn btn-primary"
                    disabled=move || !can_launch() || state.launching.get()
                    on:click=move |_| {
                        if can_launch() {
                            state.begin_launch();
                        }
                    }
                >
                    "Launch"
                </button>
                <p class="launch-blocked muted" role="status">{blocked_reason}</p>
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

//! Shared app state, provided once as context. The Design lives here, so leaving and returning to the
//! Design screen (e.g. "Modify design" after a Run) finds every choice intact.
use leptos::prelude::*;
use spaceooops::sim::{self, Catalog, Design, LaunchReview, Pick, ReliabilityTier, Role, RunResult};

use super::history_store::{self, HistoryRecord};
use super::seed::new_seed;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Screen {
    Title,
    Sources,
    ScenarioSelect,
    Brief,
    Design,
    LaunchReview,
    Run,
    Debrief,
    History,
}

#[derive(Clone, Copy)]
pub struct AppState {
    pub catalog: StoredValue<Catalog>,
    pub screen: RwSignal<Screen>,
    pub scenario_id: RwSignal<String>,
    pub design: RwSignal<Design>,
    pub tab: RwSignal<Role>,
    /// Launch Review of the current Design, recomputed on every change.
    pub review: Memo<LaunchReview>,
    /// The last Run's result (timeline + Debrief) and the seed it flew with.
    pub run_result: RwSignal<Option<RunResult>>,
    pub seed: RwSignal<u64>,
    /// Run playback position in days elapsed, fractional: 0.0 at launch, `timeline.len()` at the end.
    /// The single source of truth for everything that animates with the Run (dashboards, event log,
    /// orbit animation); read it, and only the playback driver in `run/playback.rs` writes it.
    pub playback: RwSignal<f64>,
    /// Past Runs, newest first; mirrored to local storage (ticket 09).
    pub history: RwSignal<Vec<HistoryRecord>>,
    /// A friendly message for the title screen, e.g. when a share link could not be opened (ticket 09).
    pub notice: RwSignal<Option<String>>,
}

impl AppState {
    pub fn new(catalog: Catalog) -> Self {
        let catalog = StoredValue::new(catalog);
        let scenario_id = RwSignal::new(String::new());
        let design = RwSignal::new(Design::default());
        let review = Memo::new(move |_| {
            let id = scenario_id.get();
            design.with(|d| catalog.with_value(|c| sim::launch_review(c, d, &id)))
        });
        AppState {
            catalog,
            screen: RwSignal::new(Screen::Title),
            scenario_id,
            design,
            tab: RwSignal::new(Role::Science),
            review,
            run_result: RwSignal::new(None),
            seed: RwSignal::new(0),
            playback: RwSignal::new(0.0),
            history: RwSignal::new(history_store::load()),
            notice: RwSignal::new(None),
        }
    }

    pub fn go(&self, screen: Screen) {
        self.screen.set(screen);
    }

    /// Flies the current Design with a fresh seed and starts the Run playback. Also "Fly again".
    pub fn launch(&self) {
        let seed = new_seed();
        let scenario_id = self.scenario_id.get_untracked();
        let result = self
            .design
            .with_untracked(|d| self.catalog.with_value(|c| sim::run(c, d, &scenario_id, seed)));
        let record = HistoryRecord::from_run(
            &self.design.get_untracked(),
            &scenario_id,
            seed,
            &result,
            leptos::web_sys::js_sys::Date::now(),
        );
        self.history.update(|h| {
            h.insert(0, record);
            h.truncate(history_store::MAX_RECORDS);
        });
        self.history.with_untracked(|h| history_store::save(h));
        self.seed.set(seed);
        self.run_result.set(Some(result));
        self.playback.set(0.0);
        self.go(Screen::Run);
    }

    /// Number of days in the last Run's timeline.
    pub fn run_days(&self) -> usize {
        self.run_result.with(|r| r.as_ref().map_or(0, |r| r.timeline.len()))
    }

    /// Index into the timeline of the day the playback is showing (`None` before any Run).
    pub fn playback_index(&self) -> Option<usize> {
        let days = self.run_days();
        if days == 0 {
            return None;
        }
        let shown_day = (self.playback.get().ceil() as usize).clamp(1, days);
        Some(shown_day - 1)
    }

    /// Reason why an option clashes with the current choices, if it does.
    pub fn incompatible_reason(&self, option_id: &str) -> Option<String> {
        self.review.with(|r| r.incompatible.iter().find(|i| i.option_id == option_id).map(|i| i.reason.clone()))
    }
}

/// The four Component slots that carry a Reliability Tier.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Slot {
    Solar,
    Battery,
    Attitude,
    Comms,
}

impl Slot {
    pub fn get(self, d: &Design) -> &Option<Pick> {
        match self {
            Slot::Solar => &d.solar,
            Slot::Battery => &d.battery,
            Slot::Attitude => &d.attitude,
            Slot::Comms => &d.comms,
        }
    }

    pub fn get_mut(self, d: &mut Design) -> &mut Option<Pick> {
        match self {
            Slot::Solar => &mut d.solar,
            Slot::Battery => &mut d.battery,
            Slot::Attitude => &mut d.attitude,
            Slot::Comms => &mut d.comms,
        }
    }

    /// Pick an option, keeping the tier already chosen for this slot.
    pub fn choose(self, d: &mut Design, id: &str) {
        let slot = self.get_mut(d);
        let tier = slot.as_ref().map_or(ReliabilityTier::Basic, |p| p.tier);
        *slot = Some(Pick::new(id, tier));
    }
}

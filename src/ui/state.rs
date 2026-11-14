//! Shared app state, provided once as context. The Design lives here, so leaving and returning to the
//! Design screen (e.g. "Modify design" after a Run) finds every choice intact.
use leptos::prelude::*;
use spaceooops::sim::{self, Catalog, Decision, Design, LaunchReview, Pick, ReliabilityTier, Role, RunResult};

use super::history_store::{self, HistoryRecord};
use super::seed::new_seed;
use super::team::{Team, Votes};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Screen {
    Title,
    Sources,
    TeamSetup,
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
    /// The player's answers to interactive Random Events in the current Run (ticket 10).
    pub decisions: RwSignal<Vec<Decision>>,
    /// Run playback position in days elapsed, fractional: 0.0 at launch, `timeline.len()` at the end.
    /// The single source of truth for everything that animates with the Run (dashboards, event log,
    /// orbit animation); read it, and only the playback driver in `run/playback.rs` writes it.
    pub playback: RwSignal<f64>,
    /// Past Runs, newest first; mirrored to local storage (ticket 09).
    pub history: RwSignal<Vec<HistoryRecord>>,
    /// A friendly message for the title screen, e.g. when a share link could not be opened (ticket 09).
    pub notice: RwSignal<Option<String>>,
    /// Solo or team play, player names and Role owners (ticket 06).
    pub team: RwSignal<Team>,
    /// Go/No-Go votes per Role at Launch Review; any Design change clears them.
    pub votes: RwSignal<Votes>,
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
        let votes = RwSignal::new(Votes::default());
        Effect::new(move |_| {
            design.track();
            votes.set(Votes::default());
        });
        AppState {
            team: RwSignal::new(Team::default()),
            votes,
            catalog,
            screen: RwSignal::new(Screen::Title),
            scenario_id,
            design,
            tab: RwSignal::new(Role::Science),
            review,
            run_result: RwSignal::new(None),
            seed: RwSignal::new(0),
            decisions: RwSignal::new(Vec::new()),
            playback: RwSignal::new(0.0),
            history: RwSignal::new(history_store::load()),
            notice: RwSignal::new(None),
        }
    }

    /// Launch may proceed as far as voting goes: solo play skips it, team play needs every Role on Go.
    pub fn all_roles_go(&self) -> bool {
        self.team.with(|t| t.solo) || self.votes.with(|v| v.iter().all(|x| *x == Some(true)))
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
        self.decisions.set(Vec::new());
        self.run_result.set(Some(result));
        self.playback.set(0.0);
        self.go(Screen::Run);
    }

    /// Answers a question of an interactive Random Event: re-flies the Run with the same seed and every answer
    /// so far, and updates this Run's history record in place (still one record per Run). The days already
    /// played back are identical, so the playback carries on from where it is.
    pub fn decide(&self, decision: Decision) {
        self.decisions.update(|d| d.push(decision));
        let seed = self.seed.get_untracked();
        let scenario_id = self.scenario_id.get_untracked();
        let design = self.design.get_untracked();
        let result = self.decisions.with_untracked(|decisions| {
            self.catalog.with_value(|c| sim::run_with_decisions(c, &design, &scenario_id, seed, decisions))
        });
        self.history.update(|h| {
            if let Some(newest) = h.first_mut().filter(|r| r.seed == seed && r.scenario_id == scenario_id) {
                *newest = HistoryRecord::from_run(&design, &scenario_id, seed, &result, newest.timestamp_ms);
            }
        });
        self.history.with_untracked(|h| history_store::save(h));
        self.run_result.set(Some(result));
    }

    /// Number of days in the last Run's timeline.
    pub fn run_days(&self) -> usize {
        self.run_result.with(|r| r.as_ref().map_or(0, |r| r.timeline.len()))
    }

    /// The day the playback is showing, 1-based: 0 at launch, then day k while the playback is within it.
    /// Dashboards, the day counter and the event log all read this, so they always agree.
    pub fn shown_day(&self) -> usize {
        let days = self.run_days();
        let playback = self.playback.get();
        if playback <= 0.0 { 0 } else { (playback.ceil() as usize).min(days) }
    }

    /// The playback has reached the end of the timeline.
    pub fn playback_finished(&self) -> bool {
        self.playback.get() >= self.run_days() as f64
    }

    /// Index into the timeline of the snapshot the dashboards show (day 1 at launch; `None` before any Run).
    pub fn playback_index(&self) -> Option<usize> {
        if self.run_days() == 0 {
            return None;
        }
        Some(self.shown_day().max(1) - 1)
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

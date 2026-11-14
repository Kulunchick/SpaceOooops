//! Shared app state, provided once as context. The Design lives here, so leaving and returning to the
//! Design screen (e.g. "Modify design" after a Run) finds every choice intact.
use leptos::prelude::*;
use spaceooops::sim::{self, Catalog, Design, LaunchReview, Pick, ReliabilityTier, Role};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Screen {
    Title,
    ScenarioSelect,
    Brief,
    Design,
    LaunchReview,
    /// Placeholder until the Run screen exists (ticket 03).
    Run,
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
        }
    }

    pub fn go(&self, screen: Screen) {
        self.screen.set(screen);
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

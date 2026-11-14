//! The spacecraft model: the parts the scene draws, derived from the Design and the Catalog.
//!
//! Every part's `id` is the catalog option id it comes from (the structure id for the bus, `computer-obc` for
//! the computer, an Instrument id for an Instrument), so a part is stable while its choice stays, and the same
//! id names it in Random Events (`RandomEvent::component`). The renderer diffs parts by id to play the snap
//! animation; swapping an option is a removal plus an addition.
use serde::Serialize;

use crate::sim::{AntennaKind, AttitudeHardware, Catalog, Design, ReliabilityTier, SolarLayout};

use super::SceneInputs;

/// What kind of hardware a part is; the renderer picks its geometry by kind (and `shape`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PartKind {
    /// The structure: a box of `size`. Without a structure it is an empty frame with finish `outline`.
    Bus,
    /// Solar cells on the bus walls.
    BodyPanels,
    /// Two hinged wings, one on each side (`position` is the +x hinge; the renderer mirrors it to -x).
    SolarWings,
    /// The comms antenna; `shape` says which (`whip`, `patch`, `dish`).
    Antenna,
    ReactionWheels,
    Magnetorquers,
    /// A small battery pack on the bus wall; `count` is its number of cells.
    BatteryMarker,
    /// A small computer board on the bus wall.
    ComputerMarker,
    /// An Instrument; `shape` says which shape.
    Instrument,
}

/// Surface finish of a part.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Finish {
    /// Plain anodised metal and grey cells.
    Basic,
    /// Gold foil and polished trim: a Flight-proven Component.
    FlightProven,
    /// An empty frame: no structure chosen yet.
    Outline,
}

impl From<ReliabilityTier> for Finish {
    fn from(tier: ReliabilityTier) -> Self {
        match tier {
            ReliabilityTier::Basic => Finish::Basic,
            ReliabilityTier::FlightProven => Finish::FlightProven,
        }
    }
}

/// One visible part of the spacecraft. Lengths are in metres; the bus is centred on the origin with its long
/// axis along +y (up). A part is drawn in its own frame, where it stands on the bus face at its `position` and
/// points along local +y; the renderer turns local +y onto `facing`. The bus and body panels are the
/// exception: they are centred on the origin and face up.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Part {
    /// Stable id: the catalog option id the part comes from.
    pub id: String,
    pub kind: PartKind,
    /// The variant within the kind, for antennas and Instruments (kebab-case, from the catalog).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shape: Option<String>,
    /// Bounding box of the part (x, y, z) in its own frame, y being how far it reaches out of the face.
    pub size: [f64; 3],
    /// Where the part is mounted, relative to the bus centre.
    pub position: [f64; 3],
    /// Unit vector the part points along (its local +y): out of the bus face it is mounted on.
    pub facing: [f64; 3],
    /// How many repeated elements the part has (wing segments, wheels, rods, battery cells, whips); 1 otherwise.
    pub count: u32,
    pub finish: Finish,
}

/// The spacecraft as a list of parts; the first part is always the bus.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpacecraftModel {
    pub parts: Vec<Part>,
}

impl SpacecraftModel {
    /// The bus part.
    pub fn bus(&self) -> &Part {
        &self.parts[0]
    }
}

pub(super) fn for_inputs(inputs: &SceneInputs) -> Option<SpacecraftModel> {
    if !inputs.screen.shows_spacecraft() {
        return None;
    }
    Some(assemble(inputs.catalog?, inputs.design?))
}

const UP: [f64; 3] = [0.0, 1.0, 0.0];
const DOWN: [f64; 3] = [0.0, -1.0, 0.0];
const FRONT: [f64; 3] = [0.0, 0.0, 1.0];
const BACK: [f64; 3] = [0.0, 0.0, -1.0];
const RIGHT: [f64; 3] = [1.0, 0.0, 0.0];
/// One CubeSat unit: a 10 cm cube.
const U: f64 = 0.1;
/// The empty frame drawn before a structure is chosen: a 3U outline.
const NO_STRUCTURE_ID: &str = "no-structure";

/// Outer size of a bus of `size_u` CubeSat units: 1x1xN up to 3U, a 3U-long stack of 1U columns up to 12U
/// (6U = 2x1, 12U = 2x2), and a cube above that. Every dimension grows (or stays) with size.
pub fn bus_size(size_u: f64) -> [f64; 3] {
    let u = size_u.max(1.0);
    if u <= 3.0 {
        [U, u * U, U]
    } else if u <= 12.0 {
        let area = u / 3.0;
        let wide = area.sqrt().ceil();
        [wide * U, 3.0 * U, (area / wide).max(1.0) * U]
    } else {
        let side = u.cbrt().max(2.0) * U;
        [side.max(2.0 * U), side.max(3.0 * U), side.max(2.0 * U)]
    }
}

fn part(id: &str, kind: PartKind, size: [f64; 3], position: [f64; 3], facing: [f64; 3], finish: Finish) -> Part {
    Part { id: id.to_string(), kind, shape: None, size, position, facing, count: 1, finish }
}

/// Kebab-case name of a visual catalog enum, as the catalog data spells it.
fn kebab<T: Serialize>(value: &T) -> String {
    serde_json::to_value(value).ok().and_then(|v| v.as_str().map(str::to_string)).unwrap_or_default()
}

/// Builds the spacecraft model of a Design. Unknown ids (a stale Design) are skipped, never a panic.
pub fn assemble(c: &Catalog, d: &Design) -> SpacecraftModel {
    let structure = d.structure.as_deref().and_then(|id| c.structure(id));
    let (bus_id, bus, bus_finish) = match structure {
        Some(s) => (s.id.as_str(), bus_size(s.size_u), Finish::Basic),
        None => (NO_STRUCTURE_ID, bus_size(3.0), Finish::Outline),
    };
    let [bx, by, bz] = bus;
    // Small hardware grows a little on bigger buses, so it stays visible next to them.
    let scale = (bx.min(bz) / U).clamp(1.0, 3.0);
    let mut parts = vec![part(bus_id, PartKind::Bus, bus, [0.0; 3], UP, bus_finish)];

    if let Some(pick) = &d.solar {
        if let Some(o) = c.solar_panel(&pick.id) {
            parts.push(match o.layout {
                SolarLayout::Body => {
                    part(&o.id, PartKind::BodyPanels, [bx + 0.004, by * 0.96, bz + 0.004], [0.0; 3], UP, pick.tier.into())
                }
                SolarLayout::Wings => {
                    // Each wing folds against the side wall, so a segment is as wide as the wall is deep; more
                    // output per unit means more segments (2 W/U for cells on the walls alone).
                    let segments = (o.generation_w_per_u / 2.0).round().clamp(1.0, 4.0);
                    let mut wing = part(
                        &o.id,
                        PartKind::SolarWings,
                        [by * 0.95, segments * bz, 0.006],
                        [bx / 2.0, 0.0, 0.0],
                        RIGHT,
                        pick.tier.into(),
                    );
                    wing.count = segments as u32;
                    wing
                }
            });
        }
    }

    if let Some(pick) = &d.comms {
        if let Some(o) = c.comms_option(&pick.id) {
            let base = [0.0, -by / 2.0, 0.0];
            let (size, count) = match o.antenna {
                // Quarter-wave UHF tape whips are about 17 cm long.
                AntennaKind::Whip => ([0.004, 0.17, 0.004], 4),
                AntennaKind::Patch => ([0.07 * scale, 0.006, 0.07 * scale], 1),
                AntennaKind::Dish => {
                    let dia = (bx.min(bz) * 0.9).max(0.08);
                    ([dia, dia * 0.4, dia], 1)
                }
            };
            let mut antenna = part(&o.id, PartKind::Antenna, size, base, DOWN, pick.tier.into());
            antenna.shape = Some(kebab(&o.antenna));
            antenna.count = count;
            parts.push(antenna);
        }
    }

    // The back wall (-z) carries the attitude hardware (top), the battery (middle) and the computer (bottom).
    let back = |y: f64| [0.0, y, -bz / 2.0];
    if let Some(pick) = &d.attitude {
        if let Some(o) = c.attitude_control(&pick.id) {
            let mut hw = match o.hardware {
                AttitudeHardware::ReactionWheels => part(
                    &o.id,
                    PartKind::ReactionWheels,
                    [0.08 * scale, 0.03 * scale, 0.08 * scale],
                    back(by * 0.28),
                    BACK,
                    pick.tier.into(),
                ),
                AttitudeHardware::Magnetorquers => part(
                    &o.id,
                    PartKind::Magnetorquers,
                    [0.075 * scale, 0.012 * scale, 0.07 * scale],
                    back(by * 0.28),
                    BACK,
                    pick.tier.into(),
                ),
            };
            hw.count = 3;
            parts.push(hw);
        }
    }
    if let Some(pick) = &d.battery {
        if let Some(o) = c.battery(&pick.id) {
            let cells = (o.capacity_wh / 20.0).round().clamp(1.0, 6.0);
            let mut battery = part(
                &o.id,
                PartKind::BatteryMarker,
                [0.016 * cells * scale, 0.01 * scale, 0.05 * scale],
                back(0.0),
                BACK,
                pick.tier.into(),
            );
            battery.count = cells as u32;
            parts.push(battery);
        }
    }
    parts.push(part(
        &c.computer.id,
        PartKind::ComputerMarker,
        [0.05 * scale, 0.006 * scale, 0.04 * scale],
        back(-by * 0.28),
        BACK,
        d.computer_tier.into(),
    ));

    // Instruments: the first on the top face, the rest spread down the front face.
    let instruments: Vec<_> = d.instruments.iter().filter_map(|id| c.instrument(id)).collect();
    let on_front = instruments.len().saturating_sub(1).max(1) as f64;
    for (i, o) in instruments.iter().enumerate() {
        let side = o.volume_u.max(0.1).cbrt() * U * 0.8;
        let (position, facing, size) = if i == 0 {
            ([0.0, by / 2.0, 0.0], UP, [side, side * 1.2, side])
        } else {
            let y = -by / 2.0 + by * ((i - 1) as f64 + 0.5) / on_front;
            ([0.0, y, bz / 2.0], FRONT, [side, side * 0.8, side.min(by / on_front * 0.9)])
        };
        let mut instrument = part(&o.id, PartKind::Instrument, size, position, facing, Finish::Basic);
        instrument.shape = Some(kebab(&o.shape));
        parts.push(instrument);
    }
    SpacecraftModel { parts }
}

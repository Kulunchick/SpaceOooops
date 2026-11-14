use leptos::prelude::*;

/// One external link. All links open in a new tab.
#[component]
fn Ext(href: &'static str, label: &'static str) -> impl IntoView {
    view! { <a href=href target="_blank" rel="noopener noreferrer">{label}</a> }
}

#[component]
pub fn SourcesScreen(on_back: impl Fn() + 'static) -> impl IntoView {
    view! {
        <section class="screen sources-screen">
            <h2>"Sources and credits"</h2>
            <p class="muted">
                "Where the numbers and ideas in SpaceOooops come from. Game values are simplified and "
                "only approximately realistic."
            </p>

            <h3>"NASA resources"</h3>
            <ul class="sources-list">
                <li>
                    <Ext
                        href="https://www.nasa.gov/wp-content/uploads/2017/03/nasa_csli_cubesat_101_508.pdf"
                        label="CubeSat 101: Basic Concepts and Processes for First-Time CubeSat Developers"
                    />
                    " (NASA CubeSat Launch Initiative, revision dated October 2017). "
                    "Used for CubeSat structure sizes (3U, 6U, 12U) and for how a small-satellite mission is put together, "
                    "which shapes the Design options and their approximate values."
                </li>
                <li>
                    <Ext
                        href="https://www.nasa.gov/wp-content/uploads/2025/04/250323-design-and-develop-science-mission.pdf"
                        label="Design and Develop Science Missions"
                    />
                    " (NASA webinar slides, 26 March 2025, \"Access to Space for All\" systems engineering series). "
                    "Used for the framing of a mission as launch, space, ground and user segments and for the "
                    "idea of working from a science need to a mission design."
                </li>
                <li>
                    <Ext
                        href="https://science.nasa.gov/mission/webb/game-build-it-yourself/"
                        label="Build It Yourself (NASA Webb learning game)"
                    />
                    ". Inspiration for the Real-Mission Match in the Debrief, which relates your Design to a real mission."
                </li>
                <li>
                    <Ext href="https://software.nasa.gov/software/GSC-19468-1" label="General Mission Analysis Tool (GMAT)" />
                    ", "
                    <Ext
                        href="https://dataverse.jpl.nasa.gov/dataset.xhtml?persistentId=hdl:2014/44299"
                        label="Concept Maturity Levels"
                    />
                    ", "
                    <Ext
                        href="https://etd.gsfc.nasa.gov/capabilities/integrated-design-center/mission-design-lab/"
                        label="Mission Design Lab"
                    />
                    " and "
                    <Ext href="https://nasa.github.io/mission-viz/" label="Mission Viz" />
                    ". Background reading from the challenge resources on how real teams study and visualise mission concepts."
                </li>
            </ul>

            <h3>"Launch Vehicles"</h3>
            <p>
                "Launch Vehicles in the game are generic classes (small launcher, rideshare, medium vehicle). Each tooltip "
                "names a real example, for instance Rocket Lab Electron or SpaceX Falcon 9. The operators' own pages and "
                "payload user's guides are the primary sources, but they could not be opened reliably when this page was "
                "written, so the secondary sources below were used for the figures."
            </p>
            <ul class="sources-list">
                <li>
                    <Ext href="https://en.wikipedia.org/wiki/Electron_(rocket)" label="Electron (rocket), Wikipedia" />
                    " (secondary source)."
                </li>
                <li>
                    <Ext href="https://en.wikipedia.org/wiki/Falcon_9" label="Falcon 9, Wikipedia" />
                    " (secondary source)."
                </li>
                <li>
                    "The rideshare class is modelled on SpaceX's rideshare programme. No capacity figure for it was "
                    "confirmed from an official source, so its values are approximate game values."
                </li>
            </ul>

            <h3>"Space Agency Partner context"</h3>
            <p>
                "The challenge text and video name GGPEN, the Angolan National Space Program Management Office "
                "(Gabinete de Gest\u{e3}o do Programa Espacial Nacional), as the author of the Space Mission Design Game "
                "challenge. The game's three Scenarios are tied to Angola for that reason. Official site: "
                <Ext href="https://ggpen.gov.ao" label="ggpen.gov.ao" />
                "."
            </p>

            <h3>"Use of AI tools (SI)"</h3>
            <p>
                "The code of this game was written with Claude Code, an AI coding assistant by Anthropic, working from a "
                "specification prepared by the project team."
            </p>

            <h3>"Notices"</h3>
            <p>
                "SpaceOooops uses no NASA logos, flags or mission identifiers. It is a project made for the "
                "2026 NASA Space Apps Challenge and is not endorsed by NASA. NASA does not endorse any non-U.S. "
                "Government entity and is not responsible for information on non-U.S. Government websites."
            </p>

            <button class="btn" on:click=move |_| on_back()>"Back"</button>
        </section>
    }
}

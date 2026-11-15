use leptos::prelude::*;
use spaceooops::credits;

/// One external link. All links open in a new tab.
#[component]
fn Ext(href: &'static str, label: &'static str) -> impl IntoView {
    view! { <a href=href target="_blank" rel="noopener noreferrer">{label}</a> }
}

/// Splits a source string around its first `http(s)://` URL, so the URL can be a link and the rest stay text.
fn split_first_url(s: &str) -> (String, Option<String>, String) {
    let Some(start) = s.find("http") else { return (s.to_string(), None, String::new()) };
    let end = s[start..].find(|c: char| c.is_whitespace() || c == ')').map_or(s.len(), |i| start + i);
    (s[..start].to_string(), Some(s[start..end].to_string()), s[end..].to_string())
}

#[component]
pub fn SourcesScreen(on_back: impl Fn() + 'static) -> impl IntoView {
    let assets = credits::assets();
    view! {
        <section class="screen sources-screen lp-panel">
            <h2 class="lp-display">"Sources and credits"</h2>
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
                "Launch Vehicles in the game are generic classes, such as a rideshare slot or a small dedicated Launch Vehicle. Each tooltip "
                "names a real example of its class. The operators' own pages and "
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
                "challenge. Angola is therefore the game's home: its three first Scenarios are set there. Other countries are "
                "chosen from the partner agencies of Space Apps. Official site: "
                <Ext href="https://ggpen.gov.ao" label="ggpen.gov.ao" />
                "."
            </p>

            <h3>"World Scenarios: ground stations and real missions"</h3>
            <p>
                "Each Scenario outside Angola uses real ground stations of its country's space agency and compares "
                "your Design with one of that country's real missions. Station positions are rounded."
            </p>
            <ul class="sources-list">
                <li>
                    "Brazil (Amazon Watch), Brazilian Space Agency (AEB) and INPE. Cuiab\u{e1} station: "
                    <Ext href="https://landsat.usgs.gov/CUB" label="INPE Cuiab\u{e1} ground station, USGS Landsat station list" />
                    " (position, and Amazonia-1 among the satellites it tracks). Alc\u{e2}ntara station: "
                    <Ext
                        href="https://www.gov.br/pt-br/servicos/obter-servico-de-rastreio-e-controle-para-instituicoes-nacionais-ou-internacionais-inpe"
                        label="INPE tracking and control service, gov.br"
                    />
                    " (the Cuiab\u{e1} and Alc\u{e2}ntara stations), placed at the "
                    <Ext href="https://en.wikipedia.org/wiki/Alc%C3%A2ntara_Space_Center" label="Alc\u{e2}ntara Space Center, Wikipedia" />
                    " (secondary source). Amazonia-1: "
                    <Ext href="https://www.eoportal.org/satellite-missions/amazonia-1" label="Amazonia-1, eoPortal" />
                    " (secondary source)."
                </li>
            </ul>

            <h3>"Assets and licences"</h3>
            <p>
                "Every external asset shipped with the game, with its author, source and licence. "
                "This list is read from the game's credits data file."
            </p>
            <ul class="sources-list credits-list">
                {assets
                    .iter()
                    .map(|a| {
                        let (before, url, after) = split_first_url(&a.source);
                        view! {
                            <li class="credit">
                                <strong class="credit-name">{a.name.clone()}</strong>
                                " "
                                <span class="credit-kind">{a.kind.clone()}</span>
                                <br />
                                {format!("By {}. Licence: {}. Source: {}", a.author, a.licence, before)}
                                {url.map(|u| view! { <a href=u.clone() target="_blank" rel="noopener noreferrer">{u.clone()}</a> })}
                                {after}
                                {a.ai_generated.then(|| view! { " " <strong class="credit-ai">"Made with AI tools."</strong> })}
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>

            <h3>"Use of AI tools (SI)"</h3>
            <p>
                "The code of this game was written with Claude Code, an AI coding assistant by Anthropic, working from a "
                "specification prepared by the project team."
            </p>
            <p class="ai-assets">{credits::ai_statement(&assets)}</p>

            <h3>"Notices"</h3>
            <p>
                "SpaceOooops uses no NASA logos, insignia, mission patches or flags. Real missions, including some "
                "NASA missions, are named in plain text only, as factual references for comparison; no endorsement "
                "is implied. SpaceOooops is a project made for the "
                "2026 NASA Space Apps Challenge and is not endorsed by NASA. NASA does not endorse any non-U.S. "
                "Government entity and is not responsible for information on non-U.S. Government websites."
            </p>

            <button class="lp-btn" on:click=move |_| on_back()>"Back"</button>
        </section>
    }
}

use super::{ConnectionNode, FlowLink, Protocol};
use crate::{assets::images, ui::card::Card};
use dioxus::prelude::*;
#[component]
pub fn ConnectionFlow(
    source: String,
    target: String,
    source_protocol: Protocol,
    target_protocol: Protocol,
) -> Element {
    rsx! { Card { class:"p-6", div { class:"grid items-center gap-2 sm:grid-cols-[1fr_48px_72px_48px_1fr] sm:gap-0", "aria-label":"Connection summary",
    ConnectionNode { name:source,protocol:source_protocol }
    FlowLink {}
    img { src:images::HUB,alt:"Conctr",width:72,height:80,class:"mx-auto" }
    FlowLink {}
    ConnectionNode { name:target,protocol:target_protocol }
    } } }
}

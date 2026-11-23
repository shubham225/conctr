use super::Styles;
use conctr_ui_dioxus::assets::{icons::color, images};
use dioxus::prelude::*;

#[component]
pub(super) fn ProtocolHub() -> Element {
    rsx! {
        svg { class:Styles::art.inner, view_box:"0 0 350 176", role:"img", "aria-label":"REST, SOAP and OData routes converge at Conctr",
            path {d:"M116 28 H145 Q154 28 160 40 L177 76 Q183 88 194 88 H242 M116 88 H242 M116 148 H145 Q154 148 160 136 L177 100 Q183 88 194 88"}
            for (label,asset,y) in [("REST",color::REST,9),("SOAP",color::SOAP,69),("OData",color::ODATA,129)] {
                g {key:"{label}",transform:"translate(4 {y})",
                    rect {width:112,height:38,rx:8}
                    image {href:asset,x:10,y:7,width:24,height:24}
                    text {x:44,y:24,"{label}"}
                }
            }
            image {href:images::HUB,x:232,y:37,width:102,height:102}
        }
    }
}

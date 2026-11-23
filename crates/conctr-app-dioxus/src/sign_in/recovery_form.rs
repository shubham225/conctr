use conctr_api_dioxus::forgot_password;
use conctr_ui_dioxus::ui::{button::Button, input::Input};
use dioxus::prelude::*;

#[component]
pub(super) fn RecoveryForm(on_back: EventHandler<()>) -> Element {
    let mut username = use_signal(String::new);
    let mut busy = use_signal(|| false);
    let mut sent = use_signal(|| false);
    let mut failure = use_signal(|| "");
    rsx! {
        h1 { "Reset your password" }
        if sent(){
            p { role:"status","If this account exists, your workspace administrator can provide a temporary password. Sign in with it within 15 minutes to choose a new password." }
        }else{
            p { class:"text-sm text-muted-foreground","Enter your workspace username to request a temporary password." }
            form { class:"grid gap-4", method:"post", onsubmit:move |event|{
                event.prevent_default();
                if busy(){return;}
                busy.set(true);
                failure.set("");
                let name=username();
                spawn(async move{
                    match forgot_password(name).await{
                        Ok(())=>sent.set(true),
                        Err(ServerFnError::ServerError{code:429,..})=>failure.set("Too many requests. Try again in a minute."),
                        Err(_)=>failure.set("Could not request a temporary password. Try again."),
                    }
                    busy.set(false);
                });
            },
                div {class:"grid gap-1", label {r#for:"recovery-user","Username"}
                    Input { id:"recovery-user",name:"username",class:"h-12",required:true,value:username(),oninput:move |event:FormEvent|username.set(event.value()) }
                }
                if !failure().is_empty(){p {role:"alert","{failure}"}}
                Button {button_type:"submit",disabled:busy(),if busy(){"Requesting…"}else{"Request temporary password"}}
            }
        }
        button {r#type:"button",class:"text-sm font-medium text-primary hover:underline",disabled:busy(),onclick:move |_|on_back.call(()),"Back to sign in"}
    }
}

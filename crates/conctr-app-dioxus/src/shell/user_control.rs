use super::Styles;
use crate::{Route, sign_in};
use conctr_ui_dioxus::ui::{
    avatar::*,
    button::{Button, ButtonVariant},
    dropdown_menu::*,
};
use dioxus::prelude::*;

#[component]
pub(super) fn UserControl() -> Element {
    let mut session = use_context::<sign_in::SessionState>();
    let session_info = session.read();
    let username = session_info
        .as_ref()
        .map_or("", |info| info.username.as_str());
    let mut failure = use_signal(|| false);
    let mut signing_out = use_signal(|| false);
    rsx! {
        div { class: "@container relative",
            DropdownMenu {
                DropdownMenuTrigger { class: "h-11 w-full justify-center gap-2 border-0 bg-transparent px-0 @min-[9rem]:justify-start @min-[9rem]:px-2",
                    span { class: "sr-only", "User options for {username}" }
                    UserAvatar {}
                    span { class: "hidden min-w-0 flex-1 text-left leading-tight @min-[9rem]:grid", aria_hidden: "true",
                        span { class: "truncate text-sm font-medium", "{username}" }
                        span { class: "truncate text-xs text-muted-foreground", "Workspace account" }
                    }
                    span { class: "hidden text-muted-foreground @min-[9rem]:block", aria_hidden: "true",
                        icons::ChevronsUpDown { class: "size-4" }
                    }
                }
                DropdownMenuContent { position: DropdownMenuPosition::Top,
                    class: format!("{} w-56 max-w-full rounded-lg shadow-lg md:absolute! md:top-auto! md:bottom-0 md:left-full! md:ml-4 md:max-w-none", Styles::account_menu.inner),
                    div { class: "flex items-center gap-2 px-2 py-2",
                        UserAvatar {}
                        div { class: "grid min-w-0 text-sm leading-tight",
                            strong { class: "wrap-break-word font-medium", "{username}" }
                            span { class: "text-xs text-muted-foreground", "Workspace account" }
                        }
                    }
                    DropdownMenuSeparator {}
                    DropdownMenuGroup {
                        for (route, icon) in [(Route::Profile {}, rsx! { icons::UserRound {} }), (Route::Notifications {}, rsx! { icons::Bell {} })] {
                            DropdownMenuItem { key: "{route}", class: "p-0",
                                Link { to: route.clone(), class: "flex w-full items-center gap-2 px-2 py-2", "data-dropdown-close": "",
                                    span { aria_hidden: "true", {icon} }
                                    "{route.label()}"
                                }
                            }
                        }
                    }
                    DropdownMenuSeparator {}
                    Button {
                        variant: ButtonVariant::Ghost,
                        class: "w-full justify-start px-2",
                        disabled: signing_out(),
                        onclick: move |_| async move {
                            if signing_out() { return; }
                            signing_out.set(true);
                            failure.set(false);
                            match conctr_api_dioxus::logout().await {
                                Ok(()) => session.set(None),
                                Err(_) => {
                                    signing_out.set(false);
                                    failure.set(true);
                                }
                            }
                        },
                        span { aria_hidden: "true", icons::LogOut { class: "size-4" } }
                        if signing_out() { "Logging out…" } else { "Log out" }
                    }
                    span { role: "status", class: "sr-only", if signing_out() { "Logging out…" } }
                    if failure() {
                        p { role: "alert", class: "px-2 py-1 text-sm text-destructive", "Could not log out. Try again." }
                    }
                }
            }
        }
    }
}

#[component]
fn UserAvatar() -> Element {
    rsx! {
        Avatar { class: "rounded-lg",
            AvatarFallback { class: "rounded-lg bg-secondary text-secondary-foreground",
                span { aria_hidden: "true", icons::UserRound {} }
            }
        }
    }
}

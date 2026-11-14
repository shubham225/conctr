use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::dialog::{Dialog, DialogDescription, DialogTitle};
use dioxus::html::{ScrollBehavior, ScrollLogicalPosition, ScrollToOptions};
use dioxus::prelude::*;
use dioxus_icons::lucide::{Search, X};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};
use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

#[css_module("/src/components/ui/command/style.css")]
struct Styles;
const STYLES: Asset = asset!(
    "/src/components/ui/command/style.css",
    manganis::AssetOptions::css_module()
);

#[derive(Clone)]
struct ItemRecord {
    value: String,
    keywords: Vec<String>,
    disabled: bool,
    on_select: Option<EventHandler<String>>,
}

fn matches_query(value: &str, keywords: &[String], query: &str) -> bool {
    let searchable = format!("{value} {}", keywords.join(" ")).to_lowercase();
    query
        .split_whitespace()
        .all(|part| searchable.contains(&part.to_lowercase()))
}

#[derive(Clone)]
struct CommandContext {
    id: String,
    query: Signal<String>,
    active: Signal<Option<usize>>,
    items: Rc<RefCell<BTreeMap<usize, ItemRecord>>>,
    on_select: Option<EventHandler<String>>,
}

impl CommandContext {
    fn available(&self) -> Vec<usize> {
        let query = self.query.peek();
        self.items
            .borrow()
            .iter()
            .filter(|(_, item)| {
                !item.disabled && matches_query(&item.value, &item.keywords, &query)
            })
            .map(|(index, _)| *index)
            .collect()
    }
    fn select(&self, index: usize) {
        let item = self.items.borrow().get(&index).cloned();
        if let Some(item) = item
            && !item.disabled
            && matches_query(&item.value, &item.keywords, &self.query.peek())
        {
            if let Some(callback) = item.on_select {
                callback.call(item.value.clone());
            }
            if let Some(callback) = self.on_select {
                callback.call(item.value);
            }
        }
    }
}

/// Root command menu. `id` must be stable across SSR/hydration and unique on the page.
/// Items use unique `index` values across all groups to define keyboard order.
#[component]
pub fn Command(
    id: String,
    #[props(default)] on_select: Option<EventHandler<String>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let query = use_signal(String::new);
    let active = use_signal(|| None::<usize>);
    let items = use_hook(|| Rc::new(RefCell::new(BTreeMap::new())));
    let context = use_context_provider(|| CommandContext {
        id,
        query,
        active,
        items,
        on_select,
    });
    let mut keys = context.clone();
    let base = attributes!(div {
        class: "cn-command {Styles::command.inner} flex flex-col overflow-hidden",
        "data-filtering": !query().trim().is_empty(),
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        document::Stylesheet { href: STYLES }
        div {
            onkeydown: move |event: KeyboardEvent| {
                let available = keys.available();
                let active = *keys.active.peek();
                let position = active
                    .and_then(|index| available.iter().position(|item| *item == index));
                let next = match event.key() {
                    Key::ArrowDown => {
                        available
                            .get(position.map_or(0, |n| (n + 1) % available.len().max(1)))
                            .copied()
                    }
                    Key::ArrowUp => {
                        available
                            .get(
                                position
                                    .map_or(
                                        available.len().saturating_sub(1),
                                        |n| {
                                            if n == 0 {
                                                available.len().saturating_sub(1)
                                            } else {
                                                n - 1
                                            }
                                        },
                                    ),
                            )
                            .copied()
                    }
                    Key::Home if !event.modifiers().ctrl() => available.first().copied(),
                    Key::End if !event.modifiers().ctrl() => available.last().copied(),
                    Key::Enter => {
                        event.prevent_default();
                        if let Some(index) = active.or_else(|| available.first().copied()) {
                            keys.select(index);
                        }
                        return;
                    }
                    Key::Escape => {
                        keys.active.set(None);
                        return;
                    }
                    _ => return,
                };
                event.prevent_default();
                keys.active.set(next);
            },
            ..merged,
            {children}
        }
    }
}

/// Search field. Focus stays here while arrows select list options.
#[component]
pub fn CommandInput(
    #[props(default = "Search commands".to_string())] label: String,
    #[props(default = "Type a command or search…".to_string())] placeholder: String,
    #[props(default)] disabled: bool,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let mut context = use_context::<CommandContext>();
    let mut focused = context.clone();
    let list_id = format!("{}-list", context.id);
    let active_id = (context.active)().map(|index| format!("{}-item-{index}", context.id));
    let base = attributes!(input {
        class: "cn-command-input {Styles::input.inner} w-full min-w-0 bg-transparent",
        autocomplete: "off",
        spellcheck: "false",
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        document::Stylesheet { href: STYLES }
        div { class: "cn-command-search {Styles::search.inner} flex items-center gap-3",
            span { aria_hidden: "true",
                Search { size: "20" }
            }
            input {
                role: "combobox",
                aria_label: label,
                placeholder,
                disabled,
                aria_controls: list_id,
                aria_autocomplete: "list",
                aria_expanded: "true",
                aria_activedescendant: active_id,
                value: (context.query)(),
                oninput: move |event| {
                    context.query.set(event.value());
                    let next = context.available().first().copied();
                    context.active.set(next);
                },
                onfocus: move |_| {
                    if focused.active.peek().is_none() {
                        let next = focused.available().first().copied();
                        focused.active.set(next);
                    }
                },
                ..merged,
            }
        }
    }
}

/// Scrollable list of groups and options.
#[component]
pub fn CommandList(
    #[props(default = "Commands".to_string())] label: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let context = use_context::<CommandContext>();
    let base = attributes!(div { class: "cn-command-list {Styles::list.inner} overflow-y-auto", "data-command-list": true });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        document::Stylesheet { href: STYLES }
        div {
            id: "{context.id}-list",
            role: "listbox",
            aria_label: label,
            ..merged,
            {children}
        }
    }
}

/// A group hides automatically when all its items are filtered out.
#[component]
pub fn CommandGroup(heading: String, children: Element) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }
        div {
            class: "cn-command-group {Styles::group.inner}",
            role: "group",
            aria_label: heading.clone(),
            div {
                class: "cn-command-heading {Styles::heading.inner}",
                aria_hidden: "true",
                "{heading}"
            }
            {children}
        }
    }
}

/// A selectable option. `value` and `keywords` provide searchable text.
#[component]
pub fn CommandItem(
    index: usize,
    value: String,
    #[props(default)] keywords: Vec<String>,
    #[props(default)] disabled: bool,
    #[props(default)] on_select: Option<EventHandler<String>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut context = use_context::<CommandContext>();
    context.items.borrow_mut().insert(
        index,
        ItemRecord {
            value: value.clone(),
            keywords: keywords.clone(),
            disabled,
            on_select,
        },
    );
    let registry = context.items.clone();
    let mut active_on_drop = context.active;
    use_drop(move || {
        registry.borrow_mut().remove(&index);
        if let Ok(mut active) = active_on_drop.try_write()
            && *active == Some(index)
        {
            *active = None;
        }
    });
    let hidden = !matches_query(&value, &keywords, &(context.query)());
    let selected = (context.active)() == Some(index) && !disabled && !hidden;
    let mut mounted = use_signal(|| None::<Rc<MountedData>>);
    let active = context.active;
    use_effect(move || {
        if active() == Some(index)
            && let Some(element) = mounted()
        {
            spawn(async move {
                let _ = element
                    .scroll_to_with_options(ScrollToOptions {
                        behavior: ScrollBehavior::Instant,
                        vertical: ScrollLogicalPosition::Nearest,
                        horizontal: ScrollLogicalPosition::Nearest,
                    })
                    .await;
            });
        }
    });
    let choose = context.clone();
    let base = attributes!(div {
        class: "cn-command-item {Styles::item.inner} flex items-center gap-3",
        "data-command-item": true,
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        document::Stylesheet { href: STYLES }
        div {
            id: "{context.id}-item-{index}",
            role: "option",
            tabindex: -1,
            hidden,
            aria_disabled: disabled,
            aria_selected: selected,
            "data-disabled": disabled,
            "data-selected": selected,
            onpointermove: move |_| {
                if !disabled {
                    context.active.set(Some(index));
                }
            },
            onmounted: move |event| mounted.set(Some(event.data())),
            onmousedown: move |event| event.prevent_default(),
            onclick: move |_| choose.select(index),
            ..merged,
            {children}
        }
    }
}

/// Empty result message; shown only when no matching options remain.
#[component]
pub fn CommandEmpty(children: Element) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }
        div { class: "cn-command-empty {Styles::empty.inner}", role: "status", {children} }
    }
}

/// Visual separation between groups; suppressed while filtering.
#[component]
pub fn CommandSeparator() -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }
        div {
            class: "cn-command-separator {Styles::separator.inner}",
            role: "separator",
        }
    }
}

/// Right-aligned shortcut hint; compose with Kbd/KbdGroup for individual keys.
#[component]
pub fn CommandShortcut(children: Element) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }
        span { class: "cn-command-shortcut {Styles::shortcut.inner} ml-auto shrink-0",
            {children}
        }
    }
}

/// Controlled command palette using the shared focus-trapped Dialog.
/// Bind an application shortcut or button to `open`; no global listener is installed.
#[component]
pub fn CommandDialog(
    id: String,
    open: ReadSignal<bool>,
    on_open_change: EventHandler<bool>,
    #[props(default = "Command palette".to_string())] title: String,
    #[props(default = "Search for an action, then press Enter to run it.".to_string())] description: String,
    #[props(default)] on_select: Option<EventHandler<String>>,
    children: Element,
) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }
        Dialog {
            open: Some(open()),
            on_open_change,
            class: "cn-command-dialog {Styles::dialog.inner}",
            div { class: "sr-only",
                DialogTitle { "{title}" }
                DialogDescription { "{description}" }
            }
            Command { id, on_select, {children} }
            Button {
                class: "cn-command-close {Styles::close.inner}",
                variant: ButtonVariant::Ghost,
                size: ButtonSize::IconSm,
                aria_label: "Close command palette",
                onclick: move |_| on_open_change.call(false),
                X { size: "16" }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn search_matches_all_words_and_aliases_without_case_sensitivity() {
        let aliases = vec!["workflow run".to_string()];
        assert!(matches_query(
            "Open connections",
            &aliases,
            "CONNECTIONS open"
        ));
        assert!(matches_query("Open connections", &aliases, "workflow"));
        assert!(!matches_query(
            "Open connections",
            &aliases,
            "delete workflow"
        ));
        assert!(matches_query("Open connections", &[], "  "));
    }
}

use super::Tags;
use dioxus::prelude::*;
/// Add tags with Enter or comma; remove them with their buttons or Backspace.
#[component]
pub fn TagInput(
    #[props(default)] initial: Vec<String>,
    #[props(default)] on_change: Option<EventHandler<Vec<String>>>,
    #[props(default)] disabled: bool,
) -> Element {
    let mut tags = use_signal(|| initial);
    let mut draft = use_signal(String::new);
    rsx! {div{class:"flex min-h-10 flex-wrap items-center gap-2 rounded-md border border-input bg-card p-2 focus-within:ring-2 focus-within:ring-ring",
    Tags{values:tags(),on_remove:move |index:usize|{if !disabled{tags.write().remove(index);if let Some(cb)=on_change{cb.call(tags());}}}},
    input{r#type:"text",disabled,"aria-label":"Add tag",placeholder:"Add tag",class:"min-w-20 flex-1 bg-transparent text-sm outline-none",value:draft(),oninput:move|e|draft.set(e.value()),onkeydown:move|e|{
    if e.key()==Key::Enter || e.key()==Key::Character(",".into()){e.prevent_default();let next=draft().trim().trim_end_matches(',').to_string();if !next.is_empty()&&!tags.read().contains(&next){tags.write().push(next);if let Some(cb)=on_change{cb.call(tags());}}draft.set(String::new());}
    else if e.key()==Key::Backspace && draft().is_empty(){tags.write().pop();if let Some(cb)=on_change{cb.call(tags());}}
    }}
    }}
}

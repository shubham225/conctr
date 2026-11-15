use crate::ui::button::{Button, ButtonVariant};
use dioxus::prelude::*;
use time::{Date, Month};
/// Single-date calendar with separate month/year selectors and kit day sizing.
#[component]
pub fn Calendar(
    #[props(default = 2026)] year: i32,
    #[props(default = 11)] month: u8,
    #[props(default)] selected: Option<Date>,
    #[props(default)] on_change: Option<EventHandler<Date>>,
) -> Element {
    let mut view = use_signal(|| {
        selected
            .map(|d| (d.year(), u8::from(d.month())))
            .unwrap_or((year, month))
    });
    let mut local = use_signal(|| selected);
    let (y, m) = view();
    let Ok(month) = Month::try_from(m) else {
        return rsx! {p{"Invalid month"}};
    };
    let Ok(first) = Date::from_calendar_date(y, month, 1) else {
        return rsx! {p{"Invalid year"}};
    };
    let offset = first.weekday().number_days_from_monday() as i64;
    let chosen = selected.or(local());
    rsx! {section{class:"cn-composed-calendar inline-grid w-fit gap-3 rounded-lg border border-border bg-popover p-4 text-popover-foreground shadow-sm","aria-label":"Calendar",
    div{class:"flex items-center justify-between gap-4",
    Button{variant:ButtonVariant::Ghost,class:"size-8 p-0",aria_label:"Previous month",onclick:move |_|{let next=if m==1{(y-1,12)}else{(y,m-1)};if (-9998..=9998).contains(&next.0){view.set(next);}},icons::ChevronLeft{class:"size-4"}}
    div{class:"flex min-w-0 items-center gap-1 font-heading text-sm",
     select{ "aria-label":"Select month", class:"min-w-0 rounded-sm border border-border bg-popover px-1 py-1 text-sm text-foreground", value:m.to_string(), onchange:move|event|{if let Some(next)=event.value().parse::<u8>().ok().filter(|next|(1..=12).contains(next)){view.set((y,next));}},
      for number in 1..=12_u8 { if let Ok(name)=Month::try_from(number){option{value:number.to_string(),"{name}"}} }
     }
     select{ "aria-label":"Select year", class:"min-w-0 rounded-sm border border-border bg-popover px-1 py-1 text-sm text-foreground", value:y.to_string(), onchange:move|event|{if let Some(next)=event.value().parse::<i32>().ok().filter(|next|(-9998..=9998).contains(next)){view.set((next,m));}},
      for number in y.min(1900)..=y.max(2100) {option{value:number.to_string(),"{number}"}}
     }
    }
    Button{variant:ButtonVariant::Ghost,class:"size-8 p-0",aria_label:"Next month",onclick:move |_|{let next=if m==12{(y+1,1)}else{(y,m+1)};if (-9998..=9998).contains(&next.0){view.set(next);}},icons::ChevronRight{class:"size-4"}}
    }
    div{class:"grid grid-cols-7 gap-[2px]",for day in ["Mon","Tue","Wed","Thu","Fri","Sat","Sun"]{span{class:"w-9 text-center text-xs leading-7 text-muted-foreground","{day}"}}
    for cell in 0..42_i64{if let Some(date)=first.checked_add(time::Duration::days(cell-offset)){
    Button{variant:ButtonVariant::Ghost,class:if chosen==Some(date){"size-9 rounded-sm bg-primary p-0 text-primary-foreground hover:bg-primary hover:text-primary-foreground"}else if date.month()!=month{"size-9 rounded-sm p-0 text-muted-foreground"}else{"size-9 rounded-sm p-0 text-foreground hover:bg-muted"},aria_label:date.to_string(),onclick:move |_|{local.set(Some(date));if let Some(cb)=on_change{cb.call(date);}},"{date.day()}"}
    }}
    }
    }}
}

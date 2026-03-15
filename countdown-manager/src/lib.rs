use gloo_net::http::Request;
use leptos::prelude::*;
use leptos_meta::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// ── Data model ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DateEntry {
    pub key: String,
    pub label: String,
}

// ── Entry point ─────────────────────────────────────────────────────────────

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    let _ = console_log::init_with_level(log::Level::Debug);
    leptos::mount::mount_to_body(App);
}

// ── Root component ───────────────────────────────────────────────────────────

#[component]
fn App() -> impl IntoView {
    provide_meta_context();

    let (refresh, set_refresh) = signal(0u32);

    let dates = LocalResource::new(move || {
        let _ = refresh.get();
        async move { fetch_dates().await }
    });

    let on_change = move || set_refresh.update(|n| *n += 1);
    let on_retry  = move || set_refresh.update(|n| *n += 1);

    view! {
        <Title text="Countdown Manager" />
        <div class="max-w-4xl mx-auto px-4 pb-12">
            <header class="flex items-center justify-between py-7 border-b border-border mb-7">
                <h1 class="text-2xl font-bold tracking-tight text-foreground">
                    "Countdown "
                    <span class="text-accent">"Manager"</span>
                </h1>
            </header>

            <AddDateForm on_change=on_change />
            <DateList dates=dates on_retry=on_retry />
        </div>
    }
}

// ── Add-date form ────────────────────────────────────────────────────────────

#[component]
fn AddDateForm(on_change: impl Fn() + 'static + Clone) -> impl IntoView {
    let (key, set_key) = signal(String::new());
    let (label, set_label) = signal(String::new());
    let (status, set_status) = signal(Option::<(bool, String)>::None);
    let (loading, set_loading) = signal(false);

    let input_cls = "flex-1 min-w-36 bg-canvas border border-border rounded-md \
                     text-foreground text-sm px-3 py-2 outline-none \
                     focus:border-accent placeholder:text-muted transition-colors";

    let submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let k = key.get_untracked();
        let l = label.get_untracked();

        if k.is_empty() || l.is_empty() {
            set_status.set(Some((false, "Both fields are required.".into())));
            return;
        }
        if k.len() != 8 || k.chars().any(|c| !c.is_ascii_digit()) {
            set_status.set(Some((false, "Date key must be 8 digits (YYYYMMDD).".into())));
            return;
        }

        set_loading.set(true);
        set_status.set(None);

        let on_change = on_change.clone();
        // Clone signal handles before the async move so `submit` stays Fn.
        let set_key = set_key.clone();
        let set_label = set_label.clone();
        let set_status = set_status.clone();
        let set_loading = set_loading.clone();
        leptos::task::spawn_local(async move {
            match create_date(&k, &l).await {
                Ok(_) => {
                    set_key.set(String::new());
                    set_label.set(String::new());
                    set_status.set(Some((true, format!("Added {k}."))));
                    on_change();
                }
                Err(e) => set_status.set(Some((false, e))),
            }
            set_loading.set(false);
        });
    };

    view! {
        <section class="bg-surface border border-border rounded-xl p-6 mb-7">
            <h2 class="text-xs font-semibold uppercase tracking-widest text-muted mb-4">
                "Add a Date"
            </h2>
            <form on:submit=submit>
                <div class="flex gap-2 flex-wrap">
                    <input
                        type="text"
                        placeholder="YYYYMMDD"
                        maxlength="8"
                        class=input_cls
                        prop:value=key
                        on:input=move |ev| set_key.set(event_target_value(&ev))
                    />
                    <input
                        type="text"
                        placeholder="Label (e.g. \"Birthday\")"
                        class=input_cls
                        prop:value=label
                        on:input=move |ev| set_label.set(event_target_value(&ev))
                    />
                    <button
                        type="submit"
                        class="bg-accent hover:bg-accent-light text-white font-semibold \
                               text-sm px-4 py-2 rounded-md transition-colors cursor-pointer \
                               disabled:opacity-50 disabled:cursor-not-allowed"
                        disabled=move || loading.get()
                    >
                        {move || if loading.get() { "Adding…" } else { "Add" }}
                    </button>
                </div>
            </form>
            {move || status.get().map(|(ok, msg)| {
                let cls = if ok {
                    "text-xs text-success mt-3"
                } else {
                    "text-xs text-danger mt-3"
                };
                view! { <p class=cls>{msg}</p> }
            })}
        </section>
    }
}

// ── Date list ────────────────────────────────────────────────────────────────

#[component]
fn DateList(
    dates: LocalResource<Result<Vec<DateEntry>, String>>,
    on_retry: impl Fn() + 'static + Clone + Send,
) -> impl IntoView {
    view! {
        <div>
            <Suspense fallback=move || view! {
                <div class="flex flex-col items-center gap-3 py-16 text-muted">
                    <span class="text-4xl">"⏳"</span>
                    <p class="text-sm">"Loading dates…"</p>
                </div>
            }>
                {move || dates.get().map(|result| match result {
                    Ok(entries) => {
                        let count = entries.len();
                        view! {
                            <div class="flex items-center justify-between mb-4">
                                <h2 class="font-semibold text-foreground">
                                    "Upcoming & Past Dates"
                                </h2>
                                <span class="bg-surface border border-border rounded-full \
                                             text-muted text-xs px-3 py-0.5">
                                    {count}
                                </span>
                            </div>
                            <div class="flex flex-col gap-3">
                                <For
                                    each=move || entries.clone()
                                    key=|e| e.key.clone()
                                    children=|entry| view! { <DateCard entry=entry /> }
                                />
                            </div>
                        }.into_any()
                    }
                    Err(_) => {
                        let retry = on_retry.clone();
                        view! {
                            <div class="flex flex-col items-center gap-4 py-16 text-center">
                                <span class="text-5xl">"⚠️"</span>
                                <div>
                                    <p class="font-semibold text-foreground mb-1">
                                        "Could not reach the dates service"
                                    </p>
                                    <p class="text-sm text-muted">
                                        "The API may be temporarily unavailable. \
                                         Your dates are safe — try again in a moment."
                                    </p>
                                </div>
                                <button
                                    class="bg-surface border border-border hover:bg-elevated \
                                           text-foreground text-sm font-semibold px-5 py-2 \
                                           rounded-md transition-colors cursor-pointer"
                                    on:click=move |_| retry()
                                >
                                    "Retry"
                                </button>
                            </div>
                        }.into_any()
                    }
                })}
            </Suspense>
        </div>
    }
}

// ── Date card ────────────────────────────────────────────────────────────────

#[component]
fn DateCard(entry: DateEntry) -> impl IntoView {
    let (deleting, set_deleting) = signal(false);
    let (deleted, set_deleted) = signal(false);

    let key = entry.key.clone();
    let label = entry.label.clone();

    let countdown_text = days_until_text(&entry.key);
    let chip_cls = chip_classes(&entry.key);
    let past = is_past_date(&entry.key);

    // Both class strings are complete literals so Tailwind's scanner includes them.
    let card_cls = if past {
        "bg-surface border border-border rounded-xl px-5 py-4 flex items-center \
         justify-between gap-3 hover:bg-elevated transition-colors opacity-50"
    } else {
        "bg-surface border border-border rounded-xl px-5 py-4 flex items-center \
         justify-between gap-3 hover:bg-elevated transition-colors"
    };

    let key_for_delete = key.clone();
    // StoredValue::new_local uses thread-local (non-Send) storage, which lets us
    // store the closure without requiring Send + Sync.  The StoredValue handle
    // itself is Copy, so it can be used inside Fn closures in the view.
    let on_delete = StoredValue::new_local(move |_: web_sys::MouseEvent| {
        let k = key_for_delete.clone();
        set_deleting.set(true);
        let set_deleted = set_deleted.clone();
        let set_deleting = set_deleting.clone();
        leptos::task::spawn_local(async move {
            if delete_date(&k).await.is_ok() {
                set_deleted.set(true);
            }
            set_deleting.set(false);
        });
    });

    view! {
        <Show when=move || !deleted.get()>
            <div class=card_cls>
                <div class="flex items-center gap-4 min-w-0">
                    <span class="text-xs font-semibold tracking-wide text-muted \
                                 font-mono whitespace-nowrap shrink-0">
                        {key.clone()}
                    </span>
                    <span class="text-sm font-medium truncate text-foreground">
                        {label.clone()}
                    </span>
                </div>
                <div class="flex items-center gap-3 shrink-0">
                    <span class=chip_cls>{countdown_text.clone()}</span>
                    <button
                        class="border border-danger text-danger hover:bg-danger \
                               hover:text-white text-xs font-semibold px-3 py-1.5 \
                               rounded-md transition-colors disabled:opacity-50 cursor-pointer"
                        on:click=move |ev| on_delete.with_value(|f| f(ev))
                        disabled=move || deleting.get()
                    >
                        {move || if deleting.get() { "…" } else { "Delete" }}
                    </button>
                </div>
            </div>
        </Show>
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────────

fn days_until_text(key: &str) -> String {
    let (ky, km, kd) = match parse_ymd_parts(key) {
        Some(p) => p,
        None => return "—".into(),
    };
    use web_sys::js_sys;
    let now = js_sys::Date::new_0();
    let ty = now.get_full_year() as i64;
    let tm = now.get_month() as i64 + 1;
    let td = now.get_date() as i64;

    let diff = to_julian_day(ky, km, kd) - to_julian_day(ty, tm, td);
    match diff.cmp(&0) {
        std::cmp::Ordering::Greater => format!("{} days", diff),
        std::cmp::Ordering::Equal => "Today!".into(),
        std::cmp::Ordering::Less => format!("{} days ago", -diff),
    }
}

/// Returns the full Tailwind class string for the countdown chip.
/// Complete literals are required so Tailwind's scanner picks them all up.
fn chip_classes(key: &str) -> &'static str {
    const BASE: &str = "rounded-full text-xs font-bold px-3 py-1 whitespace-nowrap";
    // These strings must appear as literals for Tailwind to include them:
    const FUTURE: &str = "bg-accent/15 text-accent-light rounded-full text-xs font-bold px-3 py-1 whitespace-nowrap";
    const TODAY:  &str = "bg-success/15 text-success rounded-full text-xs font-bold px-3 py-1 whitespace-nowrap";
    const PAST:   &str = "bg-muted/15 text-muted rounded-full text-xs font-bold px-3 py-1 whitespace-nowrap";
    let _ = BASE;

    if key.len() != 8 {
        return PAST;
    }
    match (js_today(), parse_ymd(key)) {
        (Some(t), Some(d)) => match d.cmp(&t) {
            std::cmp::Ordering::Greater => FUTURE,
            std::cmp::Ordering::Equal => TODAY,
            std::cmp::Ordering::Less => PAST,
        },
        _ => PAST,
    }
}

fn is_past_date(key: &str) -> bool {
    if key.len() != 8 {
        return false;
    }
    matches!((js_today(), parse_ymd(key)), (Some(t), Some(d)) if d < t)
}

/// Converts a Gregorian date to a Julian Day Number (absolute day count).
/// Subtracting two JDNs gives the exact number of calendar days between them.
fn to_julian_day(y: i64, m: i64, d: i64) -> i64 {
    let a = (14 - m) / 12;
    let yr = y + 4800 - a;
    let mo = m + 12 * a - 3;
    d + (153 * mo + 2) / 5 + 365 * yr + yr / 4 - yr / 100 + yr / 400 - 32045
}

/// Parses an 8-digit YYYYMMDD key into (year, month, day).
fn parse_ymd_parts(key: &str) -> Option<(i64, i64, i64)> {
    if key.len() != 8 {
        return None;
    }
    let y = key[0..4].parse::<i64>().ok()?;
    let m = key[4..6].parse::<i64>().ok()?;
    let d = key[6..8].parse::<i64>().ok()?;
    Some((y, m, d))
}

/// Returns today's date as a plain integer YYYYMMDD using the JS Date API.
fn js_today() -> Option<i64> {
    use web_sys::js_sys;
    let d = js_sys::Date::new_0();
    let y = d.get_full_year() as i64;
    let m = d.get_month() as i64 + 1;
    let day = d.get_date() as i64;
    Some(y * 10000 + m * 100 + day)
}

fn parse_ymd(s: &str) -> Option<i64> {
    s.parse::<i64>().ok()
}

// ── API calls ────────────────────────────────────────────────────────────────

async fn fetch_dates() -> Result<Vec<DateEntry>, String> {
    let resp = Request::get("/api/dates")
        .send()
        .await
        .map_err(|_| "Service unavailable — could not connect to the dates API.".to_string())?;

    if !resp.ok() {
        return Err(format!("Unexpected server response (HTTP {}).", resp.status()));
    }

    resp.json::<Vec<DateEntry>>()
        .await
        .map_err(|_| "Received an unexpected response format from the API.".to_string())
}

async fn create_date(key: &str, label: &str) -> Result<(), String> {
    let resp = Request::post(&format!("/api/dates/{key}"))
        .body(label.trim())
        .map_err(|_| "Failed to build request.".to_string())?
        .send()
        .await
        .map_err(|_| "Service unavailable — could not reach the dates API.".to_string())?;

    if resp.status() == 201 {
        Ok(())
    } else {
        Err(format!(
            "Could not add date (HTTP {}).",
            resp.status()
        ))
    }
}

async fn delete_date(key: &str) -> Result<(), String> {
    let resp = Request::delete(&format!("/api/dates/{key}"))
        .send()
        .await
        .map_err(|_| "Service unavailable — could not reach the dates API.".to_string())?;

    if resp.ok() {
        Ok(())
    } else {
        Err(format!("Could not delete date (HTTP {}).", resp.status()))
    }
}

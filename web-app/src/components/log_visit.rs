use leptos::prelude::*;
use uuid::Uuid;
use crate::server::LogVisit;

#[component]
pub fn LogVisitModal(pub_id: Uuid, on_close: Callback<()>) -> impl IntoView {
    let log_action = ServerAction::<LogVisit>::new();
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let (date, set_date) = signal(today);
    let (notes, set_notes) = signal(String::new());

    let on_submit = move |ev: leptos::web_sys::SubmitEvent| {
        ev.prevent_default();
        log_action.dispatch(LogVisit {
            pub_id,
            visit_date: date.get(),
            notes: Some(notes.get()).filter(|s| !s.is_empty()),
        });
    };

    Effect::new(move |_| {
        if let Some(Ok(())) = log_action.value().get() {
            on_close.run(());
        }
    });

    view! {
        <div class="edit-pub-modal">
            <div class="modal-content">
                <div class="modal-header">
                    <h3>"Log Visit"</h3>
                    <button class="btn btn-ghost close-btn" on:click=move |_| on_close.run(())>"×"</button>
                </div>
                <form on:submit=on_submit class="edit-form">
                    <div class="form-grid">
                        <div class="form-group">
                            <label>"Date"</label>
                            <input type="date" value=date on:input=move |ev| set_date.set(event_target_value(&ev)) required />
                        </div>
                        <div class="form-group full-width">
                            <label>"Notes (Optional)"</label>
                            <textarea on:input=move |ev| set_notes.set(event_target_value(&ev))></textarea>
                        </div>
                    </div>
                    <div class="form-actions">
                        <button type="submit" class="btn btn-primary" disabled=log_action.pending()>
                            {move || if log_action.pending().get() { "Saving..." } else { "Save Visit" }}
                        </button>
                        <button type="button" class="btn btn-ghost" on:click=move |_| on_close.run(())>"Cancel"</button>
                    </div>
                    {move || log_action.value().get().map(|v| {
                        if let Err(e) = v {
                            view! { <p class="error">{e.to_string()}</p> }.into_any()
                        } else {
                            ().into_any()
                        }
                    })}
                </form>
            </div>
        </div>
    }
}

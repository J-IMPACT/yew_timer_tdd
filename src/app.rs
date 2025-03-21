use yew::prelude::*;
use gloo_timers::callback::Interval;
use crate::timers::manager::{Phase, TimerManager};

#[function_component(App)]
pub fn app() -> Html {
    let manager = use_state(|| TimerManager::new(25 * 60, 5 * 60));
    let running = use_state(|| false);
    
    let manager_ref = use_mut_ref(|| (*manager).clone());
    let running_ref = use_mut_ref(|| *running);

    {
        let manager_setter = manager.clone();
        let manager_ref = manager_ref.clone();
        let running_ref = running_ref.clone();

        use_effect_with((), move |_| {
            let interval = Interval::new(1000, move || {
                if *running_ref.borrow() {
                    let mut m = manager_ref.borrow().clone();
                    m.tick();
                    *manager_ref.borrow_mut() = m.clone();
                    manager_setter.set(m);
                }
            });
            || drop(interval)
        });
    }

    let toggle = {
        let running = running.clone();
        let running_ref = running_ref.clone();
        Callback::from(move |_: MouseEvent| {
            let next = !*running;
            running.set(next);
            *running_ref.borrow_mut() = next;
        })
    };

    let reset = {
        let manager = manager.clone();
        let manager_ref = manager_ref.clone();
        Callback::from(move |_: MouseEvent| {
            let mut m = manager_ref.borrow().clone();
            m.reset();
            *manager_ref.borrow_mut() = m.clone();
            manager.set(m);
        })
    };

    let (phase_label, time_left) = {
        let m = &*manager;
        let label = match m.phase {
            Phase::Work => "Work",
            Phase::Break => "Break",
        };
        let time = format!("{:02}:{:02}", m.timer.remaining / 60, m.timer.remaining % 60);
        (label, time)
    };

    html! {
        <div style="text-align: center; font-family: sans-serif;">
            <h1>{ "Pomodoro Mini Timer" }</h1>
            <h2>{ format!("Phase: {}", phase_label) }</h2>
            <h3>{ time_left }</h3>
            <button onclick={toggle.clone()}>
                { if *running { "Pause" } else { "Start" } }
            </button>
            <button onclick={reset}>{ "Reset" }</button>
        </div>
    }
}
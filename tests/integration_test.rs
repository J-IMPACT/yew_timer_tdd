use yew_timer_tdd::timers::manager::{Phase, TimerManager};

#[test]
fn test_timer_manager_transitions_phase() {
    let mut manager = TimerManager::new(2, 3);

    assert_eq!(manager.phase, Phase::Work);
    assert_eq!(manager.timer.remaining, 2);

    manager.tick();
    manager.tick();

    assert_eq!(manager.phase, Phase::Break);
    assert_eq!(manager.timer.remaining, 3);
}

#[test]
fn test_timer_manager_transitions_back_to_work() {
    let mut manager = TimerManager::new(1, 1);

    manager.tick();
    assert_eq!(manager.phase, Phase::Break);

    manager.tick();
    assert_eq!(manager.phase, Phase::Work);
}

#[test]
fn test_timer_manager_reset_behavior() {
    let mut manager = TimerManager::new(10, 5);
    manager.tick();
    manager.tick();
    manager.reset();

    assert_eq!(manager.phase, Phase::Work);
    assert_eq!(manager.timer.remaining, 10);
}
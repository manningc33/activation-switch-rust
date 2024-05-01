mod activation_switch;
mod cli;
mod power_levels;
mod switch;
mod therapy_controller;
// mod therapy_controller;

use activation_switch::ActivationSwitchConfigs;
use console::Term;
use power_levels::PowerLevels;
use switch::SwitchStatesTracker;
use therapy_controller::TherapyController;

fn main() {
    //cli::update_console();
    let mut switch_state_tracker = SwitchStatesTracker::new(3);

    let mut therapy_controller = TherapyController::new(vec![
        ActivationSwitchConfigs::new(String::from("Footswitch 1"), PowerLevels::Low),
        ActivationSwitchConfigs::new(String::from("FootSwitch 2"), PowerLevels::High),
        ActivationSwitchConfigs::new(String::from("Handswitch 1"), PowerLevels::Medium),
    ]);

    loop {
        if let Ok(character) = Term::buffered_stdout().read_char() {
            cli::process_character(
                character,
                &mut switch_state_tracker,
                &mut therapy_controller,
            );
            therapy_controller.update(switch_state_tracker.get_last_event());
            cli::update_console(&switch_state_tracker, &therapy_controller);
        }
    }
}

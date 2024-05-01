use crate::power_levels::PowerLevels;
use crate::switch::SwitchStatesTracker;
use crate::therapy_controller::{self, TherapyController};

fn clear_console() {
    print!("{}[2J", 27 as char);
}

fn switch_pressed_icon(pressed: bool) -> char {
    if pressed {
        ''
    } else {
        ''
    }
}

fn switch_powerlevel_icon(power_level: &PowerLevels) -> char {
    match power_level {
        PowerLevels::Low => '󱊡',
        PowerLevels::Medium => '󱊢',
        PowerLevels::High => '󱊣',
    }
}

fn switch_enable_disable_icon(enabled: bool) -> char {
    if enabled {
        ''
    } else {
        ''
    }
}

fn therapy_controller_status_string(status: &therapy_controller::Status) -> String {
    match status {
        therapy_controller::Status::ChangingPowerLevelTherapyActive => {
            String::from("Cannot change powerlevel, therapy active.")
        }
        therapy_controller::Status::EnableDisableTherapyActive => {
            String::from("Cannot enable/disable switch, therapy active.")
        }
        _ => String::new(),
    }
}

fn print_switch_info(switch_states_tracker: &SwitchStatesTracker) {
    println!("Switch state:");

    println!("  Switches num:\t| 1 | 2 | 3 |");
    print!("  Pressed:\t|");
    for switch in switch_states_tracker.switches() {
        print!(" {0} |", switch_pressed_icon(switch.pressed()));
    }
    println!("\n");
}

fn print_therapy_indicator(therapy_controller: &TherapyController) {
    let mut therapy_indicator = '󰌶';
    let mut power_indicator = '󰂎';
    if let Some(switch_configs) = therapy_controller.get_active_switch_configs() {
        therapy_indicator = '󰌵';
        power_indicator = switch_powerlevel_icon(switch_configs.power_level());
    };
    println!("  Therapy: {therapy_indicator} {power_indicator}");
}

fn print_controller_info(therapy_controller: &TherapyController) {
    println!("therapy_controller:\n");
    print_therapy_indicator(therapy_controller);

    println!("  Switches num:\t| 1 | 2 | 3 |");
    print!("  Power Levels:\t|");
    for switch in therapy_controller.switches() {
        print!(" {0} |", switch_powerlevel_icon(switch.power_level()));
    }

    print!("\n  Enabled?:\t|");
    for switch in therapy_controller.switches() {
        print!(" {0} |", switch_enable_disable_icon(switch.enabled()));
    }

    println!(
        "\n  Status: {0}",
        therapy_controller_status_string(therapy_controller.status())
    );

    println!();
    for (switch_num, switch) in therapy_controller.switches().iter().enumerate() {
        println!("{0}: {1}", switch_num + 1, switch.name());
    }
    println!();
}

pub fn update_console(
    switch_states_tracker: &SwitchStatesTracker,
    therapy_controller: &TherapyController,
) {
    clear_console();
    print_controller_info(therapy_controller);
    print_switch_info(switch_states_tracker);
}

pub fn process_character(
    character: char,
    switches: &mut SwitchStatesTracker,
    therapy_controller: &mut TherapyController,
) {
    match character {
        // toggle switch
        '1' => switches.toggle_switch(0),
        '2' => switches.toggle_switch(1),
        '3' => switches.toggle_switch(2),

        // adjust power level
        '7' => therapy_controller.switch_adjust_power_level(0),
        '8' => therapy_controller.switch_adjust_power_level(1),
        '9' => therapy_controller.switch_adjust_power_level(2),

        // disable enable switch
        '4' => therapy_controller.switch_toggle_enabled(0),
        '5' => therapy_controller.switch_toggle_enabled(1),
        '6' => therapy_controller.switch_toggle_enabled(2),
        _ => (),
    }
}

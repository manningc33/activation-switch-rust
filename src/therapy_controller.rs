use crate::activation_switch::ActivationSwitchConfigs;
use crate::switch::SwitchEvent;

pub enum Status {
    AllGood,

    ChangingPowerLevelTherapyActive,
    EnableDisableTherapyActive,
}

pub struct TherapyController {
    switches: Vec<ActivationSwitchConfigs>,
    therapy_active: bool,
    active_switch: Option<usize>,
    status: Status,
}

impl TherapyController {
    pub fn new(_switches: Vec<ActivationSwitchConfigs>) -> Self {
        Self {
            switches: _switches,
            therapy_active: false,
            active_switch: None,
            status: Status::AllGood,
        }
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn switches(&self) -> &Vec<ActivationSwitchConfigs> {
        &self.switches
    }

    pub fn therapy_active(&self) -> bool {
        self.therapy_active
    }

    pub fn get_active_switch_configs(&self) -> Option<&ActivationSwitchConfigs> {
        match self.active_switch {
            Some(index) => Some(&self.switches[index]),
            None => None,
        }
    }

    pub fn update(&mut self, event: Option<SwitchEvent>) {
        match event {
            Some(event) => {
                if event.pressed {
                    if self.active_switch == None && self.switches[event.id].enabled() {
                        self.active_switch = Some(event.id);
                        self.therapy_active = true;
                    }
                } else {
                    if let Some(activation_switch_id) = self.active_switch {
                        if activation_switch_id == event.id {
                            self.active_switch = None;
                            self.therapy_active = false;
                        }
                    }
                }
            }
            None => (),
        }
    }

    pub fn switch_adjust_power_level(&mut self, switch_id: usize) {
        if !self.therapy_active {
            self.switches[switch_id].cycle_powerlevel();
            self.status = Status::AllGood;
        } else {
            self.status = Status::ChangingPowerLevelTherapyActive;
        }
    }
    pub fn switch_toggle_enabled(&mut self, switch_id: usize) {
        if !self.therapy_active {
            self.switches[switch_id].toggle_enabled();
            self.status = Status::AllGood;
        } else {
            self.status = Status::EnableDisableTherapyActive;
        }
    }
}

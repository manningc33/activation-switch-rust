use crate::power_levels::PowerLevels;

pub struct ActivationSwitchConfigs {
    name: String,
    power_level: PowerLevels,
    enabled: bool,
}

impl ActivationSwitchConfigs {
    pub fn new(_name: String, _power_level: PowerLevels) -> Self {
        Self {
            name: _name,
            power_level: _power_level,
            enabled: true,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn power_level(&self) -> &PowerLevels {
        &self.power_level
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn toggle_enabled(&mut self) {
        self.enabled ^= true;
    }

    pub fn cycle_powerlevel(&mut self) {
        match self.power_level {
            PowerLevels::Low => self.power_level = PowerLevels::Medium,
            PowerLevels::Medium => self.power_level = PowerLevels::High,
            PowerLevels::High => self.power_level = PowerLevels::Low,
        }
    }
}

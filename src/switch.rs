extern crate queues;

use queues::{IsQueue, Queue};

#[derive(Clone)]
pub struct Switch {
    id: usize,
    pressed: bool,
}

impl Switch {
    pub fn new(_id: usize) -> Self {
        Self {
            id: _id,
            pressed: false,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn pressed(&self) -> bool {
        self.pressed
    }

    pub fn toggle(&mut self) -> bool {
        self.pressed ^= true;
        self.pressed
    }
}

#[derive(Clone)]
pub struct SwitchEvent {
    pub id: usize,
    pub pressed: bool,
}

pub struct SwitchStatesTracker {
    switches: Vec<Switch>,
    event_fifo: Queue<SwitchEvent>,
}

impl SwitchStatesTracker {
    pub fn new(num_swtiches: usize) -> Self {
        let mut switches = Vec::new();
        for id in 0..num_swtiches {
            switches.push(Switch::new(id));
        }
        Self {
            switches,
            event_fifo: Queue::new(),
        }
    }

    pub fn switches(&self) -> &Vec<Switch> {
        &self.switches
    }

    pub fn toggle_switch(&mut self, switch_id: usize) {
        let _ = self.event_fifo.add(SwitchEvent {
            id: switch_id,
            pressed: self.switches[switch_id].toggle(),
        });
    }

    pub fn get_last_event(&mut self) -> Option<SwitchEvent> {
        self.event_fifo.remove().ok()
    }
}

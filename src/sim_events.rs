use std::ops::DerefMut;

use module::ModuleIndex;
use ship::ShipState;
use sim::SimEvent;

pub struct DamageEvent {
    module_index: ModuleIndex,
    damage: u8,
}

impl DamageEvent {
    pub fn new(module_index: ModuleIndex, damage: u8) -> DamageEvent {
        DamageEvent {
            module_index: module_index,
            damage: damage,
        }
    }
}

impl SimEvent for DamageEvent {
    fn apply(&mut self, ship_state: &mut ShipState) {
        ship_state.deal_damage(self.module_index, self.damage);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct RepairEvent {
    module_index: ModuleIndex,
    repair: u8,
}

impl RepairEvent {
    pub fn new(module_index: ModuleIndex, repair: u8) -> RepairEvent {
        RepairEvent {
            module_index: module_index,
            repair: repair,
        }
    }
}

impl SimEvent for RepairEvent {
    fn apply(&mut self, ship_state: &mut ShipState) {
        ship_state.repair_damage(self.module_index, self.repair);
    }
}
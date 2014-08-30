use std::io::IoResult;

use module::{Module, ModuleBase};
use net::{InPacket, OutPacket, Packable};
use sim_element::SimElement;

pub struct EngineModule {
    base: ModuleBase,
}

impl EngineModule {
    pub fn new() -> EngineModule {
        EngineModule{base: ModuleBase::new()}
    }
}

impl Module for EngineModule {
    fn create_sim_elements(&self) -> Vec<Box<SimElement>> {
        vec!()
    }
}

impl Packable for EngineModule {
    fn new_from_packet(packet: &mut InPacket) -> IoResult<EngineModule> {
        let base = try!(Packable::new_from_packet(packet));
    
        Ok(EngineModule {
            base: base,
        })
    }
    
    fn write_to_packet(&self, packet: &mut OutPacket) -> IoResult<()> {
        try!(self.base.write_to_packet(packet));
        Ok(())
    }
}
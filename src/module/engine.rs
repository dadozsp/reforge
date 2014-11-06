use assets::ENGINE_TEXTURE;
use module::{IModule, Module, ModuleBase, ModuleRef, ModuleType, ModuleTypeStore, Propulsion, Engine};
use net::{InPacket, OutPacket};
use ship::{ShipId, ShipState};
use sim::SimEventAdder;
use vec::{Vec2, Vec2f};

#[cfg(client)]
use sim::{SimVisuals, SimVisual};
#[cfg(client)]
use sfml_renderer::SfmlRenderer;
#[cfg(client)]
use sprite_sheet::{SpriteSheet, Stay};
#[cfg(client)]
use asset_store::AssetStore;

#[deriving(Encodable, Decodable)]
pub struct EngineModule {
    pub base: ModuleBase,
}

impl EngineModule {
    pub fn new(mod_store: &ModuleTypeStore, mod_type: ModuleType) -> Module {
        Engine(EngineModule {
            base: ModuleBase::new(mod_store, mod_type),
        })
    }
}

impl IModule for EngineModule {
    fn server_preprocess(&mut self, ship_state: &mut ShipState) {
    }
    
    fn before_simulation(&mut self, ship_state: &mut ShipState, events: &mut SimEventAdder) {
    }
    
    #[cfg(client)]
    fn add_plan_visuals(&self, asset_store: &AssetStore, visuals: &mut SimVisuals, ship_id: ShipId) {
        let mut engine_sprite = SpriteSheet::new(asset_store.get_sprite_info(ENGINE_TEXTURE));
        engine_sprite.add_animation(Stay(0.0, 5.0, 0));
    
        visuals.add(ship_id, 0, box SpriteVisual {
            position: self.base.get_render_position().clone(),
            sprite_sheet: engine_sprite,
        });
    }
    
    #[cfg(client)]
    fn add_simulation_visuals(&self, asset_store: &AssetStore, visuals: &mut SimVisuals, ship_id: ShipId) {
        self.add_plan_visuals(asset_store, visuals, ship_id);
    }
    
    fn after_simulation(&mut self, ship_state: &mut ShipState) {
    }
    
    fn write_plans(&self, packet: &mut OutPacket) {
    }
    
    fn read_plans(&mut self, packet: &mut InPacket) {
    }
    
    fn write_results(&self, packet: &mut OutPacket) {
    }
    
    fn read_results(&mut self, packet: &mut InPacket) {
    }
    
    fn on_icon_clicked(&mut self) -> bool {
        false
    }
    
    fn on_module_clicked(&mut self, ship_id: ShipId, module: &ModuleRef) -> bool {
        false
    }
}

// Sprite sheet sim visual
#[cfg(client)]
pub struct SpriteVisual {
    position: Vec2f,
    sprite_sheet: SpriteSheet,
}

#[cfg(client)]
impl SimVisual for SpriteVisual {
    fn draw(&mut self, renderer: &SfmlRenderer, time: f32) {
        self.sprite_sheet.draw(renderer, self.position.x, self.position.y, 0.0, time);
    }
}
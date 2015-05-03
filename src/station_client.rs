use std::cell::RefCell;
use std::cmp;
use std::io;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::thread;
use std::time::Duration;
use time;

use event::Events;
use opengl_graphics::Gl;
use opengl_graphics::glyph_cache::GlyphCache;
use sdl2_window::Sdl2Window;

use asset_store::AssetStore;
use battle_context::{BattleContext, ClientPacketId, ServerPacketId, TICKS_PER_SECOND};
use net::{Client, InPacket, OutPacket};
use sector_data::SectorData;
use ship::ShipStored;
use sim::{SimEvents, SimEffects};
use station_gui::StationGui;

pub struct StationClient<'a> {
    client: &'a mut Client,
    
    // The player's ship
    player_ship: Option<ShipStored>,
}

impl<'a> StationClient<'a> {
    pub fn new(client: &'a mut Client, player_ship: Option<ShipStored>) -> StationClient<'a> {
        StationClient {
            client: client,
            player_ship: player_ship,
        }
    }
    
    pub fn run(&mut self, window: &Rc<RefCell<Sdl2Window>>, gl: &mut Gl, glyph_cache: &mut GlyphCache, asset_store: &AssetStore, sectors: Vec<SectorData>) {
        use window::ShouldClose;
        use quack::Get;
    
        let ref mut gui = StationGui::new(sectors);
    
        loop {
            for e in Events::new(window.clone()) {
                use event;
                use input;
                use event::*;

                let e: event::Event<input::Input> = e;
            
                // Forward events to GUI
                gui.event(&e, &self.player_ship);
                
                // Render GUI
                e.render(|args: &RenderArgs| {
                    gl.draw([0, 0, args.width as i32, args.height as i32], |c, gl| {
                        gui.draw(
                            &c,
                            gl,
                            glyph_cache,
                            asset_store,
                            &self.player_ship,
                            (1.0/60.0) + args.ext_dt,
                        );
                    });
                });
            }
        }
    }
}
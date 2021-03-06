use std::rc::Rc;
use std::cell::RefCell;

use glutin_window::GlutinWindow;
use piston::event_loop::Events;
use graphics::Context;
use piston::input::*;
use opengl_graphics::{GlGraphics, Texture};
use opengl_graphics::glyph_cache::GlyphCache;

use gui::{TextBox, TextButton};
use login::LoginError;

#[derive(Clone)]
pub enum LoginGuiAction {
    Login(String, String, String),
    Back,
}

pub struct LoginScreen {
    action: Option<LoginGuiAction>,

    mouse_x: f64,
    mouse_y: f64,
    
    pub login_error: Option<LoginError>,
    
    // Text boxes
    username_box: TextBox,
    password_box: TextBox,
    ip_box: TextBox,
    
    // Buttons
    back_button: TextButton,
    login_button: TextButton,
}

impl LoginScreen {
    pub fn new() -> LoginScreen {
        let mut password_box = TextBox::new("".to_string(), 20, [600.0, 370.0], [300.0, 40.0]);
        password_box.hide_text = true;
    
        LoginScreen {
            action: None,
            
            mouse_x: 0.0,
            mouse_y: 0.0,
            
            login_error: None,
            
            username_box: TextBox::new("user".to_string(), 24, [600.0, 300.0], [300.0, 40.0]),
            password_box: password_box,
            ip_box: TextBox::new("localhost".to_string(), 24, [600.0, 440.0], [300.0, 40.0]),
            
            back_button: TextButton::new("Back".to_string(), 24, [450.0, 500.0], [150.0, 40.0]),
            login_button: TextButton::new("Login".to_string(), 24, [610.0, 500.0], [150.0, 40.0]),
        }
    }

    pub fn run(&mut self, window: &Rc<RefCell<GlutinWindow>>, gl: &mut GlGraphics, glyph_cache: &mut GlyphCache, bg_texture: &Texture) -> LoginGuiAction {
        // Main loop
        for e in Events::events(window.clone()) {
            use piston::event_loop as event;
            use piston::input;
            use piston::event_loop::*;

            let e: input::Event<input::Input> = e;

            self.event(&e);

            // Render GUI
            e.render(|args| {
                gl.draw(args.viewport(), |c, gl| {
                    self.draw(&c, gl, glyph_cache, bg_texture);
                });
            });

            if self.action.is_some() {
                break;
            }
        }
        
        self.action.take().unwrap()
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        use piston::event_loop::*;
        
        e.mouse_cursor(|x, y| {
            self.on_mouse_moved(x, y);
        });
        e.press(|button| {
            match button {
                Button::Mouse(button) => {
                    self.on_mouse_pressed(button);
                },
                _ => {},
            }
        });
        
        // Handle text boxes
        self.username_box.event(e, [self.mouse_x, self.mouse_y]);
        self.password_box.event(e, [self.mouse_x, self.mouse_y]);
        self.ip_box.event(e, [self.mouse_x, self.mouse_y]);
        
        // Handle buttons
        self.login_button.event(e, [self.mouse_x, self.mouse_y]);
        self.back_button.event(e, [self.mouse_x, self.mouse_y]);
        
        if self.back_button.get_clicked() {
            self.action = Some(LoginGuiAction::Back);
        }
        
        if self.login_button.get_clicked() {
            self.action = Some(LoginGuiAction::Login(self.username_box.text.clone(),
                                                     self.password_box.text.clone(),
                                                     self.ip_box.text.clone()));
        }
    }

    fn on_mouse_pressed(&mut self, button: mouse::MouseButton) {
        match button {
            mouse::MouseButton::Left => {},
            mouse::MouseButton::Right => {},
            _ => {},
        }
    }

    fn on_mouse_moved(&mut self, x: f64, y: f64) {
        self.mouse_x = x;
        self.mouse_y = y;
    }

    fn draw(&mut self, context: &Context, gl: &mut GlGraphics, glyph_cache: &mut GlyphCache, bg_texture: &Texture) {
        use graphics::*;
        
        // Clear the screen
        clear([0.0; 4], gl);

        image(bg_texture, context.transform, gl);
        
        // Draw the username and password labels
        {
            let context = context.trans(400.0, 330.0);
            Text::new_color([1.0; 4], 32).draw(
                "Username",
                glyph_cache,
                &context.draw_state, context.transform,
                gl,
            );
        }
        
        {
            let context = context.trans(400.0, 400.0);
            Text::new_color([1.0; 4], 32).draw(
                "Password",
                glyph_cache,
                &context.draw_state, context.transform,
                gl,
            );
        }
        
        {
            let context = context.trans(400.0, 470.0);
            Text::new_color([1.0; 4], 32).draw(
                "IP Address",
                glyph_cache,
                &context.draw_state, context.transform,
                gl,
            );
        }
        
        // Draw the text boxes
        self.username_box.draw(context, gl, glyph_cache);
        self.password_box.draw(context, gl, glyph_cache);
        self.ip_box.draw(context, gl, glyph_cache);
        
        // Draw the buttons
        self.back_button.draw(context, gl, glyph_cache);
        self.login_button.draw(context, gl, glyph_cache);
        
        // Draw error messages
        if let Some(login_error) = self.login_error {
            match login_error {
                LoginError::NoSuchAccount => {
                    let context = context.trans(910.0, 330.0);
                    Text::new_color([1.0, 0.0, 0.0, 1.0], 30).draw(
                        "User doesn't exist",
                        glyph_cache,
                        &context.draw_state, context.transform,
                        gl,
                    );
                },
                LoginError::AlreadyLoggedIn => {
                    let context = context.trans(910.0, 330.0);
                    Text::new_color([1.0, 0.0, 0.0, 1.0], 30).draw(
                        "User already logged in",
                        glyph_cache,
                        &context.draw_state, context.transform,
                        gl,
                    );
                },
                LoginError::WrongPassword => {
                    let context = context.trans(910.0, 400.0);
                    Text::new_color([1.0, 0.0, 0.0, 1.0], 30).draw(
                        "Incorrect password",
                        glyph_cache,
                        &context.draw_state, context.transform,
                        gl,
                    );
                },
            }
        }
    }
}

//! Game project.
use fyrox::{
    core::{
        algebra::{Vector2, Vector3},
        pool::Handle,
        reflect::prelude::*,
        uuid::{uuid, Uuid},
        visitor::prelude::*, TypeUuidProvider
    },
    animation::spritesheet::SpriteSheetAnimation,
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    impl_component_provider,
    scene::{
        dim2::{rectangle::Rectangle, rigidbody::RigidBody},
        node::{Node},
    },
    script::{ScriptContext, ScriptTrait},
};

#[derive(Visit, Reflect, Debug, Clone, Default)]
pub(crate) struct Player {
    sprite: Handle<Node>,
    move_left: bool,
    move_right: bool,
    move_up: bool,
    move_down: bool,
    animations: Vec<SpriteSheetAnimation>,
    current_animation: u32,
}

impl_component_provider!(Player,);

impl TypeUuidProvider for Player {
    // Returns unique script id for serialization needs.
    fn type_uuid() -> Uuid {
        uuid!("c5671d19-9f1a-4286-8486-add4ebaadaec")
    }
}

impl ScriptTrait for Player {
    // Called once at initialization.
    fn on_init(&mut self, context: &mut ScriptContext) {}
    
    // Put start logic - it is called when every other script is already initialized.
    fn on_start(&mut self, context: &mut ScriptContext) { }

    // Called whenever there is an event from OS (mouse click, keypress, etc.)
    fn on_os_event(&mut self, trigger_event: &Event<()>, context: &mut ScriptContext) {
        if let Event::WindowEvent { event, .. } = trigger_event {
            if let WindowEvent::KeyboardInput { input, .. } = event {
                if let Some(keycode) = input.virtual_keycode {
                    let is_pressed = input.state == ElementState::Pressed;
        
                    match keycode {
                        VirtualKeyCode::W => self.move_up = is_pressed,
                        VirtualKeyCode::S => self.move_down = is_pressed,
                        VirtualKeyCode::A => self.move_left = is_pressed,
                        VirtualKeyCode::D => self.move_right = is_pressed,
                        // VirtualKeyCode::Space => self.jump = is_pressed,
                        _ => (),
                    }
                }
            }
        }
    }

    // Called every frame at fixed rate of 60 FPS.
    fn on_update(&mut self, context: &mut ScriptContext) {
        //movement
        let speed = 2.0;
        if let Some(rigid_body) = context.scene.graph[context.handle].cast_mut::<RigidBody>() {
            let x_speed = match (self.move_left,self.move_right,self.move_up!=self.move_down) {
                (true, false, true) => speed/(2_f32).sqrt(),
                (false, true, true) => -speed/(2_f32).sqrt(),
                (true, false, false) => speed,
                (false, true, false) => -speed,
                _ => 0.0,
            };
            let y_speed = match (self.move_up,self.move_down,self.move_right!=self.move_left) {
                (true, false, true) => speed/(2_f32).sqrt(),
                (false, true, true) => -speed/(2_f32).sqrt(),
                (true, false, false) => speed,
                (false, true, false) => -speed,
                _ => 0.0,
            };
            rigid_body.set_lin_vel(Vector2::new(x_speed, y_speed));
    
            //sprite direction
            if let Some(sprite) = context.scene.graph.try_get_mut(self.sprite) {
                // We want to change player orientation only if he's moving.
                if x_speed != 0.0 {
                    let local_transform = sprite.local_transform_mut();
                    let current_scale = **local_transform.scale();

                    local_transform.set_scale(Vector3::new(
                        // Just change X scaling to mirror player's sprite.
                        current_scale.x.copysign(-x_speed),
                        current_scale.y,
                        current_scale.z,
                    ));
                }
            }

            //change animation loop
            if x_speed != 0.0 || y_speed != 0.0 {
                self.current_animation = 1;
            } else {
                self.current_animation = 0;
            }

            //
            if let Some(current_animation) = self.animations.get_mut(self.current_animation as usize) {
                current_animation.update(context.dt);
            
                if let Some(sprite) = context
                    .scene
                    .graph
                    .try_get_mut(self.sprite)
                    .and_then(|n| n.cast_mut::<Rectangle>())
                {
                    // Set new frame to the sprite.
                    sprite.set_uv_rect(
                        current_animation
                        .current_frame_uv_rect()
                        // .cloned()
                            .unwrap_or_default()
                            // .0,
                            //TODO:wtf
                    );
                }
            }
        }
    }

    // Returns unique script ID for serialization needs.
    fn id(&self) -> Uuid {
        Self::type_uuid()
    }
}
//! Game project.

use std::{path::Path};

use fyrox::{
    event_loop::ControlFlow,
    gui::message::UiMessage,
    core::{
        pool::{Handle, Pool},
        log::{Log, self}, algebra::{Vector3, UnitQuaternion}, futures::{self, executor::block_on}
    },
    plugin::{Plugin, PluginConstructor, PluginContext, PluginRegistrationContext},
    scene::{
        loader::AsyncSceneLoader, Scene, node::Node,
    }, event::Event, resource::model::{Model, ModelResourceExtension}, asset::manager::ResourceManager, script::ScriptMessagePayload,
};
mod player;
mod generation;
use generation::{import_biome, do_wave_function_collapse, Tile, Direction};
use player::Player;

use crate::generation::connects;



pub struct GameConstructor;

impl PluginConstructor for GameConstructor {
    fn register(&self, context: PluginRegistrationContext) {
        let script_constructors = &context.serialization_context.script_constructors;
        script_constructors.add::<Player>("Player");
    }

    fn create_instance(
        &self,
        override_scene: Handle<Scene>,
        context: PluginContext,
    ) -> Box<dyn Plugin> {
        Box::new(Game::new(override_scene, context))
    }
}

pub struct Game {
    scene: Handle<Scene>,
    loader: Option<AsyncSceneLoader>,
}

impl Game {
    pub fn new(override_scene: Handle<Scene>, context: PluginContext) -> Self {
        let mut loader = None;
        let scene = if override_scene.is_some() {
            override_scene
        } else {
            loader = Some(AsyncSceneLoader::begin_loading(
                "data/scene.rgs".into(),
                context.serialization_context.clone(),
                context.resource_manager.clone(),
            ));
            Default::default()
        };


        let _grid = block_on(create_grid(context.resource_manager.clone(),context.scenes.try_get_mut(scene).unwrap()));

        Self { scene, loader}
    }
}

async fn create_grid(
    resource_manager: ResourceManager,
    scene: &mut Scene,
) {
    
    let choices = import_biome("data/swampGeneration.txt".into()).unwrap();
    let mut grid = std::iter::repeat(Err(choices)).take(16*16).collect::<Vec<_>>();

    do_wave_function_collapse(&mut grid);
    
    for (i, tile) in grid.into_iter().enumerate() {
        let x = ((i as i32) % 16) as f32;
        let y = (i as f32/16.0).floor();
        let tile = tile.expect(&format!("Grid not collapsed! Cell at {:.} {:.}",x,y));
        let path_to_prefab = Path::new(&tile.prefab_path);
        let model = resource_manager.request::<Model, _>(&path_to_prefab).await.unwrap();
    
        model.instantiate_at(scene,Vector3::new(x, y, 0.0),UnitQuaternion::default());
    }
    
}

impl Plugin for Game {
    fn on_deinit(&mut self, _context: PluginContext) {
        // Do a cleanup here.
    }

    fn update(&mut self, context: &mut PluginContext, _control_flow: &mut ControlFlow) {  
        if let Some(loader) = self.loader.as_ref() {
            if let Some(result) = loader.fetch_result() {
                match result {
                    Ok(scene) => {
                        self.scene = context.scenes.add(scene);
                    }
                    Err(err) => Log::err(err),
                }
            }
        }
        // Add your global update code here.

        //TODO:if player near new chunk then load those chunks in a seperate thread
    }

    fn on_os_event(
        &mut self,
        _event: &Event<()>,
        _context: PluginContext,
        _control_flow: &mut ControlFlow,
    ) {
        // Do something on OS event here.
    }

    fn on_ui_message(
        &mut self,
        _context: &mut PluginContext,
        _message: &UiMessage,
        _control_flow: &mut ControlFlow,
    ) {
        
        //     tiles.spawn(create_grid(context.resource_manager.clone(),context.scenes.try_get_mut(self.scene).unwrap()));
        // Handle UI events here.
    }
}

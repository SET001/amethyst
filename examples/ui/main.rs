//! Displays a shaded sphere to the user.
use amethyst::assets as amethyst_assets;
use amethyst_assets::register_asset_type;
use amethyst_assets::AssetProcessorSystem;

use amethyst::{
    assets::{
        distill_importer,
        distill_importer::{typetag, SerdeImportable},
        Asset, AssetStorage, DefaultLoader, Handle, Loader, LoaderBundle,
    },
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle, Time},
    ecs::Entity,
    input::{is_close_requested, is_key_down, InputBundle},
    prelude::*,
    renderer::{
        plugins::RenderToWindow,
        rendy::{
            hal::command::ClearColor,
            mesh::{Normal, Position, TexCoord},
        },
        types::DefaultBackend,
        RenderingBundle,
    },
    shrev::{EventChannel, ReaderId},
    ui::{
        Anchor,
        FontAsset,
        LineMode,
        RenderUi,
        UiBundle,
        UiButtonBuilder,
        // UiCreator,
        // UiEvent,
        UiFinder,
        UiImage,
        UiLabel,
        UiText,
        UiTransform,
    },
    utils::{
        application_root_dir,
        fps_counter::{FpsCounter, FpsCounterBundle},
        // scene::BasicScenePrefab,
    },
    winit::event::VirtualKeyCode,
};
use log::info;
use serde::{Deserialize, Serialize};
use type_uuid::TypeUuid;
// type MyPrefabData = BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>;

#[derive(Default)]
struct Example {
    fps_display: Option<Entity>,
    random_text: Option<Entity>,
    ui_handle: Option<Handle<MyUI>>,
    font_handle: Option<Handle<FontAsset>>,
    ui: Option<MyUI>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct Label {
    id: u32,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize, TypeUuid, SerdeImportable)]
#[uuid = "d4adfc76-f5f4-40b0-8e28-8a51a12f5e46"]
struct MyUI {
    transform: UiTransform,
    text: UiText,
}
impl Asset for MyUI {
    fn name() -> &'static str {
        "EnergyBlast"
    }

    type Data = Self;
}

register_asset_type!(MyUI => MyUI; AssetProcessorSystem<MyUI>);

impl SimpleState for Example {
    fn on_start(&mut self, data: StateData<'_, GameData>) {
        let StateData {
            mut world,
            resources,
            ..
        } = data;

        // Make a button using the UiButtonBuilder.
        let (_button_id, _label) =
            UiButtonBuilder::<(), u32>::new(&"Made with UiButtonBuilder".to_string())
                .with_font_size(32.0)
                .with_position(0.0, -256.0)
                .with_size(64.0 * 6.0, 64.0)
                .with_anchor(Anchor::TopMiddle)
                .with_image(UiImage::SolidColor([0.8, 0.6, 0.3, 1.0]))
                .with_hover_image(UiImage::SolidColor([0.1, 0.1, 0.1, 0.5]))
                .build_from_world_and_resources(world, resources);

        let loader = resources.get::<DefaultLoader>().unwrap();
        self.ui_handle = Some(loader.load("ui/example.ron"));
        self.font_handle = Some(loader.load("font/square.ttf"));

        //     // initialize the scene with an object, a light and a camera.
        //     let handle = world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
        //         loader.load("prefab/sphere.ron", RonFormat, ())
        //     });
        //     world.create_entity().with(handle).build();
        //     init_output(&mut world);
        //     world.exec(|mut creator: UiCreator<'_>| {
        //         creator.create("ui/example.ron", ());
        //     });
    }

    fn handle_event(&mut self, _: StateData<'_, GameData>, event: StateEvent) -> SimpleTrans {
        match &event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(ui_event) => {
                // info!(
                //     "[HANDLE_EVENT] You just interacted with a ui element: {:?}",
                //     ui_event
                // );
                Trans::None
            }
            StateEvent::Input(input) => {
                // info!("Input Event detected: {:?}.", input);
                Trans::None
            }
        }
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData>) -> SimpleTrans {
        if self.ui.is_none() {
            let storage = state_data
                .resources
                .get::<AssetStorage<MyUI>>()
                .expect("AssetStorage<MyUI>");
            if let Some(ui) = storage.get(self.ui_handle.as_ref().expect("ui_handle.as_ref")) {
                self.ui = Some(ui.clone());
                let uitransform = ui.transform.clone();
                let uitext = ui.text.clone();

                state_data.world.push((uitext, uitransform));
            }
        }
        // let toolbar: Option<MyUI> = match storage.get(&toolbox_handle) {
        //     Some(toolbox) => Some(toolbox.clone()),
        //     None => None,
        // };
        //     let StateData { world, .. } = state_data;

        //     if self.fps_display.is_none() {
        //         world.exec(|finder: UiFinder<'_>| {
        //             if let Some(entity) = finder.find("fps") {
        //                 self.fps_display = Some(entity);
        //             }
        //         });
        //     }
        //     if self.random_text.is_none() {
        //         world.exec(|finder: UiFinder| {
        //             if let Some(entity) = finder.find("random_text") {
        //                 self.random_text = Some(entity);
        //             }
        //         });
        //     }

        //     let mut ui_text = world.write_storage::<UiText>();
        //     {
        //         if let Some(fps_display) = self.fps_display.and_then(|entity| ui_text.get_mut(entity)) {
        //             if world.read_resource::<Time>().frame_number() % 20 == 0 {
        //                 let fps = world.read_resource::<FpsCounter>().sampled_fps();
        //                 fps_display.text = format!("FPS: {:.*}", 2, fps);
        //             }
        //         }
        //     }

        //     {
        //         if let Some(random_text) = self.random_text.and_then(|entity| ui_text.get_mut(entity)) {
        //             if let Ok(value) = random_text.text.parse::<i32>() {
        //                 let mut new_value = value * 10;
        //                 if new_value > 100_000 {
        //                     new_value = 1;
        //                 }
        //                 random_text.text = new_value.to_string();
        //             } else {
        //                 random_text.text = String::from("1");
        //             }
        //         }
        //     }

        Trans::None
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let display_config_path = app_root.join("config/display.ron");
    let assets_dir = app_root.join("assets");
    let mut dispatcher = DispatcherBuilder::default();
    dispatcher
        .add_bundle(LoaderBundle)
        .add_bundle(TransformBundle)
        .add_bundle(InputBundle::new())
        .add_bundle(UiBundle::<u32>::new())
        // .with(Processor::<Source>::new(), "source_processor", &[])
        // .with_system_desc(UiEventHandlerSystemDesc::default(), "ui_event_handler", &[])
        .add_bundle(FpsCounterBundle::default())
        .add_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?.with_clear(ClearColor {
                        float32: [0.34, 0.36, 0.52, 1.0],
                    }),
                )
                .with_plugin(RenderUi::default()),
        );

    let game = Application::build(assets_dir, Example::default())?
        // Unlimited FPS
        .with_frame_limit(FrameRateLimitStrategy::Unlimited, 9999)
        .build(dispatcher)?;
    game.run();
    Ok(())
}

// /// This shows how to handle UI events.
// pub struct UiEventHandlerSystem {
//     reader_id: ReaderId<UiEvent>,
// }

// impl UiEventHandlerSystem {
//     pub fn new(reader_id: ReaderId<UiEvent>) -> Self {
//         Self { reader_id }
//     }
// }

// impl<'a> System for UiEventHandlerSystem {
//     type SystemData = Write<'a, EventChannel<UiEvent>>;

//     fn run(&mut self, events: Self::SystemData) {
//         // Reader id was just initialized above if empty
//         for ev in events.read(&mut self.reader_id) {
//             info!("[SYSTEM] You just interacted with a ui element: {:?}", ev);
//         }
//     }
// }

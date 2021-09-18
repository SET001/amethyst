use amethyst::{
    assets::{
        distill_importer,
        distill_importer::{typetag, SerdeImportable},
        register_asset_type, Asset, AssetProcessorSystem, AssetStorage, DefaultLoader, Format,
        Handle, Loader, LoaderBundle, TypeUuid,
    },
    error::{format_err, Error, ResultExt},
    prelude::*,
    renderer::{types::DefaultBackend, RenderingBundle},
    utils::application_root_dir,
};
use serde::{Deserialize, Serialize};

/// Custom asset representing an energy blast.
#[derive(Clone, Debug, Default, Deserialize, Serialize, TypeUuid, SerdeImportable)]
#[uuid = "a016abff-623d-48cf-a6e4-e76e069fe843"]
pub struct EnergyBlast {
    /// How much HP to subtract.
    pub hp_damage: u32,
    /// How much MP to subtract.
    pub mp_damage: u32,
}

impl Asset for EnergyBlast {
    fn name() -> &'static str {
        "EnergyBlast"
    }

    type Data = Self;
}

pub struct LoadingState {
    /// Handle to the energy blast.
    energy_blast_handle: Option<Handle<EnergyBlast>>,
}

use amethyst::assets as amethyst_assets;
register_asset_type!(EnergyBlast => EnergyBlast; AssetProcessorSystem<EnergyBlast>);

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData>) {
        let loader = data.resources.get::<DefaultLoader>().unwrap();
        self.energy_blast_handle = Some(loader.load("energy_blast.ron"));
    }

    fn update(&mut self, data: &mut StateData<'_, GameData>) -> SimpleTrans {
        let energy_blast_assets = data.resources.get::<AssetStorage<EnergyBlast>>().unwrap();
        if let Some(energy_blast) =
            energy_blast_assets.get(self.energy_blast_handle.as_ref().unwrap())
        {
            println!("Loaded energy blast: {:?}", energy_blast);
            Trans::Quit
        } else {
            Trans::None
        }
    }
}

fn main() -> amethyst::Result<()> {
    let config = amethyst::LoggerConfig {
        log_file: Some(std::path::PathBuf::from("asset_loading.log")),
        level_filter: amethyst::LogLevelFilter::Info,
        module_levels: vec![
            (
                "amethyst::assets".to_string(),
                amethyst::LogLevelFilter::Debug,
            ),
            (
                "distill_daemon".to_string(),
                amethyst::LogLevelFilter::Debug,
            ),
            (
                "distill_loader".to_string(),
                amethyst::LogLevelFilter::Trace,
            ),
        ],
        ..Default::default()
    };
    amethyst::start_logger(config);

    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets/");

    let mut builder = DispatcherBuilder::default();

    builder.add_bundle(LoaderBundle);
    builder.add_bundle(RenderingBundle::<DefaultBackend>::new());

    let game = Application::new(
        assets_dir,
        LoadingState {
            energy_blast_handle: None,
        },
        builder,
    )?;

    game.run();
    Ok(())
}

use crate::background::{
    handle_background_image, prepare_background, render_background, BackgroundPipeline,
    BackgroundRenderState,
};
use bevy::core_pipeline::{Core2d, Core2dSystems, Core3d, Core3dSystems};
use bevy::prelude::*;
use bevy::render::extract_resource::ExtractResourcePlugin;
use bevy::render::{Render, RenderApp, RenderSystems};

pub use nokhwa;

mod background;
pub use background::BackgroundImage;
pub mod camera;

pub struct BevyNokhwaPlugin;

impl Plugin for BevyNokhwaPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BackgroundImage(Image::default()))
            .add_plugins(ExtractResourcePlugin::<BackgroundImage>::default())
            .add_systems(Update, handle_background_image);

        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .init_resource::<BackgroundRenderState>()
            .add_systems(
                Render,
                prepare_background.in_set(RenderSystems::PrepareResources),
            )
            .add_systems(Core3d, render_background.before(Core3dSystems::MainPass))
            .add_systems(Core2d, render_background.before(Core2dSystems::MainPass));
    }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app.init_resource::<BackgroundPipeline>();
    }
}

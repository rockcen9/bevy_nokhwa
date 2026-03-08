use crate::background::handle_background_image;
use bevy::prelude::*;
use bevy::render::extract_resource::ExtractResourcePlugin;

#[cfg(not(feature = "capture_only"))]
use crate::background::{
    prepare_background, render_background, BackgroundPipeline, BackgroundRenderState,
};
#[cfg(not(feature = "capture_only"))]
use bevy::core_pipeline::{Core2d, Core2dSystems, Core3d, Core3dSystems};
#[cfg(not(feature = "capture_only"))]
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

        #[cfg(not(feature = "capture_only"))]
        {
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
    }

    fn finish(&self, #[cfg_attr(feature = "capture_only", allow(unused_variables))] app: &mut App) {
        #[cfg(not(feature = "capture_only"))]
        {
            let render_app = app.sub_app_mut(RenderApp);
            render_app.init_resource::<BackgroundPipeline>();
        }
    }
}

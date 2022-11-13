use bevy::{
    prelude::*,
    render::{camera::RenderTarget, render_graph::RenderGraph, RenderApp},
    window::{PresentMode, WindowRef, WindowResolution},
};
use bevy_egui::{EguiContext, EguiPlugin};

#[derive(Resource)]
struct SecondWindow(Entity);

#[derive(Resource)]
struct Images {
    bevy_icon: Handle<Image>,
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .init_resource::<SharedUiState>()
        .add_startup_system(load_assets_system)
        .add_startup_system(create_new_window_system)
        .add_system(ui_first_window_system)
        .add_system(ui_second_window_system);

    let second_window_entity = app
        .world
        .spawn(Window {
            resolution: WindowResolution::new(800., 600.),
            present_mode: PresentMode::AutoVsync,
            title: "Second window".to_string(),
            ..Default::default()
        })
        .id();
    app.insert_resource(SecondWindow(second_window_entity));

    let render_app = app.sub_app_mut(RenderApp);
    let mut graph = render_app.world.get_resource_mut::<RenderGraph>().unwrap();

    bevy_egui::setup_pipeline(
        &mut graph,
        bevy_egui::RenderGraphConfig {
            window_entity: second_window_entity,
            egui_pass: SECONDARY_EGUI_PASS,
        },
    );

    app.run();
}

const SECONDARY_EGUI_PASS: &str = "secondary_egui_pass";

fn create_new_window_system(mut commands: Commands, second_window: Res<SecondWindow>) {
    // second window camera
    commands.spawn(Camera3dBundle {
        camera: Camera {
            target: RenderTarget::Window(WindowRef::Entity(second_window.0)),
            ..Default::default()
        },
        transform: Transform::from_xyz(6.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn load_assets_system(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(Images {
        bevy_icon: assets.load("icon.png"),
    });
}

#[derive(Default)]
struct UiState {
    input: String,
}

#[derive(Default, Resource)]
struct SharedUiState {
    shared_input: String,
}

fn ui_first_window_system(
    mut egui_context: ResMut<EguiContext>,
    mut ui_state: Local<UiState>,
    mut shared_ui_state: ResMut<SharedUiState>,
    images: Res<Images>,
) {
    let bevy_texture_id = egui_context.add_image(images.bevy_icon.clone_weak());
    egui::Window::new("First Window")
        .vscroll(true)
        .show(egui_context.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut ui_state.input);
            });
            ui.horizontal(|ui| {
                ui.label("Shared input: ");
                ui.text_edit_singleline(&mut shared_ui_state.shared_input);
            });

            ui.add(egui::widgets::Image::new(bevy_texture_id, [256.0, 256.0]));
        });
}

fn ui_second_window_system(
    second_window: Res<SecondWindow>,
    mut egui_context: ResMut<EguiContext>,
    mut ui_state: Local<UiState>,
    mut shared_ui_state: ResMut<SharedUiState>,
    images: Res<Images>,
) {
    let bevy_texture_id = egui_context.add_image(images.bevy_icon.clone_weak());
    let ctx = match egui_context.try_ctx_for_window_mut(second_window.0) {
        Some(ctx) => ctx,
        None => return,
    };
    egui::Window::new("Second Window")
        .vscroll(true)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Write something else: ");
                ui.text_edit_singleline(&mut ui_state.input);
            });
            ui.horizontal(|ui| {
                ui.label("Shared input: ");
                ui.text_edit_singleline(&mut shared_ui_state.shared_input);
            });

            ui.add(egui::widgets::Image::new(bevy_texture_id, [256.0, 256.0]));
        });
}

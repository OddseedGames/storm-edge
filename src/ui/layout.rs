use super::*;

pub(super) fn setup(mut commands: Commands, assets: Res<AssetServer>, scene: Res<SceneTarget>) {
    let font = assets.load("fonts/IBMPlexMono-Regular.ttf");
    commands.spawn((
        ImageNode::new(scene.image.clone()),
        Node {
            position_type: PositionType::Absolute,
            width: percent(100),
            height: percent(100),
            ..default()
        },
        GlobalZIndex(-10),
        UiTargetCamera(scene.ui_camera),
    ));
    commands
        .spawn((
            Item::Root,
            UiTargetCamera(scene.ui_camera),
            Node {
                position_type: PositionType::Absolute,
                width: percent(100),
                height: percent(100),
                ..default()
            },
        ))
        .with_children(|root| {
            menus::intro(root, &font);
            hud::spawn(root, &font);
            menus::overlay(root, &font);
        });
}

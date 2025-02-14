use bevy::prelude::*;
use bevy::render::RenderPlugin;
use lightyear::prelude::client::Interpolated;

use crate::protocol::*;

#[derive(Clone)]
pub struct ExampleRendererPlugin;

impl Plugin for ExampleRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);
        // app.add_systems(Update, draw_boxes);
        app.add_systems(Update, handle_sprite_movement);
    }
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2d);
}

/// System that draws the boxes of the player positions.
/// The components should be replicated from the server to the client
// pub(crate) fn draw_boxes(mut gizmos: Gizmos, players: Query<(&PlayerPosition, &PlayerColor)>) {
//     for (position, color) in &players {
//         gizmos.rect_2d(
//             Isometry2d::from_translation(position.0),
//             Vec2::ONE * 50.0,
//             color.0,
//         );
//     }
// }

// for (position, mut transform) in &mut position_query {
//     transform.translation = Vec3::new(position.0.x, position.0.y, 0.0);
// }

pub(crate) fn handle_sprite_movement(
    mut players: Query<(&PlayerPosition, &mut Transform), (With<Sprite>, With<Interpolated>)>,
) {
    for (position, mut transform) in &mut players {
        transform.translation = Vec3::new(position.0.x, position.0.y, 0.0);
    }
}

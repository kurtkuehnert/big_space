//! Contains tools for debugging the floating origin.

use bevy::{
    color::palettes::basic::{BLUE, GREEN, RED},
    prelude::*,
};
use std::marker::PhantomData;

use crate::{
    precision::GridPrecision,
    reference_frame::{local_origin::ReferenceFrames, ReferenceFrame},
    FloatingOrigin, GridCell,
};

/// This plugin will render the bounds of occupied grid cells.
#[derive(Default)]
pub struct FloatingOriginDebugPlugin<P: GridPrecision>(PhantomData<P>);
impl<P: GridPrecision> Plugin for FloatingOriginDebugPlugin<P> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (update_debug_bounds::<P>, update_reference_frame_axes::<P>)
                .chain()
                .after(bevy::transform::TransformSystem::TransformPropagate),
        );
    }
}

/// Update the rendered debug bounds to only highlight occupied [`GridCell`]s.
pub fn update_debug_bounds<P: GridPrecision>(
    mut gizmos: Gizmos,
    reference_frames: ReferenceFrames<P>,
    occupied_cells: Query<(Entity, &GridCell<P>, Option<&FloatingOrigin>)>,
) {
    for (cell_entity, cell, origin) in occupied_cells.iter() {
        let Some(frame) = reference_frames.parent_frame(cell_entity) else {
            continue;
        };
        let transform = frame.global_transform(
            cell,
            &Transform::from_scale(Vec3::splat(frame.cell_edge_length() * 0.999)),
        );
        if origin.is_none() {
            gizmos.cuboid(transform, GREEN)
        } else {
            // gizmos.cuboid(transform, Color::rgba(0.0, 0.0, 1.0, 0.5))
        }
    }
}

/// Draw axes for reference frames.
pub fn update_reference_frame_axes<P: GridPrecision>(
    mut gizmos: Gizmos,
    frames: Query<(&GlobalTransform, &ReferenceFrame<P>)>,
) {
    for (transform, frame) in frames.iter() {
        let start = transform.translation();
        let len = frame.cell_edge_length() * 2.0;
        gizmos.ray(start, transform.right() * len, RED);
        gizmos.ray(start, transform.up() * len, GREEN);
        gizmos.ray(start, transform.back() * len, BLUE);
    }
}

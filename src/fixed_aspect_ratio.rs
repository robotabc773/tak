//! Provides a `FixedAspectRatio` component that manages a `Node` to ensure it
//! fills its parent as much as possible while maintaining a fixed aspect ratio.

use bevy::prelude::*;

/// This plugin must be added for `FixedAspectRatio` to work
pub struct FixedAspectRatioPlugin;

impl Plugin for FixedAspectRatioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_fixed_aspect_ratio);
    }
}

/// Ensures that the `Node` on this entity always has the aspect ratio of
/// `Node.aspect_ratio`. This is accomplished by overriding:
/// - `Node.width`
/// - `Node.min_width`
/// - `Node.max_width`
/// - `Node.height`
/// - `Node.min_height`
/// - `Node.max_height`
///
/// to either `Val::Auto` or `Val::Percent(100.)`, depending on if the containing
/// area is wider than it is tall or taller than it is wide. Possibly has a one
/// frame delay, but because this only changes when the viewport resizes anyways,
/// it isn't a very big deal.
#[derive(Component)]
#[require(Node)]
pub struct FixedAspectRatio;

fn update_fixed_aspect_ratio(
    root_query: Query<
        (&mut Node, &ComputedNodeTarget),
        (
            Without<ChildOf>,
            With<FixedAspectRatio>,
            Changed<ComputedNodeTarget>,
        ),
    >,
    changed_parent_query: Query<&ComputedNode, (With<Node>, With<Children>, Changed<ComputedNode>)>,
    child_query: Query<(&mut Node, &ChildOf), With<FixedAspectRatio>>,
) {
    for (mut node, computed_node_target) in root_query {
        let parent_size = computed_node_target.logical_size();
        info!("root: {}", parent_size);
        fix_aspect_ratio(&mut node, parent_size);
    }

    for (mut node, child_of) in child_query {
        if let Ok(computed_parent) = changed_parent_query.get(child_of.parent()) {
            let parent_size = computed_parent.size() * computed_parent.inverse_scale_factor();
            info!("parent: {}", parent_size);
            fix_aspect_ratio(&mut node, parent_size);
        }
    }
}

fn fix_aspect_ratio(node: &mut Node, parent_size: Vec2) {
    if parent_size.x > parent_size.y {
        node.width = Val::Auto;
        node.min_width = Val::Auto;
        node.max_width = Val::Auto;
        node.height = Val::Percent(100.);
        node.min_height = Val::Percent(100.);
        node.max_height = Val::Percent(100.);
    } else {
        node.width = Val::Percent(100.);
        node.min_width = Val::Percent(100.);
        node.max_width = Val::Percent(100.);
        node.height = Val::Auto;
        node.min_height = Val::Auto;
        node.max_height = Val::Auto;
    }
}

use crate::*;
use bevy::{
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

pub struct ClosestBeam(pub BeamColor);
pub fn update_closest_beam(
    player: Query<&Transform, (With<Player>, Without<BeamColor>)>,
    beams: Query<(&Pivot, &BeamColor)>,
    mut color: ResMut<ClosestBeam>,
) {
    let pos = player.single().translation.xy();
    let mut beams = beams.iter().collect::<Vec<_>>();
    beams.sort_unstable_by(|a, b| {
        b.0 .0
            .distance(pos)
            .partial_cmp(&a.0 .0.distance(pos))
            .unwrap()
    });
    color.0 = *beams.pop().unwrap().1;
}

pub fn move_light_beam(
    mut query: Query<(&mut Transform, &Pivot, &BeamColor)>,
    color: Res<ClosestBeam>,
    mouse: Res<MousePos>,
) {
    for (mut trans, pivot, beam) in &mut query {
        if *beam == color.0 {
            //
            let diff =
                Vec2::new((mouse.pos.x - pivot.0.x).abs(), mouse.pos.y - pivot.0.y).normalize();
            let angle = diff.angle_between(Vec2::X);

            trans.translation = (pivot.0 + diff * BEAM_LENGTH / 2.0).extend(0.0);
            trans.rotation = Quat::from_rotation_z(-angle);
        }
    }
}

impl Material2d for BeamMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/beam_material.wgsl".into()
    }
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct BeamMaterial {
    #[uniform(0)]
    pub color: Color,
    #[uniform(0)]
    pub time: f32,
    #[uniform(0)]
    pub offset: f32,
    #[uniform(0)]
    pub selected: f32,
}

pub fn update_beam_material(
    query: Query<(&Handle<BeamMaterial>, &BeamColor)>,
    mut a: ResMut<Assets<BeamMaterial>>,
    time: Res<Time>,
    color: Res<ClosestBeam>,
) {
    for (handle, beam) in &query {
        if let Some(mat) = a.get_mut(handle) {
            mat.time = time.seconds_since_startup() as f32;

            mat.selected = (*beam == color.0).then_some(1.0).unwrap_or(0.0);
        }
    }
}

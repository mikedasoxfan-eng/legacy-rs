//! Entity, EntityLiving and vehicle packet callbacks from Beta 1.8.1.
use super::{Entity, EntityKind, Vec3D};
use crate::World;

#[derive(Clone, Debug, Default)]
pub struct NetworkState {
    pub increments: i32,
    pub position: Vec3D,
    pub yaw: f64,
    pub pitch: f64,
    pub velocity: Vec3D,
    /// EntityLiving.isMultiplayerEntity suppresses autonomous action state.
    pub multiplayer_entity: bool,
}

impl Entity {
    pub fn set_rotation(&mut self, yaw: f32, pitch: f32) {
        self.yaw = yaw % 360.0;
        self.pitch = pitch % 360.0;
    }

    pub fn set_velocity(&mut self, x: f64, y: f64, z: f64) {
        if matches!(self.kind, EntityKind::Fish(..)) {
            self.set_fish_velocity(Vec3D::new(x, y, z));
            return;
        }
        self.motion = Vec3D::new(x, y, z);
        if matches!(self.kind, EntityKind::Boat(..) | EntityKind::Minecart(..)) {
            self.network.velocity = self.motion;
        }
    }

    pub fn set_position_and_rotation2(
        &mut self,
        w: &mut World,
        p: Vec3D,
        yaw: f32,
        pitch: f32,
        increments: i32,
    ) {
        if matches!(self.kind, EntityKind::Fish(..)) {
            self.interpolate_fish(p, yaw, pitch, increments);
            return;
        }
        if self.kind.is_living()
            || matches!(self.kind, EntityKind::Boat(..) | EntityKind::Minecart(..))
        {
            self.network.position = p;
            self.network.yaw = yaw as f64;
            self.network.pitch = pitch as f64;
            self.network.increments = increments.wrapping_add(match self.kind {
                EntityKind::Boat(..) => 4,
                EntityKind::Minecart(..) => 2,
                _ => 0,
            });
            if self.kind.is_living() {
                self.y_offset = 0.0;
            } else {
                self.motion = self.network.velocity;
            }
        } else {
            self.set_position(p.x, p.y, p.z);
            self.set_rotation(yaw, pitch);
            self.raise_after_network_collision(w);
        }
    }

    fn raise_after_network_collision(&mut self, w: &mut World) {
        let boxes = w.get_colliding_bounding_boxes(
            Some(self.entity_id),
            self.bounding_box.contract(1.0 / 32.0, 0.0, 1.0 / 32.0),
        );
        if !boxes.is_empty() {
            let mut top = 0.0;
            for b in boxes {
                if b.max_y > top {
                    top = b.max_y;
                }
            }
            self.set_position(
                self.pos.x,
                self.pos.y + (top - self.bounding_box.min_y),
                self.pos.z,
            );
        }
    }

    pub(super) fn interpolate_network(&mut self, w: &mut World, collide: bool) {
        if self.network.increments <= 0 {
            return;
        }
        let n = self.network.increments as f64;
        let p = Vec3D::new(
            self.pos.x + (self.network.position.x - self.pos.x) / n,
            self.pos.y + (self.network.position.y - self.pos.y) / n,
            self.pos.z + (self.network.position.z - self.pos.z) / n,
        );
        let mut yaw = self.network.yaw - self.yaw as f64;
        while yaw < -180.0 {
            yaw += 360.0;
        }
        while yaw >= 180.0 {
            yaw -= 360.0;
        }
        self.yaw = (self.yaw as f64 + yaw / n) as f32;
        self.pitch = (self.pitch as f64 + (self.network.pitch - self.pitch as f64) / n) as f32;
        self.network.increments -= 1;
        self.set_position(p.x, p.y, p.z);
        self.set_rotation(self.yaw, self.pitch);
        if collide {
            self.raise_after_network_collision(w);
        }
    }
}

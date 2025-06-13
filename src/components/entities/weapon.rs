use bevy::prelude::*;
use crate::components::attributes::{Speed, Collider};

/// Marker component for weapons
#[derive(Component, Debug)]
pub struct Weapon {
    /// Rate of fire in shots per second
    pub fire_rate: f32,
    /// Timer to track when weapon can fire next
    pub fire_timer: Timer,
    /// Damage dealt by projectiles from this weapon
    pub damage: f32,
    /// Speed of projectiles fired by this weapon
    pub projectile_speed: f32,
    /// Whether the weapon is currently firing (for automatic weapons)
    pub is_firing: bool,
}

impl Weapon {
    pub fn new(fire_rate: f32, damage: f32, projectile_speed: f32) -> Self {
        Self {
            fire_rate,
            fire_timer: Timer::from_seconds(1.0 / fire_rate, TimerMode::Repeating),
            damage,
            projectile_speed,
            is_firing: false,
        }
    }
    
    pub fn can_fire(&self) -> bool {
        self.fire_timer.finished()
    }
    
    pub fn fire(&mut self) {
        self.fire_timer.reset();
    }
    
    pub fn update(&mut self, delta_time: std::time::Duration) {
        self.fire_timer.tick(delta_time);
    }
}

/// Marker component for projectiles
#[derive(Component, Debug)]
pub struct Projectile {
    /// Damage dealt by this projectile
    pub damage: f32,
    /// Direction the projectile is moving
    pub direction: Vec3,
    /// Lifetime of the projectile in seconds
    pub lifetime: f32,
    /// Timer to track when projectile should despawn
    pub lifetime_timer: Timer,
}

impl Projectile {
    pub fn new(damage: f32, direction: Vec3, lifetime: f32) -> Self {
        Self {
            damage,
            direction: direction.normalize_or_zero(),
            lifetime,
            lifetime_timer: Timer::from_seconds(lifetime, TimerMode::Once),
        }
    }
    
    pub fn update(&mut self, delta_time: std::time::Duration) -> bool {
        self.lifetime_timer.tick(delta_time);
        self.lifetime_timer.finished()
    }
}

/// Bundle for weapon entities
#[derive(Bundle)]
pub struct WeaponBundle {
    pub weapon: Weapon,
    pub transform: Transform,
}

impl WeaponBundle {
    pub fn new(fire_rate: f32, damage: f32, projectile_speed: f32) -> Self {
        Self {
            weapon: Weapon::new(fire_rate, damage, projectile_speed),
            transform: Transform::default(),
        }
    }
}

/// Bundle for projectile entities
#[derive(Bundle)]
pub struct ProjectileBundle {
    pub projectile: Projectile,
    pub speed: Speed,
    pub collider: Collider,
    pub transform: Transform,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
}

impl ProjectileBundle {
    pub fn new(
        damage: f32,
        direction: Vec3,
        speed: f32,
        lifetime: f32,
        position: Vec3,
        mesh: Handle<Mesh>,
        material: Handle<ColorMaterial>,
    ) -> Self {
        Self {
            projectile: Projectile::new(damage, direction, lifetime),
            speed: Speed::new(speed),
            collider: Collider::new(3.0), // Small projectile radius
            transform: Transform::from_translation(position),
            mesh: Mesh2d(mesh),
            material: MeshMaterial2d(material),
        }
    }
} 
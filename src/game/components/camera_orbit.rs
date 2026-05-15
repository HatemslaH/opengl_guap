/// Orbital control of the camera with keyboard: **A/D** — azimuth around the target, **W/S** — pitch.
///
/// Put on the same entity as [`Position`], [`Camera`], [`CameraLookTarget`]. System
/// [`crate::ecs::systems::camera_keyboard_orbit_system`] (before [`crate::ecs::systems::camera_look_at_system`])
/// updates [`Position`] on the sphere around the target; the look is still set by look-at.
#[derive(Clone, Debug)]
pub struct CameraKeyboardOrbit {
    pub enabled: bool,
    /// Speed of azimuth when holding Q or E (degrees per second).
    pub yaw_speed_deg_per_sec: f32,
    /// Speed of pitch when holding W or S (degrees per second).
    pub pitch_speed_deg_per_sec: f32,
    pub pitch_min_deg: f32,
    pub pitch_max_deg: f32,
    /// Current azimuth and pitch (same convention as [`camera_yaw_pitch_deg_from_look_direction`](crate::graphics::camera_yaw_pitch_deg_from_look_direction)).
    pub yaw_deg: f32,
    pub pitch_deg: f32,
    pub distance: f32,
    /// After the first frame with a valid target, the angles and distance are taken from the current camera position.
    pub initialized: bool,
}

impl Default for CameraKeyboardOrbit {
    fn default() -> Self {
        Self {
            enabled: true,
            yaw_speed_deg_per_sec: 72.0,
            pitch_speed_deg_per_sec: 56.0,
            pitch_min_deg: -85.0,
            pitch_max_deg: 85.0,
            yaw_deg: 0.0,
            pitch_deg: 0.0,
            distance: 4.0,
            initialized: false,
        }
    }
}

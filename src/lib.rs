/// Torpedo is a trait that defines the interface for a plugin that can be loaded by `torpedo`.
pub trait Torpedo {
    /// The name of the torpedo type
    fn name(&self) -> &str;

    /// Load the torpedo into the specified bay.
    fn load(&self, bay: i32) -> bool;

    /// Target the torpedo at the specified coordinates.
    fn target(&self, bay: i32, x: f32, y: f32, z: f32) -> bool;

    /// Fire the torpedo!
    fn fire(&self, bay: i32) -> bool;

    /// Track the torpedo, firing the callback with the current status of the torpedo, until it hits or misses its target.
    fn track(&self, bay: i32, f: fn(String)) -> bool;
}

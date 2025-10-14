use torpedo::Torpedo;

// A sample implementation of a torpedo
pub struct AntimatterTorpedo;

impl Torpedo for AntimatterTorpedo {
    fn name(&self) -> &str {
        "Antimatter Torpedo"
    }

    fn load(&self, _bay: i32) -> bool {
        true
    }

    fn target(&self, _bay: i32, _x: f32, _y: f32, _z: f32) -> bool {
        true
    }

    fn fire(&self, _bay: i32) -> bool {
        true
    }

    fn track(&self, bay: i32, f: fn(String)) -> bool {
        println!("Tracking torpedo fired from bay {}...", bay);
        f("Calculating...".to_string());
        false
    }
}

// Export a function to create an instance of the plugin
#[unsafe(no_mangle)]
pub fn create_plugin() -> *mut dyn Torpedo {
    Box::into_raw(Box::new(AntimatterTorpedo))
}

use torpedo::Torpedo;

// A sample implementation of a torpedo
pub struct PhotonTorpedo;

impl Torpedo for PhotonTorpedo {
    fn name(&self) -> &str {
        "Photon Torpedo"
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
        let delay = std::time::Duration::from_millis(250);

        println!("Tracking torpedo fired from bay {}...", bay);
        std::thread::sleep(delay);
        f("25%".to_string());
        std::thread::sleep(delay);
        f("50%".to_string());
        std::thread::sleep(delay);
        f("75%".to_string());
        std::thread::sleep(delay);
        f("100%".to_string());
        true
    }
}

// Export a function to create an instance of the plugin
#[unsafe(no_mangle)]
pub fn create_plugin() -> *mut dyn Torpedo {
    Box::into_raw(Box::new(PhotonTorpedo))
}

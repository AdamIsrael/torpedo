use libloading::{Library, Symbol};
use torpedo::Torpedo;

static PLUGIN_DIRS: &[&str] = &[
    "/usr/lib/torpedo/plugins",
    "/usr/local/lib/torpedo/plugins",
    "~/.local/lib/torpedo/plugins",
];

fn find_plugins() -> Vec<String> {
    let mut plugins = Vec::<String>::new();

    for dir in PLUGIN_DIRS {
        let mut path = dir.to_string();

        if path.starts_with("~") {
            path = shellexpand::tilde(&path).to_string();
        }

        // Scan the files in each directory for .so files
        if std::fs::exists(&path).unwrap_or(false) {
            for entry in std::fs::read_dir(&path).unwrap() {
                let entry = entry.unwrap();
                if entry.file_type().unwrap().is_file()
                    && entry.file_name().to_str().unwrap().ends_with(".so")
                {
                    let plugin = format!(
                        "{}/{}",
                        path,
                        entry.file_name().to_str().unwrap().to_string()
                    );

                    plugins.push(plugin);
                }
            }
        }
    }
    plugins
}

// fn process_closure(action: Box<dyn Fn()>) {
//     println!("Processing closure...");
//     action(); // Call the closure
// }

// fn track(_i: i32) {
//     println!("Tracking....");
//     // f();
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Iterate through installed plugins
    let plugins = find_plugins();
    let mut bay = 1;

    for plugin in &plugins {
        println!("Plugin: {}", plugin);
        unsafe {
            let lib = Library::new(plugin)?; // Adjust for your OS
            let create_plugin: Symbol<unsafe extern "C" fn() -> *mut dyn Torpedo> =
                lib.get(b"create_plugin\0")?;

            let plugin_ptr = create_plugin();
            let plugin: Box<dyn Torpedo> = Box::from_raw(plugin_ptr); // Reclaim ownership

            println!("Plugin Name: {}", plugin.name());

            let (x, y, z) = (1.0, 2.0, 3.0);

            plugin.load(bay);
            plugin.target(bay, x, y, z);
            plugin.fire(bay);

            // Test execution w/status
            let tracker = |progress: String| {
                println!("Progress: {}", progress);
            };
            if plugin.track(bay, tracker) {
                println!("Direct hit!");
            } else {
                println!("Missed!");
            }
            bay += 1;
        }
    }

    Ok(())
}

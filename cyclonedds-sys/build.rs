use std::env;
use std::path::{Path, PathBuf};

/// Wrapper to override bindgen callbacks so that link names for items ending in `_bindgen_wrapper`
/// can be intercepted and mapped.
#[derive(Debug)]
struct BindgenWrapperCallbacks {
    /// Stores the list of names mapped during the generate name override phase so that they can
    /// be correctly emitted during link name override phase.
    mapped_names: std::cell::RefCell<Vec<String>>,
}

impl BindgenWrapperCallbacks {
    /// Create a new wrapper with no mapped names.
    pub fn new() -> Self {
        Self {
            mapped_names: Default::default(),
        }
    }
}

impl bindgen::callbacks::ParseCallbacks for BindgenWrapperCallbacks {
    fn generated_name_override(
        &self,
        item_info: bindgen::callbacks::ItemInfo<'_>,
    ) -> Option<String> {
        match (
            &item_info.kind,
            item_info.name.rsplit_once("_bindgen_wrapper"),
        ) {
            (bindgen::callbacks::ItemKind::Function, Some((start, _))) => {
                let name = start.to_string();
                self.mapped_names.borrow_mut().push(name.clone());
                Some(name)
            }
            _ => None,
        }
    }

    fn generated_link_name_override(
        &self,
        item_info: bindgen::callbacks::ItemInfo<'_>,
    ) -> Option<String> {
        let name = item_info.name.into();

        match item_info.kind {
            bindgen::callbacks::ItemKind::Function
                if self.mapped_names.borrow().contains(&name) =>
            {
                Some(name)
            }
            _ => None,
        }
    }
}

#[cfg(target_os = "macos")]
/// Fix the exported bindings on macOS.
fn platform_fixup(bindings_path: &Path) {
    let contents = std::fs::read_to_string(bindings_path).expect("could not read bindings.rs");
    // Fixes an issue under macOS in bindgen: https://github.com/rust-lang/rust-bindgen/issues/1725
    let cleaned = contents.replace("#[link_name = \"\\u{1}dds", "#[link_name = \"dds");
    std::fs::write(&bindings_path, cleaned).expect("could not write cleaned bindings.rs");
}

#[cfg(not(target_os = "macos"))]
/// Fix the exported bindings (a no-op for this platform).
fn platform_fixup(_bindings_path: &Path) {}

fn main() {
    println!("cargo:rustc-link-lib=ddsc");

    // Prepare the bindings.
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(BindgenWrapperCallbacks::new()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .wrap_static_fns(true)
        .generate_cstr(true)
        .derive_default(true)
        .disable_untagged_union()
        .formatter(bindgen::Formatter::Prettyplease)
        .layout_tests(true)
        .generate()
        .expect("unable to generate bindings");

    // Prepare the output directory.
    let out_path = PathBuf::from(
        env::var("OUT_DIR").expect("could not detect 'OUT_DIR' environment variable"),
    );

    // Write the preliminary bindings.
    let bindings_path = out_path.join("bindings.rs");
    bindings
        .write_to_file(&bindings_path)
        .expect("could not write bindings");

    // Fix up the bindings as needed for the target.
    platform_fixup(&bindings_path);
}

use bevy::prelude::*;

// --- PUBLIC INTERFACE ---

/// Saves a byte array to a file.
/// On Native: Opens a system "Save As" dialog.
/// On Web: Triggers a browser download.
pub fn save_sculpt_map(image_data: Vec<u8>, default_name: &str) {
    save_file_impl(image_data, default_name);
}

// --- NATIVE IMPLEMENTATION ---

#[cfg(not(target_arch = "wasm32"))]
fn save_file_impl(data: Vec<u8>, default_name: &str) {
    use rfd::FileDialog;
    use std::fs::write;

    // 1. Create an owned String copy of the name
    let name = default_name.to_string();

    std::thread::spawn(move || {
        if let Some(path) = FileDialog::new()
            .set_file_name(name) // 2. Use the owned String here
            .add_filter("TGA Image", &["tga"])
            .save_file()
        {
            if let Err(e) = write(path, data) {
                eprintln!("Failed to save file: {}", e);
            } else {
                println!("File saved successfully.");
            }
        }
    });
}

// --- WASM IMPLEMENTATION ---

#[cfg(target_arch = "wasm32")]
fn save_file_impl(data: Vec<u8>, default_name: &str) {
    use wasm_bindgen::JsCast;
    use web_sys::{Blob, BlobPropertyBag, HtmlAnchorElement, Url};

    // 1. Create a Blob from the data
    let parts = js_sys::Array::new();
    let uint8_array = js_sys::Uint8Array::from(&data[..]);
    parts.push(&uint8_array);

    let mut props = BlobPropertyBag::new();
    props.type_("image/tga");

    let blob = Blob::new_with_u8_array_sequence_and_options(&parts, &props)
        .expect("Failed to create blob");

    // 2. Create a URL for the Blob (e.g. blob:http://localhost:...)
    let url = Url::create_object_url_with_blob(&blob).expect("Failed to create object URL");

    // 3. Create an invisible <a> tag
    let window = web_sys::window().expect("Failed to get window");
    let document = window.document().expect("Failed to get document");
    let a: HtmlAnchorElement = document
        .create_element("a")
        .expect("Failed to create anchor")
        .dyn_into()
        .expect("Failed to cast to HtmlAnchorElement");

    // 4. Configure the download
    a.set_href(&url);
    a.set_download(default_name);
    a.style().set_property("display", "none").unwrap();

    // 5. Append, Click, Remove
    document.body().unwrap().append_child(&a).unwrap();
    a.click();
    document.body().unwrap().remove_child(&a).unwrap();

    // 6. Clean up the URL to free memory
    Url::revoke_object_url(&url).unwrap();

    info!("Download triggered for {}", default_name);
}

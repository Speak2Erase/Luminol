// wit_bindgen generates an unsafe extern "C" fn with no unsafe block inside it, so we NEED to allow this
#![allow(unsafe_op_in_unsafe_fn)]

wit_bindgen::generate!({
    world: "test"
});

struct TheGuest;

impl Guest for TheGuest {
    fn run() {
        print("Hello from a plugin!");
    }
}

export!(TheGuest);

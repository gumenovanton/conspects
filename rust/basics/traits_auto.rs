DEFAULT
// when i want to define some fields from struct, and other want to set as default
#[derive(Default)]
struct Config {
    timeout: u32,
    retries: u8,
    debug: bool,
}

// instead:
let config1 = Config {
    timeout: 30,
    retries: 3,
    debug: false,
};

// i can do like this:
let config2 = Config {
    timeout: 30,
    ..Default::default()  // set default values of type
};

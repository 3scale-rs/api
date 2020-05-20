use vergen::{ConstantsFlags, generate_cargo_keys};
use autocfg;

fn main() {
    let ac = autocfg::new();
    ac.emit_has_path("std::string");

    // Setup the flags, toggling off the 'SEMVER' flag
    let mut flags = ConstantsFlags::all();
    flags.toggle(ConstantsFlags::SEMVER);

    // Generate the 'cargo:' key output
    generate_cargo_keys(ConstantsFlags::all()).expect("Unable to generate the cargo keys!");
}
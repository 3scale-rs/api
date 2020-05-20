const fn info() -> &'static str {
    concat!(
        env!("CARGO_PKG_NAME"),
        " ",
        env!("CARGO_PKG_VERSION"),
        " (",
        env!("VERGEN_BUILD_DATE"),
        ") - ",
        env!("VERGEN_SEMVER_LIGHTWEIGHT"),
        " for ",
        env!("VERGEN_TARGET_TRIPLE"),
        "\n"
    )
}

pub fn write_info<W: std::io::Write>(out: &mut W) -> std::io::Result<()> {
    write!(out, "{}\n", info())
}

pub fn setup() {
    human_panic::setup_panic!();
}

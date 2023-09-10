use std::error::Error;
use vergen::EmitBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    // Emit the instructions
    EmitBuilder::builder()
        // each of the following three methods prepares a set of VERGEN environment variables
        .all_build() // prepares the "VERGEN_BUILD_*" environment variables
        .all_rustc() // prepares the "VERGEN_RUSTC_*" environment variables
        .all_git() // prepares the "VERGEN_GIT_*" environment variables
        .emit_and_set()?; // actually sets the environment variables
    Ok(())
}

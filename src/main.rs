#[cfg(feature = "bin")]
mod cli;

fn main() {
    #[cfg(feature = "bin")]
    cli::run()
}

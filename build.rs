use std::path::Path;
use conan2::ConanInstall;

fn main() {
    ConanInstall::new().build("missing").output_folder(Path::new("build")).run().parse().emit();
}
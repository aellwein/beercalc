use vergen_gitcl::{Emitter, GitclBuilder};

fn main() {
    let gitcl = GitclBuilder::all_git().expect("failed getting git information");
    Emitter::default()
        .add_instructions(&gitcl)
        .expect("failed to add instructions")
        .emit()
        .expect("failed emitting git information");
}

mod util;

fn main() {
    util::run(
        &util::expand_tilde("~/server/some-server"),
        &["cargo", "run", "--", "--addr", "0.0.0.0:6969"],
    );
}

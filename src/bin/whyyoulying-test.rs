// Unlicense — cochranblock.org
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
//! f70=whyyoulying_test. TRIPLE SIMS via exopack::triple_sims::f60. f30=run_tests.

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let ok = exopack::triple_sims::f60(|| async { whyyoulying::tests::f30() == 0 }).await;
    std::process::exit(if ok { 0 } else { 1 });
}

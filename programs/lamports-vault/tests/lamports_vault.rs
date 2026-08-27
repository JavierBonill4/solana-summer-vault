mod common;

use {
    common::{
        build_deposit_ix, build_withdraw_ix, fund, initialize_vault, send, setup_svm,
        ONE_SOL,
    },
    solana_keypair::Keypair,
    solana_signer::Signer,
};


#[test]
fn withdraw_under_the_limit_succeeds() {
    let mut svm = setup_svm();
    let user = Keypair::new();
    fund(&mut svm, &user.pubkey(), 10 * ONE_SOL);

    let max = 2 * ONE_SOL;
    initialize_vault(&mut svm, &user, max);
    send(&mut svm, &user, &[build_deposit_ix(&user.pubkey(), 5 * ONE_SOL)], &[])
        .expect("deposit should succeed");

    send(&mut svm, &user, &[build_withdraw_ix(&user.pubkey(), max - 1)], &[])
        .expect("withdrawing under the limit should succeed");
}

#[test]
fn withdraw_exactly_at_the_limit_succeeds() {
    let mut svm = setup_svm();
    let user = Keypair::new();
    fund(&mut svm, &user.pubkey(), 10 * ONE_SOL);

    let max = 2 * ONE_SOL;
    initialize_vault(&mut svm, &user, max);
    send(&mut svm, &user, &[build_deposit_ix(&user.pubkey(), 5 * ONE_SOL)], &[])
        .expect("deposit should succeed");

    // The boundary is inclusive: `amount == max_withdraw` must be allowed.
    send(&mut svm, &user, &[build_withdraw_ix(&user.pubkey(), max)], &[])
        .expect("withdrawing exactly the limit should succeed");
}

#[test]
fn withdraw_over_the_limit_fails() {
    let mut svm = setup_svm();
    let user = Keypair::new();
    fund(&mut svm, &user.pubkey(), 10 * ONE_SOL);

    let max = 2 * ONE_SOL;
    initialize_vault(&mut svm, &user, max);
    // Deposit far more than we try to withdraw, so a rejection can ONLY be the cap.
    send(&mut svm, &user, &[build_deposit_ix(&user.pubkey(), 5 * ONE_SOL)], &[])
        .expect("deposit should succeed");

    let res = send(&mut svm, &user, &[build_withdraw_ix(&user.pubkey(), max + 1)], &[]);
    assert!(res.is_err(), "one lamport over the limit must be rejected");
}
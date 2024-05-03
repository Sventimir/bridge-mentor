#![allow(unused_imports)]
use crate::data::result::*;
use crate::sexpr::*;

#[cfg(test)]
fn sexp_contract(s: &str) -> ContractResult {
    ContractResult::from_sexp(&sexp::parse(s).unwrap()).unwrap()
}

#[test]
fn test_passed_board_scores_zero() {
    let contract = sexp_contract("(1 pass 0)");
    assert_eq!(contract.score(), 0)
}

#[test]
fn test_major_partscore_by_ns() {
    let contract = sexp_contract("(1 2 S N 0)");
    assert_eq!(contract.score(), 110)
}

#[test]
fn test_minor_partscore_by_we() {
    let contract = sexp_contract("(1 2 C W 0)");
    assert_eq!(contract.score(), -90)
}

#[test]
fn test_notrump_score_with_overtricks() {
    let contract = sexp_contract("(1 2 NT N 2)");
    assert_eq!(contract.score(), 180)
}

#[test]
fn test_vulnerable_partscore() {
    let contract = sexp_contract("(4 2 NT S 2)");
    assert_eq!(contract.score(), 180)
}

#[test]
fn test_invulnerable_game() {
    let contract = sexp_contract("(1 3 NT N 0)");
    assert_eq!(contract.score(), 400)
}

#[test]
fn test_vulerable_game() {
    let contract = sexp_contract("(4 3 NT E 0)");
    assert_eq!(contract.score(), -600)
}

#[test]
fn test_partscore_doubled() {
    let contract = sexp_contract("(1 2 S x N 0)");
    assert_eq!(contract.score(), 470)
}
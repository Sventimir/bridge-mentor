use serde::{Deserialize, Serialize};
use sexp::Sexp;

use bridge::data::board::{Board, BoardNumber};
use bridge::data::result::ContractResult;
use bridge::data::scoring::{Score, Scorable};
use bridge::dealer::deal;
use bridge::sexpr::*;

#[derive(Debug, Serialize, Deserialize)]
pub enum Cmd {
    #[serde(rename = "deal")]
    Deal(BoardNumber),
    #[serde(rename = "score")]
    Score(ContractResult),
}

impl Sexpable for Cmd {
    fn to_sexp(&self) -> Sexp {
        match self {
            Cmd::Deal(board) => {
                sexp::list(&[sexp::atom_s("deal"), sexp::atom_s(&board.to_string())])
            }
            Cmd::Score(result) => sexp::list(&[sexp::atom_s("score"), result.to_sexp()]),
        }
    }

    fn from_sexp(sexp: &Sexp) -> Result<Self, SexpError> {
        let cmd = expect_list(sexp)?;
        let (t, rem) = cmd
            .split_first()
            .ok_or(SexpError::InvalidValue(sexp.clone(), "command".to_string()))?;
        let tag = expect_string(t)?;
        match tag {
            "deal" => {
                let board = expect_int(&rem[0])?;
                Ok(Cmd::Deal(board as BoardNumber))
            }
            "score" => {
                let result = ContractResult::from_sexp(&sexp::list(&rem))?;
                Ok(Cmd::Score(result))
            }
            _ => Err(SexpError::InvalidTag(tag.to_string())),
        }
    }
}

#[derive(Debug, Serialize)]
pub enum CommandError {}

impl Sexpable for CommandError {
    fn to_sexp(&self) -> Sexp {
        NIL
    }

    fn from_sexp(sexp: &Sexp) -> Result<Self, SexpError> {
        Err(SexpError::InvalidValue(
            sexp.clone(),
            "command error".to_string(),
        ))
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum CommandResult {
    Deal(Board),
    Score(Score),
}

impl Sexpable for CommandResult {
    fn to_sexp(&self) -> Sexp {
        match self {
            CommandResult::Deal(board) => board.to_sexp(),
            CommandResult::Score(score) => score.to_sexp(),
        }
    }

    fn from_sexp(sexp: &Sexp) -> Result<Self, SexpError> {
        Ok(CommandResult::Deal(Board::from_sexp(sexp)?))
    }
}

impl Cmd {
    pub fn execute(&self) -> Result<CommandResult, CommandError> {
        match self {
            Cmd::Deal(board) => Ok(CommandResult::Deal(deal(*board))),
            Cmd::Score(result) => Ok(CommandResult::Score(result.score())),
        }
    }
}

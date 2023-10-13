use chap::common::errors::ChapError;


pub type EvalResult = core::result::Result<String, ChapError>;

pub fn clone_result(eval_result: &EvalResult) -> EvalResult{
    match eval_result {
        EvalResult::Ok(arg0) => EvalResult::Ok(arg0.clone()),
        EvalResult::Err(arg0) => EvalResult::Err(arg0.clone()),
    }
}
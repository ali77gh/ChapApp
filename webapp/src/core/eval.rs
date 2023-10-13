use super::eval_result::{EvalResult, clone_result};


static mut TEMP: String = String::new();
static mut EVAL_RESULT: EvalResult = EvalResult::Ok(String::new());

// unsafe code to move reslults out of pointer functions
pub fn eval(source: String) -> EvalResult{ unsafe{ 

    TEMP.clear();
    EVAL_RESULT = EvalResult::Ok(String::new());

    chap::eval::eval(source, |x|{
        TEMP.push_str(x.clone());
        TEMP.push_str("\n");
        EVAL_RESULT = EvalResult::Ok(TEMP.clone());
    }, ||{ 
        return "".to_string(); 
    }, ||{
        // nothing to do
    }, |err|{
        EVAL_RESULT = EvalResult::Err(err);
    });

    clone_result(&EVAL_RESULT)
}}
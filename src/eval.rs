use chap::common::errors::ChapError;

//TODO run this function async (sample: countdown with delays)
pub fn eval(source: String) -> core::result::Result<String, ChapError> {
    let mut error: Option<ChapError> = None;
    let mut std_out = String::new();

    chap::runners::eval::eval(
        source,
        Box::new(|std_out_msg| {
            std_out.push_str(std_out_msg);
            std_out.push('\n');
        }),
        Box::new(|| prompt()),
        |err| error = Some(err),
    );

    if let Some(err) = error {
        Err(err)
    } else {
        Ok(std_out)
    }
}

fn prompt() -> String {
    match web_sys::window() {
        Some(window) => match window.prompt() {
            Ok(Some(input_text)) => input_text,
            _ => "".to_string(),
        },
        None => "".to_string(),
    }
}

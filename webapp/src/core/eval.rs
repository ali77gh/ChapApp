
static mut TEMP: String = String::new();

pub fn eval(source: String) -> String{
    unsafe{TEMP.clear()}
    chap::eval::eval(source, |x|{
        unsafe{
            TEMP.push_str(x.clone());
            TEMP.push_str("\n")
        }
    }, ||{ return "".to_string(); }, ||{}, |e|{});
    unsafe{ TEMP.clone() } 
}
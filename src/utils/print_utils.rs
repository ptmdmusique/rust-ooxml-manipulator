use colored::Colorize;

pub fn get_error_message(message: &str) -> String {
    format!("{} {} {}", "!!! ERROR".red(), message, "!!!".red())
}

pub fn print_error_with_panic(message: &str) -> ! {
    panic!("{}", get_error_message(message));
}

pub fn print_fn_progress(fn_name: &str, message: &str) {
    let fn_name = format!("[{}]", fn_name).blue();
    println!("{} {}", fn_name, message);
}


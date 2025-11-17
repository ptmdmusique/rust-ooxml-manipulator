use colored::Colorize;

pub fn get_error_message(message: &str) -> String {
    let error_icon = "✗".red().bold();
    let error_label = "ERROR".red().bold();
    format!("{} {} {}", error_icon, error_label, message.red())
}

pub fn print_error_with_panic(message: &str) -> ! {
    panic!("{}", get_error_message(message));
}

pub fn print_fn_progress(fn_name: &str, message: &str) {
    let fn_name_formatted = format!("[{}]", fn_name).bright_cyan().bold();
    let arrow = "→".bright_blue();
    let message_formatted = message.bright_white();
    println!("{} {} {}", fn_name_formatted, arrow, message_formatted);
}

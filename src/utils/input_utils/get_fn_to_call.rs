use crate::utils::{
    analyze_custom_xml::analyze_custom_xml_wrapper,
    summarize::summarize_wrapper,
    sync_custom_xml::sync_custom_xml_wrapper,
    types::UserPreference,
    zip_utils::{extract_zip_wrapper, rezip_folder_wrapper},
};
use prompted::input;

pub fn get_fn_to_call() -> Result<(), &'static str> {
    let mut user_preference = UserPreference::new();

    println!("Here is the list of features:");
    println!("\t1. Extract");
    println!("\t2. Rezip");
    println!("\t3. Summarize");
    println!("\t4. Analyze customXML");
    println!("\t5. Sync customXML");

    let mut input_feature = input!(
        "Enter the feature number you want to use (default: {}): ",
        user_preference.last_used_feature
    );

    if input_feature.is_empty() {
        input_feature = user_preference.last_used_feature.clone();
    } else if input_feature != user_preference.last_used_feature {
        user_preference.save_last_used_feature(input_feature.clone());
    }

    match input_feature.as_str() {
        "1" => extract_zip_wrapper(&mut user_preference),
        "2" => rezip_folder_wrapper(&mut user_preference),
        "3" => summarize_wrapper(&mut user_preference),
        "4" => analyze_custom_xml_wrapper(&mut user_preference),
        "5" => sync_custom_xml_wrapper(&mut user_preference),
        _ => return Err("Invalid feature"),
    }

    Ok(())
}

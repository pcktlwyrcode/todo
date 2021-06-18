use serde_json::value::Value;
use serde_json::Map;
use actix_web::HttpRequest;
use crate::to_do;
use crate::state::read_file;
use crate::processes::process_input;
/// 1 Load the current state of the to do item list. ie JSON file
/// 2 Get the title of the new to do item from the URL.
/// 3 Pass the title and the pending string through to_do_factory. remember what expect does?
/// 4 Pass the result of the previous step, along with the create string and the state, into the process module interface.
/// 5 Return a string to the user to signal that the process has finished.


pub async fn create(req: HttpRequest) -> String {
    let state: Map<String, Value> = read_file(String::from(
        "./state.json")); // 1
    let title: String = req.match_info().get("title"
    ).unwrap().to_string();
    let title_reference: String = title.clone(); // 2
    let item = to_do::to_do_factory(&String::from("pending"), 
                title).expect("create "); // 3
    process_input(item, "create".to_string(), &state); // 4
    return format!("{} created", title_reference) // 5
}
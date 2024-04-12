fn main() {
    let username = get_username(1);
    // match username {
    //     Some(name) => println!("{name}"),
    //     None => {}
    // }
    

    if let Some(name) = username {
        println!("{name}");
    }
}

// enum Option<T>{
//     None,
//     Some(T)
// } 

fn get_username(user_id: u32) -> Option<String> {
    //get username from database
    let query = format!("GET username FROM users where id ={user_id}");
    let db_result = query_db(query);
    db_result.ok()
    // if user_id==1{
        
    //     Some(db_result)
    // }
    // else{
    //     None
    // }
}

// enum Result<T,E>{
//     OK(T),
//     Err(E),
// }
fn query_db(query: String) -> Result<String, String>{
    if query.is_empty() {
        Err(String::from("Query string is empty"))
    }else{
        Ok(String::from("Irfan"))
    }
    
}


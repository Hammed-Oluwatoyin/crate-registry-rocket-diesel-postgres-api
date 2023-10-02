extern crate crate_registry;



#[rocket::main] 
async fn main() {
    let _ = rocket::build()
        .mount("/", rocket::routes![
           crate_registry::rocket_routes::crates::get_crates,
            crate_registry::rocket_routes::crates::view_crate,
             crate_registry::rocket_routes::crates::create_crate,
             crate_registry::rocket_routes::crates::update_crate,
             crate_registry::rocket_routes::crates::delete_crate,
             crate_registry::rocket_routes::rustaceans::get_rustaceans,
             crate_registry::rocket_routes::rustaceans::view_rustacean,
             crate_registry::rocket_routes::rustaceans::create_rustacean,
             crate_registry::rocket_routes::rustaceans::update_rustacean,
             crate_registry::rocket_routes::rustaceans::delete_rustacean,
        ])
        
        .attach(crate_registry::rocket_routes::DbConn::fairing())
        .launch()
        .await;
}
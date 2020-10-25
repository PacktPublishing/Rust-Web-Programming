use actix_web::Responder;

use super::utils::return_state;


/// This view gets all of the saved to do items that are stored in the state.json file.
///
/// # Arguments
/// None
///
/// # Returns
/// * (web::Json): all of the stored to do items
pub async fn get() -> impl Responder {
    return return_state()
}

use leptos::*;
use crate::models::conversation::Conversation;




#[server(Converse "/api")]
pub async fn converse(cx:Scope,prompt:Converstion)->Result<String,
ServeFnError>{
    use llm::models::Llama;
    use leptos_actix::extract;
    use activix_wb::web::Data;
    use actix_web::dev::ConnectionInfo;

    let model = extract(cx,|data:Data<Llama>,_connection:ConnectionInfo|async{
        data.into_inner()
      
    }).await.unwrap();
    use llm::KnownModel;
     
}
pub async fn converse(cx:Scope,prompt:Converstion)->Result<String,
ServeFnError>{
    use llm::models::Llama;
    use leptos_actix::extract;
    use activix_wb::web::Data;
    use actix_web::dev::ConnectionInfo;
     
}
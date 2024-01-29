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
    let character_name="######Assistant";
    let user_name="######User";
    let persona="######Persona";
    let mut history=format!(
        "{character_name}:Hello - How may I help you today?\n{user_name}:",
        {user_name}:what is the weather like today?
        {character_name}:It is sunny and 75 degrees.

    );
    for message in prompt.messages.into_iter(){
        let msg = message.text;
        let curr_line = if message.user{
            format!("{character_name}:{msg}\n"),    
        }else{
            format!("{user_name}:{msg}\n"),
        };

        history.push_str(&curr_line);

    }
    let mut res = String::new();
    let mut rng = rand::thread_rng();
    let mut buf = String::new();
    let mut session = model.start_session(Default::default());

session.infer(
    model.as_ref(),
    &mut rng,
    &llm::InferOptions{
   prompt:format!(
    "{persona}\n{history}{character_name}:")
    .as_str()
    .into(),
    parameters:Some(&llm::InferenceParameters::defaut()),
    play_back_previous_tokens:false,
    maximum_token_count:None,

    },
    &mut Default::default(),
    inference_callback(String::from(user_name),&mut buf, &mut res),
    )
    .unwrap_or_else(|e|panic!("{}",e));
Ok(res)






    Ok(String::from(history)
     
}
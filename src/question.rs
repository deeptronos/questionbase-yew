use crate::*;

#[derive(Properties, Clone, PartialEq, serde::Deserialize)]
pub struct QuestionStruct {
    pub id: String,
    pub title: String,
    pub body: String,
    pub asker: String,
}

impl QuestionStruct {
    pub async fn get_question(key: Option<String>) -> Msg {
        let request = match &key {
            None => "http://localhost:3000/api/question".to_string(),
            Some(ref key) => format!("http://localhost:3000/api/question/{}", key,),
        };
        log!(format!("CTEST: request: {:?}", request));
        let response = http::Request::get(&request).send().await;
        log!(format!("CTEST: response: {:?}", response));
        match response {
            Err(e) => Msg::GotQuestion(Err(e)),
            Ok(data) => Msg::GotQuestion(data.json().await),
        }
    }
}
pub fn format_tags(tags: &HashSet<String>) -> String {
    let taglist: Vec<&str> = tags.iter().map(String::as_ref).collect();
    taglist.join(", ")
}

#[derive(Properties, Clone, PartialEq, serde::Deserialize)]
pub struct QuestionProps {
    pub question: QuestionStruct,
}

#[function_component(Question)]
pub fn question(question: &QuestionProps) -> Html {
    let question = &question.question;
    html! { <> // TODO edit HTML/template/css
        <div class="joke">
            <span class="teller">{"Knock-Knock!"}</span><br/>
            <span class="tellee">{"Who's there?"}</span><br/>
            <span class="teller">{question.title.clone()}</span><br/>
            <span class="tellee">{format!("{} who?", &question.body)}</span><br/>
        </div>
        <span class="annotation">
            {format!("[id: {}", &question.id)}
            // if let Some(ref tags) = question.tags {
            //     {format!("; tags: {}", &format_tags(tags))}
            // }
            {format!("; source: {}", &question.asker)}
            {"]"}
        </span>
    </> }
}

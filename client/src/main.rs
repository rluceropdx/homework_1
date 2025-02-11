// use std::collections::HashMap;
use reqwest::{Client, Error, Response};
use serde_json::json;
use shared::question::{Question}; //, Method, Response};

/// example code
///
/// Create a reqwest client
/// let client = Client::new();
/// Make a GET HTTP request to our backend's /example route
/// let res = client.get("http://localhost:8088/example").send().await?;
///
/// Get the response from backend's data
/// let body = res.text().await?;
/// Print out that response
/// println!("GET Response: {}", body);
///
/// Same as GET, but makes a POST request with appropriate header
/// let res = client
///     .post("http://localhost:8088/example")
///     .header("Content-Type", "application/json")
///     .body("Example Body")
///     .send().await?;
///
/// let body = res.text().await?;
/// println!("POST Response: {}", body);
///
/// You'll use these methods along with DELETE to accomplish your task
#[tokio::main]
async fn main() {
    // Your code here!

    let mut get_questions = test_api("http://localhost:8088/questions");
    let mut questions_arr = get_questions.await.unwrap().json::<Vec<Question>>().await.unwrap();
    let length = questions_arr.len() as usize;
    println!("\nTest 1: DISPLAYING ALL QUESTIONS FROM API");
    output_questions(questions_arr);

    let mut last_question_id: usize = length - 1;
    let mut url_path: String = "http://localhost:8088/question?question_id=".to_owned();
    let id_string: String = last_question_id.to_string();
    url_path.push_str(&id_string);
    let get_last_question = test_api(&url_path);
    let question = get_last_question.await.unwrap().json::<Question>().await.unwrap();
    let vc= vec![question];
    println!("\nTest 2: DISPLAYING LAST QUESTION FROM API");
    output_questions(vc);

    let question_content: String = "new question content2".to_string();
    let question_title = "new question title2".to_string();
    let question_tags: Option<Vec<String>> = Option::from(vec!["custom".to_string()]);

    let new_question = json!({
        "title": question_title,
        "content": question_content,
        "tags": question_tags
    });


    let client = Client::new();
    let mut _res = client.post("http://localhost:8088/question")
        .header("Content-Type", "application/json")
        .json(&new_question)
        .send()
        .await;

    println!("\nTest 3: NEW QUESTION CREATED/SAVED");
    get_questions = test_api("http://localhost:8088/questions");
    questions_arr = get_questions.await.unwrap().json::<Vec<Question>>().await.unwrap();
    println!("\nTest 4: DISPLAYING ALL QUESTIONS AGAIN FROM API");
    last_question_id = (questions_arr.len() as usize) - 1;

    output_questions(questions_arr);

    let mut del_url_path: String = "http://localhost:8088/question?question_id=".to_owned();
    let id_string2: String = last_question_id.to_string();
    del_url_path.push_str(&id_string2);


   _res = client.delete(del_url_path).send().await;

    println!("\nTest 5: DELETED LAST QUESTION");
    get_questions = test_api("http://localhost:8088/questions");
    questions_arr = get_questions.await.unwrap().json::<Vec<Question>>().await.unwrap();
    println!("\nTest 6: DISPLAYING ALL QUESTIONS AGAIN FROM API");
    output_questions(questions_arr);

}

async fn test_api(url: &str) -> Result<Response, Error> {
    // Create a reqwest client
    let client = Client::new();
    let result = client.get(url).send().await;
    return result;
}

fn output_questions(q_arr: Vec<Question>) {
    let mut index = 0;
    while index < q_arr.len() {
        println!("{:?}", q_arr.get(index).unwrap().title);
        index += 1;
    }
}
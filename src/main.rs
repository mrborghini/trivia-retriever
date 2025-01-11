use std::{fs, time::Duration};

use rand::{seq::SliceRandom, thread_rng};
use reqwest::Client;
use serde::{Deserialize, Serialize};

const URL: &str = "https://opentdb.com/api.php?amount=50";
const OUTPUT_FILE: &str = "trivia.json";

#[derive(Debug, Deserialize)]
struct OpentDBResponse {
    response_code: i32,
    results: Vec<OpentDBTrivia>,
}

#[derive(Debug, Deserialize)]
struct OpentDBTrivia {
    #[serde(rename = "type")]
    type_string: String,
    difficulty: String,
    question: String,
    correct_answer: String,
    incorrect_answers: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Trivia {
    question: String,
    answer: String,
    difficulty: String,
}

fn format_question(mut trivia: OpentDBTrivia) -> String {
    if trivia.type_string == "boolean" {
        trivia.question.push_str(format!(" True or False").as_str());
        return trivia.question;
    }

    // Add the correct answer for the string
    trivia.incorrect_answers.push(trivia.correct_answer);
    trivia.incorrect_answers.shuffle(&mut thread_rng());

    let last_index = trivia.incorrect_answers.len() - 1;

    for i in 0..trivia.incorrect_answers.len() {
        if i == last_index {
            trivia
                .question
                .push_str(format!(" or {}", trivia.incorrect_answers[i]).as_str());
            continue;
        }
        trivia
            .question
            .push_str(format!(" {},", trivia.incorrect_answers[i]).as_str());
    }
    return trivia.question;
}

fn add_data(response: OpentDBResponse) {
    let current_data = fs::read_to_string(OUTPUT_FILE).unwrap_or_else(|_| "[]".to_string());
    let mut current_trivia: Vec<Trivia> =
        serde_json::from_str(&current_data).unwrap_or_else(|_| Vec::new());

    let mut added = 0;
    for trivia in response.results {
        let difficulty = trivia.difficulty.clone();
        let answer = trivia.correct_answer.clone();
        let original_question = trivia.question.clone();
        let formatted_question = format_question(trivia);

        let mut exists = false;
        for existing in &current_trivia {
            if existing.question.contains(&original_question) {
                exists = true;
                break;
            }
        }
        if exists {
            continue;
        }

        current_trivia.push(Trivia {
            question: formatted_question,
            difficulty,
            answer,
        });
        added += 1;
    }

    let output_data = serde_json::to_string(&current_trivia);

    match output_data {
        Ok(json_string) => {
            fs::write(OUTPUT_FILE, json_string).unwrap();
            println!("Added {} new entries. Total: {}", added, &current_trivia.len());
        }
        Err(why) => {
            eprintln!("Could not serialize data {}", why);
        }
    }
}

#[tokio::main]
async fn main() {
    let client = Client::new();

    loop {
        let opendb_response = client
            .get(URL)
            .send()
            .await
            .unwrap()
            .json::<OpentDBResponse>()
            .await
            .unwrap();

        // Only add data when it's a success code
        if opendb_response.response_code == 0 {
            add_data(opendb_response);
        }

        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}

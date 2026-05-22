use crate::config::Settings;
use crate::models::RetractionData;
use crate::utils::truncate;
use reqwest::blocking::Client;
use serde_json::Value;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

pub fn get_retraction_data(
    client: &Client,
    settings: &Settings,
    category_id: &str,
    start_date: &str,
    end_date: &str,
) -> Result<RetractionData, Box<dyn Error>> {
    let filter = format!(
        "from_publication_date:{},to_publication_date:{},type:article,concepts.id:{}",
        start_date, end_date, category_id
    );

    let params = vec![
        ("filter".to_string(), filter),
        ("group_by".to_string(), "is_retracted".to_string()),
    ];

    let data = call_openalex(client, settings, params)?;

    let mut retracted: u64 = 0;
    let mut not_retracted: u64 = 0;

    if let Some(group_by) = data.get("group_by").and_then(|v| v.as_array()) {
        for item in group_by {
            let key_display = item
                .get("key_display_name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_lowercase();

            let count = item.get("count").and_then(|v| v.as_u64()).unwrap_or(0);

            if key_display == "true" {
                retracted = count;
            } else if key_display == "false" {
                not_retracted = count;
            }
        }
    }

    let total_articles = retracted + not_retracted;

    Ok(RetractionData {
        total_articles,
        not_retracted,
        retracted,
    })
}

pub fn get_textual_proxy_count(
    client: &Client,
    settings: &Settings,
    category_id: &str,
    start_date: &str,
    end_date: &str,
    keyword: &str,
) -> Result<u64, Box<dyn Error>> {
    let filter = format!(
        "from_publication_date:{},to_publication_date:{},type:article,concepts.id:{},title_and_abstract.search:{}",
        start_date, end_date, category_id, keyword
    );

    let params = vec![("filter".to_string(), filter)];
    let data = call_openalex(client, settings, params)?;

    let count = data
        .get("meta")
        .and_then(|meta| meta.get("count"))
        .and_then(|count| count.as_u64())
        .unwrap_or(0);

    Ok(count)
}

fn call_openalex(
    client: &Client,
    settings: &Settings,
    mut query_params: Vec<(String, String)>,
) -> Result<Value, Box<dyn Error>> {
    let max_retries = 3;

    if let Some(api_key) = &settings.api_key {
        query_params.push(("api_key".to_string(), api_key.clone()));
    }

    if let Some(mailto) = &settings.mailto {
        query_params.push(("mailto".to_string(), mailto.clone()));
    }

    for attempt in 1..=max_retries {
        let response = client.get(&settings.base_url).query(&query_params).send();

        match response {
            Ok(resp) => {
                let status = resp.status();

                if status.is_success() {
                    let json: Value = resp.json()?;
                    return Ok(json);
                } else {
                    let body = resp.text().unwrap_or_else(|_| String::from(""));
                    println!(
                        "Errore HTTP {}. Tentativo {}/{}",
                        status, attempt, max_retries
                    );
                    println!("Risposta: {}", truncate(&body, 300));
                    sleep(Duration::from_secs(2));
                }
            }
            Err(err) => {
                println!(
                    "Errore richiesta: {}. Tentativo {}/{}",
                    err, attempt, max_retries
                );
                sleep(Duration::from_secs(2));
            }
        }
    }

    Err("Chiamata OpenAlex fallita dopo vari tentativi.".into())
}

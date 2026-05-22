use std::env;

#[derive(Clone)]
pub struct Settings {
    pub base_url: String,
    pub api_key: Option<String>,
    pub mailto: Option<String>,
    pub timeout_seconds: u64,
    pub sleep_seconds: u64,
}

#[derive(Clone)]
pub struct Period {
    pub name: &'static str,
    pub start_date: &'static str,
    pub end_date: &'static str,
}

#[derive(Clone)]
pub struct Category {
    pub name: &'static str,
    pub concept_id: &'static str,
}

impl Settings {
    pub fn from_env() -> Self {
        Self {
            base_url: env::var("OPENALEX_BASE_URL")
                .unwrap_or_else(|_| "https://api.openalex.org/works".to_string()),
            api_key: env::var("OPENALEX_API_KEY")
                .ok()
                .filter(|v| !v.trim().is_empty()),
            mailto: env::var("OPENALEX_MAILTO")
                .ok()
                .filter(|v| !v.trim().is_empty()),
            timeout_seconds: env::var("OPENALEX_TIMEOUT_SECONDS")
                .ok()
                .and_then(|v| v.parse::<u64>().ok())
                .unwrap_or(60),
            sleep_seconds: env::var("OPENALEX_SLEEP_SECONDS")
                .ok()
                .and_then(|v| v.parse::<u64>().ok())
                .unwrap_or(1),
        }
    }
}

pub fn periods() -> Vec<Period> {
    vec![
        Period {
            name: "Pre-Covid 2016-2017",
            start_date: "2016-01-01",
            end_date: "2017-12-31",
        },
        Period {
            name: "Covid 2020-2021",
            start_date: "2020-01-01",
            end_date: "2021-12-31",
        },
        Period {
            name: "Post-Covid 2024-2025",
            start_date: "2024-01-01",
            end_date: "2025-12-31",
        },
    ]
}

pub fn categories() -> Vec<Category> {
    vec![
        Category {
            name: "Medicine & Health Sciences",
            concept_id: "C71924100",
        },
        Category {
            name: "Life Sciences",
            concept_id: "C86803240",
        },
        Category {
            name: "Physical Sciences",
            concept_id: "C121332964|C185592680|C192562407",
        },
        Category {
            name: "Engineering",
            concept_id: "C127413603",
        },
        Category {
            name: "Computer Science",
            concept_id: "C41008148",
        },
        Category {
            name: "Social Sciences",
            concept_id: "C144024400|C15744967|C17744445",
        },
        Category {
            name: "Economics & Management",
            concept_id: "C162324750|C144133560",
        },
        Category {
            name: "Humanities",
            concept_id: "C138885662|C95457728|C142362112",
        },
    ]
}

mod config;
mod models;
mod openalex;
mod output;
mod utils;

use crate::config::{categories, periods, Settings};
use crate::models::Row;
use crate::openalex::{get_retraction_data, get_textual_proxy_count};
use crate::output::csv_export::write_csv;
use crate::output::html_report::write_html_report;
use crate::utils::{format_int, format_pct, percentage};
use reqwest::blocking::Client;
use std::error::Error;
use std::fs;
use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();

    let settings = Settings::from_env();
    let periods = periods();
    let categories = categories();

    fs::create_dir_all("output")?;

    let client = Client::builder()
        .timeout(Duration::from_secs(settings.timeout_seconds))
        .build()?;

    let mut rows: Vec<Row> = Vec::new();

    println!("====================================================");
    println!("Avvio estrazione dati OpenAlex");
    println!("Categorie disciplinari: {}", categories.len());
    println!("Periodizzazioni: {}", periods.len());
    println!("Indicatori: ritrattazioni, correction, erratum, correction+erratum");
    println!(
        "API key configurata: {}",
        if settings.api_key.is_some() {
            "sì"
        } else {
            "no"
        }
    );
    println!(
        "Mailto configurato: {}",
        if settings.mailto.is_some() {
            "sì"
        } else {
            "no"
        }
    );
    println!("====================================================\n");

    for category in &categories {
        println!("####################################################");
        println!("CATEGORIA: {}", category.name);
        println!("####################################################");

        let mut category_rows: Vec<Row> = Vec::new();

        for period in &periods {
            println!("\n----------------------------------------------------");
            println!("Elaboro: {} | {}", category.name, period.name);
            println!("Periodo: {} -> {}", period.start_date, period.end_date);
            println!("----------------------------------------------------");

            println!("1/3 Recupero ritrattazioni tramite is_retracted...");
            let retraction_data = get_retraction_data(
                &client,
                &settings,
                category.concept_id,
                period.start_date,
                period.end_date,
            )?;

            println!(
                "   Totale articoli: {} | Ritrattati: {} | Non ritrattati: {}",
                format_int(retraction_data.total_articles),
                format_int(retraction_data.retracted),
                format_int(retraction_data.not_retracted)
            );

            println!("2/3 Recupero proxy correzioni keyword 'correction'...");
            let correction_count = get_textual_proxy_count(
                &client,
                &settings,
                category.concept_id,
                period.start_date,
                period.end_date,
                "correction",
            )?;

            println!("   Correction count: {}", format_int(correction_count));

            println!("3/3 Recupero proxy correzioni keyword 'erratum'...");
            let erratum_count = get_textual_proxy_count(
                &client,
                &settings,
                category.concept_id,
                period.start_date,
                period.end_date,
                "erratum",
            )?;

            println!("   Erratum count: {}", format_int(erratum_count));

            let total_articles = retraction_data.total_articles;
            let correction_erratum_proxy = correction_count + erratum_count;

            let perc_retracted = percentage(retraction_data.retracted, total_articles);
            let perc_correction = percentage(correction_count, total_articles);
            let perc_erratum = percentage(erratum_count, total_articles);
            let perc_correction_erratum_proxy =
                percentage(correction_erratum_proxy, total_articles);

            println!("\nPercentuali periodo:");
            println!("   % ritrattati: {}", format_pct(perc_retracted));
            println!("   % correction: {}", format_pct(perc_correction));
            println!("   % erratum: {}", format_pct(perc_erratum));
            println!(
                "   % correzioni proxy correction + erratum: {}",
                format_pct(perc_correction_erratum_proxy)
            );

            let row = Row {
                category: category.name.to_string(),
                period: period.name.to_string(),
                start_date: period.start_date.to_string(),
                end_date: period.end_date.to_string(),
                total_articles,
                not_retracted: retraction_data.not_retracted,
                retracted: retraction_data.retracted,
                perc_retracted,
                correction_count,
                perc_correction,
                erratum_count,
                perc_erratum,
                correction_erratum_proxy,
                perc_correction_erratum_proxy,
            };

            category_rows.push(row.clone());
            rows.push(row);

            sleep(Duration::from_secs(settings.sleep_seconds));
        }

        println!("\n====================================================");
        println!("RIEPILOGO PERCENTUALI CATEGORIA: {}", category.name);
        println!("====================================================");
        print_category_summary(&category_rows);
        println!("====================================================\n");
    }

    let csv_filename = "output/openalex_ritrattazioni_correzioni_2016_2017_2020_2021_2024_2025.csv";
    let html_filename =
        "output/openalex_ritrattazioni_correzioni_report_2016_2017_2020_2021_2024_2025.html";

    write_csv(csv_filename, &rows)?;
    write_html_report(html_filename, &rows)?;

    println!("\n====================================================");
    println!("ESTRAZIONE COMPLETATA");
    println!("CSV creato: {}", csv_filename);
    println!("Dashboard HTML creata: {}", html_filename);
    println!("Tempo totale esecuzione: {:.2?}", start_time.elapsed());
    println!("====================================================");

    Ok(())
}

fn print_category_summary(rows: &[Row]) {
    println!(
        "{:<25} | {:>14} | {:>14} | {:>14} | {:>22}",
        "Periodo", "% ritrattati", "% correction", "% erratum", "% correction+erratum"
    );

    println!("{}", "-".repeat(100));

    for row in rows {
        println!(
            "{:<25} | {:>14} | {:>14} | {:>14} | {:>22}",
            row.period,
            format_pct(row.perc_retracted),
            format_pct(row.perc_correction),
            format_pct(row.perc_erratum),
            format_pct(row.perc_correction_erratum_proxy)
        );
    }
}

use crate::config::{categories, periods};
use crate::models::{metric_value, Metric, Row};
use crate::utils::{format_int, format_pct, html_escape, percentage};
use std::error::Error;
use std::fs::File;
use std::io::Write;

pub fn write_html_report(filename: &str, rows: &[Row]) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(filename)?;

    let total_articles: u64 = rows.iter().map(|r| r.total_articles).sum();
    let total_retracted: u64 = rows.iter().map(|r| r.retracted).sum();
    let total_correction: u64 = rows.iter().map(|r| r.correction_count).sum();
    let total_erratum: u64 = rows.iter().map(|r| r.erratum_count).sum();
    let total_correction_erratum = total_correction + total_erratum;

    let overall_retracted_pct = percentage(total_retracted, total_articles);
    let overall_correction_pct = percentage(total_correction, total_articles);
    let overall_erratum_pct = percentage(total_erratum, total_articles);
    let overall_correction_erratum_pct = percentage(total_correction_erratum, total_articles);

    let mut html = String::new();

    html.push_str(
        r#"
<!DOCTYPE html>
<html lang="it">
<head>
    <meta charset="UTF-8">
    <title>Dashboard OpenAlex</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            background: #f5f7fb;
            margin: 0;
            padding: 28px;
            color: #111827;
        }

        .dashboard {
            max-width: 1400px;
            margin: auto;
        }

        h1 {
            margin: 0 0 6px 0;
            font-size: 30px;
        }

        .subtitle {
            color: #4b5563;
            margin-bottom: 24px;
            font-size: 15px;
        }

        .cards {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(190px, 1fr));
            gap: 16px;
            margin-bottom: 24px;
        }

        .card {
            background: white;
            border-radius: 16px;
            padding: 18px;
            box-shadow: 0 6px 18px rgba(0,0,0,0.07);
            border: 1px solid #e5e7eb;
        }

        .card-title {
            font-size: 13px;
            color: #6b7280;
            margin-bottom: 8px;
        }

        .card-value {
            font-size: 24px;
            font-weight: 700;
        }

        .section {
            background: white;
            border-radius: 16px;
            padding: 20px;
            margin-top: 20px;
            box-shadow: 0 6px 18px rgba(0,0,0,0.06);
            border: 1px solid #e5e7eb;
        }

        .method-box {
            background: #eff6ff;
            border-left: 5px solid #2563eb;
            padding: 14px 16px;
            border-radius: 10px;
            margin-bottom: 18px;
            font-size: 14px;
            line-height: 1.5;
        }

        .warning-box {
            background: #fff7ed;
            border-left: 5px solid #f97316;
            padding: 14px 16px;
            border-radius: 10px;
            margin-bottom: 18px;
            font-size: 14px;
            line-height: 1.5;
        }

        .scroll {
            overflow-x: auto;
        }

        table {
            border-collapse: collapse;
            width: 100%;
            font-size: 13px;
            margin-top: 12px;
        }

        th {
            background: #111827;
            color: white;
            padding: 10px;
            text-align: right;
            white-space: nowrap;
        }

        th:first-child {
            text-align: left;
        }

        td {
            border: 1px solid #e5e7eb;
            padding: 9px;
            text-align: right;
            white-space: nowrap;
        }

        td:first-child {
            text-align: left;
            font-weight: 600;
        }

        tr:nth-child(even) {
            background: #f9fafb;
        }

        .small-note {
            color: #6b7280;
            font-size: 12px;
            margin-top: 12px;
        }
    </style>
</head>
<body>
<div class="dashboard">
"#,
    );

    html.push_str("<h1>Dashboard OpenAlex</h1>");
    html.push_str(
        r#"<div class="subtitle">Ritrattazioni e correzioni per categoria disciplinare e periodo: 2016-2017, 2020-2021, 2024-2025.</div>"#,
    );

    html.push_str(r#"<div class="cards">"#);
    html.push_str(&card_html(
        "Totale articoli osservati",
        &format_int(total_articles),
    ));
    html.push_str(&card_html(
        "Totale ritrattazioni",
        &format_int(total_retracted),
    ));
    html.push_str(&card_html(
        "Totale correction",
        &format_int(total_correction),
    ));
    html.push_str(&card_html("Totale erratum", &format_int(total_erratum)));
    html.push_str(&card_html(
        "Totale correction + erratum",
        &format_int(total_correction_erratum),
    ));
    html.push_str(&card_html(
        "% ritrattati complessiva",
        &format_pct(overall_retracted_pct),
    ));
    html.push_str(&card_html(
        "% correction complessiva",
        &format_pct(overall_correction_pct),
    ));
    html.push_str(&card_html(
        "% erratum complessiva",
        &format_pct(overall_erratum_pct),
    ));
    html.push_str(&card_html(
        "% correction + erratum complessiva",
        &format_pct(overall_correction_erratum_pct),
    ));
    html.push_str("</div>");

    html.push_str(
        r#"
<div class="method-box">
    <strong>Metodo:</strong>
    le ritrattazioni sono misurate tramite il campo strutturato
    <code>is_retracted</code> di OpenAlex.
    Le correzioni sono approssimate tramite ricerca testuale nei titoli e negli abstract
    usando le keyword <code>correction</code> ed <code>erratum</code>.
</div>

<div class="warning-box">
    <strong>Attenzione metodologica:</strong>
    il dato sulle ritrattazioni è più robusto. Il dato sulle correzioni è una proxy esplorativa:
    la keyword <code>correction</code> può intercettare anche articoli che parlano di correzioni
    statistiche, tecniche o metodologiche, non necessariamente correzioni editoriali.
    Inoltre, <code>correction + erratum</code> è una somma non deduplicata.
</div>
"#,
    );

    html.push_str(
        "<div class=\"section\"><h2>Percentuale di articoli ritrattati</h2><div class=\"scroll\">",
    );
    html.push_str(&pivot_table_html(rows, Metric::Retracted));
    html.push_str("</div></div>");

    html.push_str(
        "<div class=\"section\"><h2>Percentuale keyword “correction”</h2><div class=\"scroll\">",
    );
    html.push_str(&pivot_table_html(rows, Metric::Correction));
    html.push_str("</div></div>");

    html.push_str(
        "<div class=\"section\"><h2>Percentuale keyword “erratum”</h2><div class=\"scroll\">",
    );
    html.push_str(&pivot_table_html(rows, Metric::Erratum));
    html.push_str("</div></div>");

    html.push_str("<div class=\"section\"><h2>Percentuale proxy correzioni “correction + erratum”</h2><div class=\"scroll\">");
    html.push_str(&pivot_table_html(rows, Metric::CorrectionErratum));
    html.push_str("</div><div class=\"small-note\">Nota: questa somma non è deduplicata. Serve solo come indicatore esplorativo.</div></div>");

    html.push_str("<div class=\"section\"><h2>Tabella completa</h2><div class=\"scroll\">");
    html.push_str(&full_table_html(rows));
    html.push_str("</div></div>");

    html.push_str(
        r#"
<div class="small-note">
    Fonte: OpenAlex API.
</div>
</div>
</body>
</html>
"#,
    );

    file.write_all(html.as_bytes())?;
    Ok(())
}

fn pivot_table_html(rows: &[Row], metric: Metric) -> String {
    let categories = categories();
    let periods = periods();

    let mut html = String::new();

    html.push_str("<table><thead><tr>");
    html.push_str("<th>Categoria disciplinare</th>");

    for period in &periods {
        html.push_str(&format!("<th>{}</th>", html_escape(period.name)));
    }

    html.push_str("</tr></thead><tbody>");

    for category in &categories {
        html.push_str("<tr>");
        html.push_str(&format!("<td>{}</td>", html_escape(category.name)));

        for period in &periods {
            let value = rows
                .iter()
                .find(|r| r.category == category.name && r.period == period.name)
                .map(|r| metric_value(r, metric))
                .unwrap_or(0.0);

            html.push_str(&format!("<td>{}</td>", format_pct(value)));
        }

        html.push_str("</tr>");
    }

    html.push_str("</tbody></table>");
    html
}

fn full_table_html(rows: &[Row]) -> String {
    let mut html = String::new();

    html.push_str("<table><thead><tr>");
    html.push_str("<th>Categoria disciplinare</th>");
    html.push_str("<th>Periodo</th>");
    html.push_str("<th>Data inizio</th>");
    html.push_str("<th>Data fine</th>");
    html.push_str("<th>Totale articoli</th>");
    html.push_str("<th>Non ritrattati</th>");
    html.push_str("<th>Ritrattati</th>");
    html.push_str("<th>% ritrattati</th>");
    html.push_str("<th>Correction count</th>");
    html.push_str("<th>% correction</th>");
    html.push_str("<th>Erratum count</th>");
    html.push_str("<th>% erratum</th>");
    html.push_str("<th>Correction + Erratum proxy</th>");
    html.push_str("<th>% correction + erratum proxy</th>");
    html.push_str("</tr></thead><tbody>");

    for row in rows {
        html.push_str("<tr>");
        html.push_str(&format!("<td>{}</td>", html_escape(&row.category)));
        html.push_str(&format!("<td>{}</td>", html_escape(&row.period)));
        html.push_str(&format!("<td>{}</td>", html_escape(&row.start_date)));
        html.push_str(&format!("<td>{}</td>", html_escape(&row.end_date)));
        html.push_str(&format!("<td>{}</td>", format_int(row.total_articles)));
        html.push_str(&format!("<td>{}</td>", format_int(row.not_retracted)));
        html.push_str(&format!("<td>{}</td>", format_int(row.retracted)));
        html.push_str(&format!("<td>{}</td>", format_pct(row.perc_retracted)));
        html.push_str(&format!("<td>{}</td>", format_int(row.correction_count)));
        html.push_str(&format!("<td>{}</td>", format_pct(row.perc_correction)));
        html.push_str(&format!("<td>{}</td>", format_int(row.erratum_count)));
        html.push_str(&format!("<td>{}</td>", format_pct(row.perc_erratum)));
        html.push_str(&format!(
            "<td>{}</td>",
            format_int(row.correction_erratum_proxy)
        ));
        html.push_str(&format!(
            "<td>{}</td>",
            format_pct(row.perc_correction_erratum_proxy)
        ));
        html.push_str("</tr>");
    }

    html.push_str("</tbody></table>");
    html
}

fn card_html(title: &str, value: &str) -> String {
    format!(
        r#"
<div class="card">
    <div class="card-title">{}</div>
    <div class="card-value">{}</div>
</div>
"#,
        html_escape(title),
        html_escape(value)
    )
}

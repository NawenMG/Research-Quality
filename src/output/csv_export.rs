use crate::models::Row;
use csv::Writer;
use std::error::Error;

pub fn write_csv(filename: &str, rows: &[Row]) -> Result<(), Box<dyn Error>> {
    let mut writer = Writer::from_path(filename)?;

    writer.write_record([
        "Categoria disciplinare",
        "Periodo",
        "Data inizio",
        "Data fine",
        "Totale articoli",
        "Non ritrattati",
        "Ritrattati",
        "% ritrattati",
        "Correction count",
        "% correction",
        "Erratum count",
        "% erratum",
        "Correction + Erratum proxy",
        "% correction + erratum proxy",
    ])?;

    for row in rows {
        writer.write_record([
            row.category.to_string(),
            row.period.to_string(),
            row.start_date.to_string(),
            row.end_date.to_string(),
            row.total_articles.to_string(),
            row.not_retracted.to_string(),
            row.retracted.to_string(),
            format!("{:.4}", row.perc_retracted),
            row.correction_count.to_string(),
            format!("{:.4}", row.perc_correction),
            row.erratum_count.to_string(),
            format!("{:.4}", row.perc_erratum),
            row.correction_erratum_proxy.to_string(),
            format!("{:.4}", row.perc_correction_erratum_proxy),
        ])?;
    }

    writer.flush()?;
    Ok(())
}

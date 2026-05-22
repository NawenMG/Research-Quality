#[derive(Clone)]
pub struct RetractionData {
    pub total_articles: u64,
    pub not_retracted: u64,
    pub retracted: u64,
}

#[derive(Clone)]
pub struct Row {
    pub category: String,
    pub period: String,
    pub start_date: String,
    pub end_date: String,
    pub total_articles: u64,
    pub not_retracted: u64,
    pub retracted: u64,
    pub perc_retracted: f64,
    pub correction_count: u64,
    pub perc_correction: f64,
    pub erratum_count: u64,
    pub perc_erratum: f64,
    pub correction_erratum_proxy: u64,
    pub perc_correction_erratum_proxy: f64,
}

#[derive(Clone, Copy)]
pub enum Metric {
    Retracted,
    Correction,
    Erratum,
    CorrectionErratum,
}

pub fn metric_value(row: &Row, metric: Metric) -> f64 {
    match metric {
        Metric::Retracted => row.perc_retracted,
        Metric::Correction => row.perc_correction,
        Metric::Erratum => row.perc_erratum,
        Metric::CorrectionErratum => row.perc_correction_erratum_proxy,
    }
}

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct DataStore {
    #[serde(flatten)]
    dnos: HashMap<String, DnoEntry>,
}

#[derive(Serialize, Deserialize)]
struct DnoEntry {
    metadata: Metadata,
    crawl: CrawlConfig,
    data: HashMap<String, YearData>,
}

#[derive(Serialize, Deserialize)]
struct Metadata {
    dno_name: Vec<String>,
    description: String,
    region: String,
}

#[derive(Serialize, Deserialize)]
struct CrawlConfig {
    #[serde(rename = "type")]
    crawl_type: String,
    source: HashMap<String, String>,
    file_pattern: HashMap<String, String>,
    settings: CrawlSettings,
}

#[derive(Serialize, Deserialize)]
struct CrawlSettings {
    auto_crawl: bool,
    auto_crawl_interval: bool,
    auto_crawl_years: Vec<u32>,
}

#[derive(Serialize, Deserialize)]
struct YearData {
    latest_update: String,
    hlzf: HlzfData,
    netzentgelte: NetzentgelteData,
    source: SourceData,
}


#[derive(Serialize, Deserialize)]
struct HlzfData {
    #[serde(rename = "Winter_1_Start")]
    winter_1_start: Option<String>,
    #[serde(rename = "Winter_1_Ende")]
    winter_1_ende: Option<String>,
    #[serde(rename = "Winter_2_Start")]
    winter_2_start: Option<String>,
    #[serde(rename = "Winter_2_Ende")]
    winter_2_ende: Option<String>,
    #[serde(rename = "Winter_3_Start")]
    winter_3_start: Option<String>,
    #[serde(rename = "Winter_3_Ende")]
    winter_3_ende: Option<String>,
    #[serde(rename = "Winter_4_Start")]
    winter_4_start: Option<String>,
    #[serde(rename = "Winter_4_Ende")]
    winter_4_ende: Option<String>,
    #[serde(rename = "Fruehling_1_Start")]
    fruehling_1_start: Option<String>,
    #[serde(rename = "Fruehling_1_Ende")]
    fruehling_1_ende: Option<String>,
    #[serde(rename = "Fruehling_2_Start")]
    fruehling_2_start: Option<String>,
    #[serde(rename = "Fruehling_2_Ende")]
    fruehling_2_ende: Option<String>,
    #[serde(rename = "Fruehling_3_Start")]
    fruehling_3_start: Option<String>,
    #[serde(rename = "Fruehling_3_Ende")]
    fruehling_3_ende: Option<String>,
    #[serde(rename = "Fruehling_4_Start")]
    fruehling_4_start: Option<String>,
    #[serde(rename = "Fruehling_4_Ende")]
    fruehling_4_ende: Option<String>,
    #[serde(rename = "Sommer_1_Start")]
    sommer_1_start: Option<String>,
    #[serde(rename = "Sommer_1_Ende")]
    sommer_1_ende: Option<String>,
    #[serde(rename = "Sommer_2_Start")]
    sommer_2_start: Option<String>,
    #[serde(rename = "Sommer_2_Ende")]
    sommer_2_ende: Option<String>,
    #[serde(rename = "Sommer_3_Start")]
    sommer_3_start: Option<String>,
    #[serde(rename = "Sommer_3_Ende")]
    sommer_3_ende: Option<String>,
    #[serde(rename = "Sommer_4_Start")]
    sommer_4_start: Option<String>,
    #[serde(rename = "Sommer_4_Ende")]
    sommer_4_ende: Option<String>,
    #[serde(rename = "Herbst_1_Start")]
    herbst_1_start: Option<String>,
    #[serde(rename = "Herbst_1_Ende")]
    herbst_1_ende: Option<String>,
    #[serde(rename = "Herbst_2_Start")]
    herbst_2_start: Option<String>,
    #[serde(rename = "Herbst_2_Ende")]
    herbst_2_ende: Option<String>,
    #[serde(rename = "Herbst_3_Start")]
    herbst_3_start: Option<String>,
    #[serde(rename = "Herbst_3_Ende")]
    herbst_3_ende: Option<String>,
    #[serde(rename = "Herbst_4_Start")]
    herbst_4_start: Option<String>,
    #[serde(rename = "Herbst_4_Ende")]
    herbst_4_ende: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct NetzentgelteData {
    hs: NetzentgeltTarif,
    #[serde(rename = "hs/ms")]
    hs_ms: NetzentgeltTarif,
    ms: NetzentgeltTarif,
    #[serde(rename = "ms/ns")]
    ms_ns: NetzentgeltTarif,
    ns: NetzentgeltTarif,
}

#[derive(Serialize, Deserialize)]
struct NetzentgeltTarif {
    #[serde(rename = "Leistung")]
    leistung: f64,
    #[serde(rename = "Arbeit")]
    arbeit: f64,
    #[serde(rename = "Leistung_unter_2500h")]
    leistung_unter_2500h: f64,
    #[serde(rename = "Arbeit_unter_2500h")]
    arbeit_unter_2500h: f64,
}

#[derive(Serialize, Deserialize)]
struct SourceData {
    #[serde(rename = "type")]
    source_type: String,
    url: SourceUrl,
    file: SourceFile,
}

#[derive(Serialize, Deserialize)]
struct SourceUrl {
    hlzf: Option<String>,
    netzentgelte: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct SourceFile {
    hlzf: String,
    netzentgelte: String,
}

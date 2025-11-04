use ollama_rust::{model::ModelOptions, ollama::Ollama};

pub mod common;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let top_25_results = vec![
        "LSZH: Airport {\n    icao: \"LSZH\",\n    iata: \"ZRH\",\n    name: \"Zurich Airport\",\n    city: \"Zurich\",\n    state: \"Zurich\",\n    country: \"CH\",\n    elevation: 1416,\n    lat: 47.4646987915,\n    lon: 8.5491695404,\n    tz: \"Europe/Zurich\",\n}",
        "LSZG: Airport {\n    icao: \"LSZG\",\n    iata: \"ZHI\",\n    name: \"Grenchen Airport\",\n    city: \"\",\n    state: \"Solothurn\",\n    country: \"CH\",\n    elevation: 1411,\n    lat: 47.1815986633,\n    lon: 7.4171900749,\n    tz: \"Europe/Zurich\",\n}",
        "LSZN: Airport {\n    icao: \"LSZN\",\n    iata: \"\",\n    name: \"Hausen am Albis Airport\",\n    city: \"\",\n    state: \"Zurich\",\n    country: \"CH\",\n    elevation: 1928,\n    lat: 47.2386016846,\n    lon: 8.5155601501,\n    tz: \"Europe/Zurich\",\n}",
        "LSZK: Airport {\n    icao: \"LSZK\",\n    iata: \"\",\n    name: \"Speck-Fehraltorf Airport\",\n    city: \"\",\n    state: \"Zurich\",\n    country: \"CH\",\n    elevation: 1748,\n    lat: 47.3763999939,\n    lon: 8.7574996948,\n    tz: \"Europe/Zurich\",\n}",
        "LSPH: Airport {\n    icao: \"LSPH\",\n    iata: \"\",\n    name: \"Winterthur Airport\",\n    city: \"Winterthur\",\n    state: \"Zurich\",\n    country: \"CH\",\n    elevation: 1506,\n    lat: 47.5149993896,\n    lon: 8.7719402313,\n    tz: \"Europe/Zurich\",\n}",
        "LSZO: Airport {\n    icao: \"LSZO\",\n    iata: \"\",\n    name: \"Luzern-Beromunster Airport\",\n    city: \"Lucerne\",\n    state: \"Lucerne\",\n    country: \"CH\",\n    elevation: 2146,\n    lat: 47.1899986267,\n    lon: 8.2047195435,\n    tz: \"Europe/Zurich\",\n}",
        "LSPK: Airport {\n    icao: \"LSPK\",\n    iata: \"\",\n    name: \"Hasenstrick Airport\",\n    city: \"\",\n    state: \"Zurich\",\n    country: \"CH\",\n    elevation: 0,\n    lat: 47.2799987793,\n    lon: 8.881939888,\n    tz: \"Europe/Zurich\",\n}",
        "LSZA: Airport {\n    icao: \"LSZA\",\n    iata: \"LUG\",\n    name: \"Lugano Airport\",\n    city: \"Lugano\",\n    state: \"Ticino\",\n    country: \"CH\",\n    elevation: 915,\n    lat: 46.0042991638,\n    lon: 8.9105796814,\n    tz: \"Europe/Zurich\",\n}",
        "LSMI: Airport {\n    icao: \"LSMI\",\n    iata: \"ZIN\",\n    name: \"Interlaken Air Base\",\n    city: \"\",\n    state: \"Bern\",\n    country: \"CH\",\n    elevation: 0,\n    lat: 46.6766014099,\n    lon: 7.8790798187,\n    tz: \"Europe/Zurich\",\n}",
        "LSZW: Airport {\n    icao: \"LSZW\",\n    iata: \"\",\n    name: \"Thun Airport\",\n    city: \"Thun\",\n    state: \"Bern\",\n    country: \"CH\",\n    elevation: 1837,\n    lat: 46.756401062,\n    lon: 7.6005601883,\n    tz: \"Europe/Zurich\",\n}",
        "LSGK: Airport {\n    icao: \"LSGK\",\n    iata: \"\",\n    name: \"Saanen Airport\",\n    city: \"Saanen\",\n    state: \"Bern\",\n    country: \"CH\",\n    elevation: 3307,\n    lat: 46.4874992371,\n    lon: 7.2508301735,\n    tz: \"Europe/Zurich\",\n}",
        "LSZR: Airport {\n    icao: \"LSZR\",\n    iata: \"ACH\",\n    name: \"St Gallen Altenrhein Airport\",\n    city: \"Altenrhein\",\n    state: \"Saint-Gallen\",\n    country: \"CH\",\n    elevation: 1306,\n    lat: 47.4850006104,\n    lon: 9.5607700348,\n    tz: \"Europe/Vienna\",\n}",
        "LSTZ: Airport {\n    icao: \"LSTZ\",\n    iata: \"\",\n    name: \"Zweisimmen Airport\",\n    city: \"\",\n    state: \"Bern\",\n    country: \"CH\",\n    elevation: 3068,\n    lat: 46.5525016785,\n    lon: 7.3805599213,\n    tz: \"Europe/Zurich\",\n}",
        "LSZV: Airport {\n    icao: \"LSZV\",\n    iata: \"\",\n    name: \"Sitterdorf Airport\",\n    city: \"\",\n    state: \"Thurgau\",\n    country: \"CH\",\n    elevation: 1660,\n    lat: 47.5088996887,\n    lon: 9.2627801895,\n    tz: \"Europe/Zurich\",\n}",
        "LSZI: Airport {\n    icao: \"LSZI\",\n    iata: \"\",\n    name: \"Fricktal-Schupfart Airport\",\n    city: \"\",\n    state: \"Aargau\",\n    country: \"CH\",\n    elevation: 1788,\n    lat: 47.5088996887,\n    lon: 7.9499998093,\n    tz: \"Europe/Berlin\",\n}",
        "LSZL: Airport {\n    icao: \"LSZL\",\n    iata: \"ZJI\",\n    name: \"Locarno Airport\",\n    city: \"\",\n    state: \"Ticino\",\n    country: \"CH\",\n    elevation: 650,\n    lat: 46.1608009338,\n    lon: 8.8786096573,\n    tz: \"Europe/Zurich\",\n}",
        "EDSI: Airport {\n    icao: \"EDSI\",\n    iata: \"\",\n    name: \"Binningen Airport\",\n    city: \"Binningen\",\n    state: \"Baden-Wuerttemberg\",\n    country: \"DE\",\n    elevation: 1594,\n    lat: 47.7991676331,\n    lon: 8.720000267,\n    tz: \"Europe/Berlin\",\n}",
        "LSMD: Airport {\n    icao: \"LSMD\",\n    iata: \"\",\n    name: \"Dubendorf Airport\",\n    city: \"Zurich\",\n    state: \"Zurich\",\n    country: \"CH\",\n    elevation: 1470,\n    lat: 47.398601532,\n    lon: 8.648229599,\n    tz: \"Europe/Zurich\",\n}",
        "LSZJ: Airport {\n    icao: \"LSZJ\",\n    iata: \"\",\n    name: \"Courtelary Airport\",\n    city: \"\",\n    state: \"Bern\",\n    country: \"CH\",\n    elevation: 2247,\n    lat: 47.1836013794,\n    lon: 7.0908298492,\n    tz: \"Europe/Zurich\",\n}",
        "LSZP: Airport {\n    icao: \"LSZP\",\n    iata: \"\",\n    name: \"Biel-Kappelen Airport\",\n    city: \"\",\n    state: \"Bern\",\n    country: \"CH\",\n    elevation: 1437,\n    lat: 47.0891990662,\n    lon: 7.2899999619,\n    tz: \"Europe/Zurich\",\n}",
        "LSZX: Airport {\n    icao: \"LSZX\",\n    iata: \"\",\n    name: \"Schanis Airport\",\n    city: \"\",\n    state: \"Saint-Gallen\",\n    country: \"CH\",\n    elevation: 1365,\n    lat: 47.1716995239,\n    lon: 9.039440155,\n    tz: \"Europe/Zurich\",\n}",
        "LSZB: Airport {\n    icao: \"LSZB\",\n    iata: \"BRN\",\n    name: \"Bern Belp Airport\",\n    city: \"Bern\",\n    state: \"Bern\",\n    country: \"CH\",\n    elevation: 1674,\n    lat: 46.914100647,\n    lon: 7.4971499443,\n    tz: \"Europe/Zurich\",\n}",
        "LSZU: Airport {\n    icao: \"LSZU\",\n    iata: \"\",\n    name: \"Buttwil Airport\",\n    city: \"\",\n    state: \"Aargau\",\n    country: \"CH\",\n    elevation: 2372,\n    lat: 47.2647018433,\n    lon: 8.3024997711,\n    tz: \"Europe/Zurich\",\n}",
        "LSZE: Airport {\n    icao: \"LSZE\",\n    iata: \"\",\n    name: \"Bad Ragaz Airport\",\n    city: \"\",\n    state: \"Saint-Gallen\",\n    country: \"CH\",\n    elevation: 1617,\n    lat: 47.0149993896,\n    lon: 9.4819402695,\n    tz: \"Europe/Zurich\",\n}",
        "LSZC: Airport {\n    icao: \"LSZC\",\n    iata: \"BXO\",\n    name: \"Buochs Airport\",\n    city: \"Buochs\",\n    state: \"Nidwalden\",\n    country: \"CH\",\n    elevation: 1475,\n    lat: 46.9744444444,\n    lon: 8.3969444444,\n    tz: \"Europe/Zurich\",\n}",
    ];

    let question = "What are the coordinates of the Zurich Airport and Lugano Airport?";

    println!("\nQueston: {question}\n");

    let ollama = Ollama::default();
    for doc in top_25_results {
        let response = ollama
            .rerank(
                common::QWEN3_RERANK_8B_4096D,
                question,
                doc,
                None,
                ModelOptions::default()
                    .temperature(0.)
                    .top_k(1)
                    .top_p(1.)
                    .repeat_penalty(1.)
                    .stop(vec!["\n".to_string()]),
            )
            .await
            .expect("Rerank failed");
        println!(
            "Document: {} | Score: {}",
            doc.split_once('\n').unwrap().0.split_once(':').unwrap().0,
            response.response.trim().trim_matches('{').trim()
        );
    }

    println!("\n");

    Ok(())
}

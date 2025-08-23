// Dieses Attribut importiert die Makros aus dem Rocket-Crate,
// damit wir sie direkt verwenden können (z.B. `#[get("/")]`).
#[macro_use] extern crate rocket;

// Importiert die notwendigen Module aus den Crates.
// `RawHtml` wird für die Rückgabe von rohem HTML-Code benötigt.
use rocket::response::content::RawHtml;
// `System`, `SystemExt` und `CpuExt` aus dem `sysinfo`-Crate, um Systeminformationen auszulesen.
use sysinfo::{System, SystemExt, CpuExt};

use std::env;

// Definiert eine Route für den HTTP-GET-Request auf "/".
// Diese Funktion wird aufgerufen, wenn jemand die Hauptseite besucht.
#[get("/")]
fn index() -> &'static str {
    "Hello, world! Besuche /info für Systeminformationen."
}

// Definiert eine Route für den HTTP-GET-Request auf "/info".
// Diese Funktion sammelt Systeminformationen und gibt sie als HTML zurück.
#[get("/info")]
fn info() -> RawHtml<String> {
    // Eine neue `System`-Instanz erstellen, um Informationen abzurufen.
    let mut sys = System::new_all();

    // Alle Systeminformationen aktualisieren.
    sys.refresh_all();

    // Die gesammelten Informationen in einer HTML-Tabelle formatieren.
    // `format!` ist ein Makro, das einen String mit ersetzten Werten erstellt.
    let html_content = format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Systeminformationen</title>
            <style>
                body {{ font-family: sans-serif; }}
                table {{ border-collapse: collapse; width: 50%; margin: 20px 0; }}
                th, td {{ border: 1px solid #dddddd; text-align: left; padding: 8px; }}
                th {{ background-color: #f2f2f2; }}
            </style>
        </head>
        <body>
            <h1>Systeminformationen</h1>
            <table>
                <tr>
                    <th>Kategorie</th>
                    <th>Information</th>
                </tr>
                <tr>
                    <td>Betriebssystem</td>
                    <td>{os_name} {os_version}</td>
                </tr>
                <tr>
                    <td>Kernel-Version</td>
                    <td>{kernel_version}</td>
                </tr>
                <tr>
                    <td>CPU-Modell</td>
                    <td>{cpu_model}</td>
                </tr>
                <tr>
                    <td>Anzahl der Kerne</td>
                    <td>{core_count}</td>
                </tr>
                <tr>
                    <td>Arbeitsspeicher</td>
                    <td>{memory:.2} GB</td>
                </tr>
                <tr>
                    <td>Hostname</td>
                    <td>{hostname}</td>
                </tr>
            </table>
        </body>
        </html>
        "#,
        // Betriebssystemname und -version abrufen.
        // `unwrap_or_else` bietet einen Standardwert, falls die Information nicht verfügbar ist.
        os_name = sys.name().unwrap_or_else(|| "N/A".to_string()),
        os_version = sys.os_version().unwrap_or_else(|| "N/A".to_string()),
        // Kernel-Version abrufen.
        kernel_version = sys.kernel_version().unwrap_or_else(|| "N/A".to_string()),
        // CPU-Informationen abrufen. Wir nehmen den ersten CPU aus der Liste.
        // `map_or` ist eine sichere Methode, um auf optionale Werte zuzugreifen.
        cpu_model = sys.cpus().get(0).map_or("N/A", |cpu| cpu.brand()),
        // Anzahl der physischen CPU-Kerne.
        core_count = sys.physical_core_count().map_or("N/A".to_string(), |c| c.to_string()),
        // Gesamten Arbeitsspeicher in Gigabytes umrechnen.
        memory = sys.total_memory() as f64 / (1024 * 1024 * 1024) as f64,
        // Hostname abrufen.
        hostname = sys.host_name().unwrap_or_else(|| "N/A".to_string())
    );

    // Den HTML-String als `RawHtml` zurückgeben, damit Rocket ihn als HTML interpretiert.
    RawHtml(html_content)
}
#[get("/env")]
fn env_vars_as_html_table() -> String {
    // Einen leeren, veränderbaren String erstellen, um das HTML zu sammeln.
    let mut html_output = String::new();

    // Mit dem öffnenden table-Tag beginnen.
    html_output.push_str("<table>");
    html_output.push_str("<tr><th>Variable</th><th>Wert</th></tr>"); // Tabellenkopf

    // Durch alle Umgebungsvariablen iterieren.
    for (key, value) in env::vars() {
        // Für jede Variable eine neue Tabellenzeile (<tr>) mit zwei Zellen (<td>)
        // zum HTML-String hinzufügen.
        let row = format!("<tr><td>{key}</td><td>{value}</td></tr>", key = key, value = value);
        html_output.push_str(&row);
    }

    // Den schließenden table-Tag hinzufügen.
    html_output.push_str("</table>");

    // Den fertigen HTML-String zurückgeben.
    html_output
}

// Das `#[launch]`-Attribut markiert die `rocket()`-Funktion als den Startpunkt
// der Rocket-Anwendung. Rocket wird diese Funktion aufrufen, um den Server zu starten.
#[launch]
fn rocket() -> _ {
    // `rocket::build()` erstellt eine neue Rocket-Instanz.
    // `.mount()` registriert die Routen unter einem Basispfad.
    // `routes!` ist ein Makro, das eine Liste von Routenfunktionen sammelt.
    rocket::build().mount("/", routes![index, info, env_vars_as_html_table])
}

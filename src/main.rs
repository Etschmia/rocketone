#[macro_use]
extern crate rocket;

use rocket_dyn_templates::Template;
use serde::Serialize;
use std::env;
use sysinfo::{CpuExt, System, SystemExt};

#[derive(Serialize)]
struct IndexContext {
    title: String,
}

#[get("/")]
fn index() -> Template {
    let context = IndexContext {
        title: "Willkommen bei RocketOne".to_string(),
    };
    Template::render("index", &context)
}

#[derive(Serialize)]
struct InfoContext {
    title: String,
    os_name: String,
    os_version: String,
    kernel_version: String,
    cpu_model: String,
    core_count: String,
    memory: String,
    hostname: String,
}

#[get("/info")]
fn info() -> Template {
    let mut sys = System::new_all();
    sys.refresh_all();

    let context = InfoContext {
        title: "Systeminformationen".to_string(),
        os_name: sys.name().unwrap_or_else(|| "N/A".to_string()),
        os_version: sys.os_version().unwrap_or_else(|| "N/A".to_string()),
        kernel_version: sys.kernel_version().unwrap_or_else(|| "N/A".to_string()),
        cpu_model: sys.cpus().get(0).map_or("N/A".to_string(), |c| c.brand().to_string()),
        core_count: sys.physical_core_count().map_or("N/A".to_string(), |c| c.to_string()),
        memory: format!("{:.2}", sys.total_memory() as f64 / (1024 * 1024 * 1024) as f64),
        hostname: sys.host_name().unwrap_or_else(|| "N/A".to_string()),
    };

    Template::render("info", &context)
}

#[derive(Serialize)]
struct EnvVar {
    key: String,
    value: String,
}

#[derive(Serialize)]
struct EnvContext {
    title: String,
    vars: Vec<EnvVar>,
}

#[get("/env")]
fn env_page() -> Template {
    let vars = env::vars()
        .map(|(key, value)| EnvVar { key, value })
        .collect();

    let context = EnvContext {
        title: "Umgebungsvariablen".to_string(),
        vars,
    };

    Template::render("env", &context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, info, env_page])
        .attach(Template::fairing())
}
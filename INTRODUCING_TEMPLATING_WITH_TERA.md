# Umstellung auf Templates mit Tera in Rocket

Dieses Dokument beschreibt den Prozess und die Gründe für die Umstellung der HTML-Generierung von manuellen `format!`-Makros auf ein Template-System mit `Tera`.

## Das Problem (Warum diese Änderung?)

In der ursprünglichen Version wurde für jede Route (`/info`, `/env`) das komplette HTML-Dokument als String direkt in der jeweiligen Rust-Funktion erzeugt.

Dies führte zu mehreren Problemen:
1.  **Code-Duplizierung:** Das HTML-Grundgerüst (inkl. `<head>`, `<style>`, `<body>` etc.) musste für jede neue Route kopiert werden.
2.  **Schlechte Wartbarkeit:** Eine Änderung am Design (z.B. am CSS) hätte in jeder einzelnen Route nachgezogen werden müssen.
3.  **Vermischung von Logik und Darstellung:** Rust-Code (Logik zur Datenbeschaffung) war eng mit HTML-Code (Darstellung) vermischt. Das macht den Code unübersichtlich und schwer zu lesen.
4.  **Fehlende Skalierbarkeit:** Mit wachsender Anwendungsgröße wäre dieser Ansatz schnell unüberschaubar und fehleranfällig geworden.

## Die Lösung: Template-Engine

Um diese Probleme zu lösen, wurde ein Template-Engine eingeführt. Dies ist der Standardansatz in der modernen Webentwicklung.

- **Technologie:** `rocket_dyn_templates` (offizielles Crate von Rocket) in Kombination mit `Tera` (eine beliebte und mächtige Template-Engine für Rust).

Das Prinzip ist einfach:
- **Basis-Template:** Ein einziges HTML-Grundgerüst (`base.html.tera`) wird erstellt. Es enthält Platzhalter für variable Teile wie den Seitentitel und den Hauptinhalt.
- **Inhalts-Templates:** Für jede Seite (`info`, `env`) gibt es eine eigene kleine HTML-Datei, die nur den spezifischen Inhalt (z.B. eine Tabelle) definiert. Diese Datei "erbt" vom Basis-Template.
- **Rust-Code:** Die Rust-Funktionen sind nur noch für die Bereitstellung der Daten zuständig. Sie sammeln die Informationen, packen sie in eine Struktur (`struct`) und übergeben diese an das Template-System mit der Anweisung: "Rendere Template X mit diesen Daten".

## Der Umsetzungsplan

Die Umstellung erfolgte in den folgenden Schritten:

### Schritt 1: Abhängigkeiten in `Cargo.toml` hinzufügen
Die notwendigen Crates für das Templating und die Daten-Serialisierung wurden hinzugefügt:
```toml
[dependencies]
rocket = "0.5.0"
sysinfo = "0.29.0"
# Hinzugefügt:
rocket_dyn_templates = { version = "0.1.0", features = ["tera"] }
serde = { version = "1.0", features = ["derive"] }
```

### Schritt 2: Template-Verzeichnis und Basis-Layout erstellen
1.  Ein neues Verzeichnis `templates/` wurde im Projekt-Stamm angelegt.
2.  Darin wurde die Datei `templates/base.html.tera` mit dem HTML-Grundgerüst und den Tera-Platzhaltern `{{ title }}` und `{% block content %}` erstellt.

### Schritt 3: Spezifische Templates für `info` und `env` erstellen
- Die Dateien `templates/info.html.tera` und `templates/env.html.tera` wurden erstellt. Sie erweitern das `base.html.tera` und definieren den `content`-Block, um die jeweiligen Tabellen darzustellen. Die Tabellen werden dynamisch mit Tera-Schleifen (`{% for ... %}`) aus den von Rust übergebenen Daten generiert.

### Schritt 4: Rust-Code (`main.rs`) anpassen
Der Code in `src/main.rs` wurde grundlegend überarbeitet:
1.  **Nötige Imports** für `Template`, `context` und `Serialize` wurden hinzugefügt.
2.  **Context-Structs** (`InfoContext`, `EnvContext`) wurden definiert, um die Daten typsicher an die Templates zu übergeben.
3.  Die **Routen-Funktionen** (`info`, `env_vars_as_html_table`) wurden so umgebaut, dass sie keine HTML-Strings mehr erzeugen, sondern die Context-Structs befüllen und ein `Template`-Objekt zurückgeben.
4.  Die **Rocket-Instanz** wurde beim Start mit `.attach(Template::fairing())` versehen, um das Template-System zu initialisieren.

Dieses Vorgehen führt zu sauberem, wartbarem und skalierbarem Code, der den Best Practices der Webentwicklung mit Rust und Rocket entspricht.

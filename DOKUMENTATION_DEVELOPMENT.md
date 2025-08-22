# Projekt: rocketone - Entwicklungsdokumentation

Dieses Dokument beschreibt die Schritte, die während der Entwicklung der `rocketone`-Anwendung durchgeführt wurden.

## Initialisierung

Das Projekt wurde mit `cargo new rocketone` initialisiert. Die grundlegende Projektstruktur mit `src/main.rs` und `Cargo.toml` wurde damit erstellt.

## Implementierung des Web-Servers

1.  **Abhängigkeiten hinzufügen:** Die `rocket`- und `sysinfo`-Crates wurden zur `Cargo.toml`-Datei hinzugefügt, um die Webserver-Funktionalität und das Auslesen von Systeminformationen zu ermöglichen.

2.  **Grundlegende Routen erstellen:** In `src/main.rs` wurde eine minimale Rocket-Anwendung mit zwei Routen erstellt:
    *   `/`: Eine Index-Route, die eine einfache Begrüßungsnachricht zurückgibt.
    *   `/info`: Die Route, die für die Anzeige der Systeminformationen vorgesehen ist.

3.  **Systeminformationen sammeln und anzeigen:**
    *   Die `sysinfo`-Bibliothek wurde verwendet, um auf Betriebssystem- und Hardware-Informationen zuzugreifen.
    *   Die `info`-Routenfunktion wurde implementiert, um diese Daten zu sammeln (Betriebssystem, Kernel, CPU, Speicher, Hostname).
    *   Die gesammelten Daten werden dynamisch in eine HTML-Tabelle formatiert.
    *   Rocket's `RawHtml` wurde verwendet, um den generierten HTML-Code als Antwort an den Client zu senden.

4.  **Code-Dokumentation:** Der Rust-Code in `src/main.rs` wurde ausführlich kommentiert, um die Funktionsweise der Anwendung, die verwendeten Crates und die einzelnen Code-Abschnitte zu erklären.
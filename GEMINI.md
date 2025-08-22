# Projekt: rocketone

## 🎯 Projektziel

Das Ziel dieses Projekts ist die Erstellung einer Webanwendung mit dem **Rust-Framework Rocket**. Die Anwendung soll eine einzelne Webseite bereitstellen, die detaillierte Systeminformationen über den Mac anzeigt, auf dem sie läuft. Die Funktionalität soll an die `phpinfo()`-Funktion von PHP erinnern und so viele Systemdetails wie möglich sammeln und übersichtlich darstellen.

Dieses Projekt dient primär dem Erlernen von Rust und dem Rocket-Framework.

## 🛠️ Technologiestack

* **Sprache:** Rust
* **Web-Framework:** Rocket
* **Entwicklungsumgebung:** Visual Studio Code (VS Code)
* **Zielplattform:** macOS

## 📋 Anforderungen

1.  **Rocket-Setup:** Einrichten eines grundlegenden Rocket-Servers.
2.  **Informationssammlung:**
    * Betriebssystem (Name, Version, Kernel-Version)
    * Hardware (CPU-Modell, Anzahl der Kerne, Arbeitsspeicher)
    * Netzwerkinformationen (Hostname, IP-Adressen)
    * Laufende Prozesse oder Umgebungsvariablen.
3.  **Darstellung:** Die gesammelten Informationen sollen in einer einfachen, gut lesbaren HTML-Tabelle auf einer einzigen Webseite unter der Route `/info` angezeigt werden.
4.  **Lernfokus:** Der gesamte Code soll ausführlich dokumentiert und kommentiert sein. Jeder Schritt, jede verwendete Bibliothek (`Crate`) und jedes wichtige Konzept soll erklärt werden, um den Lernprozess zu unterstützen.

## 🧑‍💻 Entwicklerprofil

Der Entwickler hat langjährige Erfahrung mit **Ansi C** und **PHP** sowie gelegentliche Erfahrung mit **Python**. Rust und das Rocket-Framework sind neu. Der Fokus liegt darauf, die Konzepte von Rust im Kontext eines praktischen Web-Projekts zu verstehen.
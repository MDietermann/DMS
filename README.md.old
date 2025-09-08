# DMS - Datenbank-Management-System

![DMS Logo](https://github.com/MDietermann/DMS/blob/master/src/assets/dms-logo.jpg)

**DMS** (*Datenbank-Management-System*) ist ein vielseitiges Tool zur Verwaltung von Datenbanken, zum Exportieren von Tabellendaten in **JSON** oder **CSV** sowie zum nahtlosen Importieren von **JSON**- oder **CSV**-Dateien in vorhandene Tabellen. Entwickelt für **Datenbankunabhängigkeit**, unterstützt DMS mehrere Datenbanksysteme, darunter:

- **MySQL**
- **SQL Server**
- **MariaDB**
- **PostgreSQL**
- **SQLite**

---

## Inhaltsverzeichnis

- [DMS - Datenbank-Management-System](#dms---datenbank-management-system)
  - [Inhaltsverzeichnis](#inhaltsverzeichnis)
  - [Übersicht](#übersicht)
  - [Funktionen](#funktionen)
    - [Kernanforderungen](#kernanforderungen)
    - [Optimaler Funktionsumfang](#optimaler-funktionsumfang)
  - [Verwendete Technologien](#verwendete-technologien)
    - [Rust](#rust)
    - [Tauri](#tauri)
    - [Vue.js](#vuejs)
    - [HTML, CSS und TypeScript](#html-css-und-typescript)
    - [Bootstrap \& TailwindCSS](#bootstrap--tailwindcss)
    - [Voraussetzungen](#voraussetzungen)
    - [Installationsschritte](#installationsschritte)
  - [Warum DMS?](#warum-dms)
  - [Erste Schritte](#erste-schritte)

---

## Übersicht

**DMS** bietet eine zentrale Anlaufstelle für die Datenbankverwaltung und ermöglicht es Anwendern, wichtige Aufgaben einfach auszuführen:

- **Daten exportieren**: Speichern von Tabellendaten als **JSON** oder **CSV** an einem gewünschten Speicherort.
- **Daten importieren**: Präziser Import von **JSON**- oder **CSV**-Dateien in vorhandene Tabellen.
- **Cross-Datenbank-Kompatibilität**: Mehrere Datenbanksysteme mühe- und nahtlos verwalten.
- **Benutzer- und Berechtigungsverwaltung** *(Best-Case-Szenario)*: Benutzerdefinierter Zugriff auf Datenbankverbindungen.

Ob Entwickler, Datenbankadministrator oder Daten-Enthusiast – **DMS** vereinfacht Datenbankoperationen durch ein modernes, leistungsstarkes Design.

---

## Funktionen

### Kernanforderungen
- **Datenbankverbindungen** intern über SQLite speichern und verwalten.
- Tabellendaten aus einer verbundenen MySQL-Datenbank extrahieren und als **JSON** oder **CSV** exportieren.
- Daten aus **JSON**- oder **CSV**-Dateien in ausgewählte Tabellen importieren.
- Dateien an einem angegebenen Ort auf dem Gerät speichern und laden.

### Optimaler Funktionsumfang
Im optimalen Zustand bietet **DMS**:
- Unterstützung mehrerer Datenbanksysteme: **MySQL**, **MariaDB**, **PostgreSQL**, **SQL Server** und **SQLite**.
- Benutzerdefinierte Speicherung von Datenbankverbindungen inklusive rollenbasierter Berechtigungen.
- Direkte Datenbankverwaltung über die DMS-Oberfläche (Tabellen erstellen, bearbeiten und löschen).
- Eine intuitive und benutzerfreundliche Oberfläche für erweiterte Import-/Export-Workflows.

---

## Verwendete Technologien

### Rust
[Rust](https://www.rust-lang.org) ist eine leistungsstarke, systemnahe Programmiersprache, die sich auf **Sicherheit**, **Geschwindigkeit** und **Nebenläufigkeit** konzentriert. Dank seiner speichersicheren Eigenschaften und hervorragenden Performance eignet sich Rust ideal für die Backend-Entwicklung.

### Tauri
[Tauri](https://tauri.app) ist ein modernes **plattforübergreifendes** Framework für die Erstellung schlanker Desktop-Anwendungen. Mit **Rust** für das Backend und **Webtechnologien** (HTML, CSS, JavaScript/TypeScript) für das Frontend entstehen ressourceneffiziente Anwendungen für Windows, macOS und Linux.

### Vue.js
[Vue.js](https://vuejs.org) ist ein progressives **JavaScript-Framework** zur Erstellung dynamischer Benutzeroberflächen. Dank seiner komponentenbasierten Architektur wird Modularität und Wartbarkeit gewährleistet, während die **reaktive** Datenbindung UI-Updates vereinfacht.

**Hinweis**: Die **Composition API** wurde für saubereren und modulareren Code im Frontend verwendet.

### HTML, CSS und TypeScript
- **HTML**: Strukturierung des Inhalts der Benutzeroberfläche.
- **CSS**: Verbessert das visuelle Design und sorgt für ein modernes, sauberes Erscheinungsbild.
- **TypeScript**: Fügt JavaScript statische Typisierung hinzu, was die Codeklarheit, Wartbarkeit und Fehlerminimierung verbessert.

### Bootstrap & TailwindCSS
- **Bootstrap**: Ein responsives, gridbasiertes CSS-Framework zur schnellen Erstellung benutzerfreundlicher UIs.
- **TailwindCSS**: Ein **Utility-First-CSS-Framework**, das hochgradig anpassbare und effiziente Styles direkt im Markup ermöglicht.

---

### Voraussetzungen
1. **Rust** (für das Tauri-Backend)
   - Rust über [rustup](https://rustup.rs/) installieren:
     ```sh
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```

2. **Node.js** (LTS-Version) und **npm** (Node Package Manager)
   - Von der [Node.js-Website](https://nodejs.org/) herunterladen und installieren:
     ```sh
     # Installation überprüfen:
     node -v
     npm -v
     ```

3. **Tauri CLI**
   - Tauri-Tools global installieren:
     ```sh
     cargo install tauri-cli
     ```

### Installationsschritte
1. **Repository klonen**:
   ```sh
    git clone https://github.com/MDietermann/DMS.git
    cd DMS
   ```
2. **Abhängigkeiten installieren**: Projektabhängigkeiten mit _npm_ installieren:
   ```sh
    npm install
   ```
3. **Dev-Modus starten**: Entwicklungsserver starten, um die Anwendung zu testen:
   ```sh
    npm run tauri dev
   ```
4. **Anwendung erstellen**: Anwendung für die Produktion kompilieren:
   ```sh
    npm run tauri build
   ```

---

## Warum DMS?

- **Datenbankunabhängigkeit**: Mehrere Datenbanktypen verbinden und verwalten ohne Plattformbeschränkungen.
- **Flexible Datenverwaltung**: **JSON**- oder **CSV**-Daten einfach exportieren und importieren.
- **Leistungsstark**: Entwickelt mit **Rust** und **Tauri** für Geschwindigkeit, Sicherheit und geringe Ressourcenbelastung.
- **Benutzerzentriertes Design**: Eine moderne, saubere Oberfläche, die auf Benutzerfreundlichkeit und Flexibilität ausgelegt ist.

---

## Erste Schritte

Um **DMS** zu erkunden, klone das Repository und folge den Anweisungen unter [Installationsschritte](#installationsschritte).

**Mitwirken**: Beiträge sind willkommen! Weitere Details findest du in der Datei `CONTRIBUTING.md`.

---

**DMS** – Vereinfache deine Datenbankverwaltung. 🚀

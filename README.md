
# DMS - Database Management System

- [DMS - Database Management System](#dms---database-management-system)
  - [Generelles](#generelles)
  - [Programmiersprachen und Frameworks](#programmiersprachen-und-frameworks)
    - [Rust](#rust)
    - [Tauri](#tauri)
    - [HTML (HyperText Markup Language)](#html-hypertext-markup-language)
    - [CSS (Cascading Style Sheets)](#css-cascading-style-sheets)
    - [JavaScript / TypeScript](#javascript--typescript)
    - [Vue.js](#vuejs)
    - [Bootstrap](#bootstrap)
    - [TailwindCSS](#tailwindcss)
  - [Funktionen](#funktionen)
    - [Mindestanforderungen](#mindestanforderungen)
    - [Best Case](#best-case)

## Generelles
Das **DMS** - kurz für **D**atabase **M**anagement **S**ystem - ist ein zentrales HUB, um Datenbanken zu verwalten, Daten aus Tabellen in *JSON* oder *CSV* zu exportieren oder Tabellen mit *JSON* oder *CSV* Dateien zu befüllen.

Ein Vorteil des **DMS** ist die Unabhängigkeit des Datenbanksystems - egal ob *MySQL*, *SQL-Server*, *MariaDB*, *PostgreSQL* oder *SQLite*.

## Programmiersprachen und Frameworks
Entwickelt wurde das **DMS** als *Cross-Plattform Webapplikation* mit **[Tauri](https://tauri.app)**.
Das Backend ist mit **[Rust](https://www.rust-lang.org)** entwickelt, das Frontend mit **HTML**, **CSS** und **TypeScript** über das **[Vue.js](https://vuejs.org)** Framework entwickelt. Hierbei wurde für das Styling eine Kombination aus **[Bootstrap](https://getbootstrap.com)** und **[TailwindCSS](https://tailwindcss.com)** verwendet.

### Rust
**Rust** ist eine systemspezifische Programmiersprache, die sich auf *Sicherheit*, *Performance* und *Parallelität* konzentriert. Sie bietet *Speicher- und Thread-Sicherheit* ohne Garbage Collection, was sie besonders für *leistungsintensive* Anwendungen geeignet macht. Rust wird oft in Bereichen wie *WebAssembly*, *Embedded* *Systems* und Softwareentwicklung auf *niedriger* *Ebene* verwendet.

---
### Tauri
**Tauri** ist ein *Open-Source-Framework* zur Erstellung von *plattformübergreifenden* Desktop-Anwendungen mit Web-Technologien wie **HTML**, **CSS** und **JavaScript** bzw. **TypeScript**. Es ermöglicht Entwicklern, mit einer nativen *Rust-basierten Backend-Logik* zu interagieren, um die Leistung und Sicherheit von Desktop-Anwendungen zu optimieren. Tauri-Anwendungen sind *ressourcenschonend* und können für **Windows**, **macOS** und **Linux** gebaut werden.

---
### HTML (HyperText Markup Language)
**HTML** ist die *Standard-Auszeichnungssprache* für das *Erstellen* und *Strukturieren* von Inhalten auf *Webseiten*. Sie verwendet *Tags*, um verschiedene Elemente wie Überschriften, Absätze, Listen, Links und Medien zu definieren. HTML bildet das *Grundgerüst* einer Webseite und ist unerlässlich für die *Darstellung* von Inhalten im Web.

---
### CSS (Cascading Style Sheets)
**CSS** wird verwendet, um das visuelle *Design* und *Layout* von Webseiten zu *definieren* und zu *steuern*. Es ermöglicht Entwicklern, das *Aussehen* von HTML-Elementen zu verändern, einschließlich der *Farben*, *Schriftarten*, *Abstände* und *Positionen*. CSS trägt dazu bei, das Design einer Webseite *ansprechend* und *benutzerfreundlich* zu gestalten, indem es das HTML mit *Styles* trennt.

---
### JavaScript / TypeScript
**JavaScript** ist eine dynamische *Programmiersprache*, die es ermöglicht, Webseiten *interaktiv* zu gestalten. Mit JavaScript können Elemente auf der Seite in *Echtzeit* geändert, *Animationen* erstellt und *Benutzereingaben* verarbeitet werden. Es ist eine der grundlegenden Technologien für die Entwicklung moderner Webanwendungen und ermöglicht *serverseitige* sowie *clientseitige* Programmierung.

**TypeScript** ist eine von *Microsoft* entwickelte, *statisch* *typisierte* *Obermenge* von **JavaScript**, die die Verwendung von *Typen* und moderne *JavaScript-Funktionen* ermöglicht. Es fügt **JavaScript** die Möglichkeit hinzu, Variablen, Funktionen und Objekte *explizit* mit *Typen* zu deklarieren, was hilft, *Fehler* *frühzeitig* im Entwicklungsprozess zu *erkennen* und die *Wartbarkeit* von Code zu *verbessern*. **TypeScript** wird in **JavaScript** *transpiliert*, was bedeutet, dass der TypeScript-Code in *regulären* JavaScript-Code *umgewandelt* wird, der in *allen* *gängigen* *Browsern* und *Umgebungen* ausgeführt werden kann.

---
### Vue.js
**Vue.js** ist ein *progressives JavaScript-Framework*, das zur Erstellung von *Benutzeroberflächen* und *Single-Page-Anwendungen* verwendet wird. Es verfolgt einen *komponentenbasierten* Ansatz, bei dem die Benutzeroberfläche in *wiederverwendbare*, *isolierte* Komponenten unterteilt wird. **Vue.js** ist bekannt für seine *einfache Lernkurve*, *flexible Integration* in bestehende Projekte und *leistungsstarke Reaktivitätsmechanismen*, die eine effiziente *Aktualisierung* der UI bei *Änderungen* im Zustand ermöglichen.

Im **DMS** wurde **Vue.js** mit der *Composition API* verwendet.

---
### Bootstrap
**Bootstrap** ist ein populäres *Open-Source-Framework* für die Entwicklung von *responsiven* und *mobilen* Webseiten. Es bietet eine Sammlung von vorgefertigten *HTML-, CSS- und JavaScript-Komponenten*, die Entwicklern helfen, schnell *ansprechende* und *funktionale* Webanwendungen zu erstellen. Mit seinem *grid-basierten* Layout-System und einer Vielzahl von UI-Komponenten wie *Buttons*, *Navigation* und *Modals* erleichtert Bootstrap die *Gestaltung* von Webseiten, die auf *verschiedenen Geräten* gut aussehen und funktionieren.

---
### TailwindCSS
**TailwindCSS** ist ein *utility-first CSS-Framework*, das eine große Sammlung von *vordefinierten* *CSS-Klassen* bietet, um das *Styling* von HTML-Elementen direkt *im Markup* vorzunehmen. Im Gegensatz zu traditionellen CSS-Frameworks, bei denen vorgefertigte Komponenten verwendet werden, ermöglicht **TailwindCSS** eine *hohe* *Flexibilität* und *Anpassbarkeit*, indem es Entwicklern erlaubt, spezifische *Designentscheidungen* direkt in der HTML-Datei zu treffen. **Tailwind** fördert eine *schnelle* *Entwicklung* und *sauberen* *Code*, da es über eine konfigurierbare* Build-Pipeline* verfügt, die nicht benötigte CSS-Klassen entfernt.

## Funktionen

### Mindestanforderungen
Das **DMS** soll *mindestens* die Möglichkeit bieten, Datenbankverbindungen in einer internen SQLite-Datenbank zu speichern und von dort abzurufen.

Es soll *mindestens* in der Lage sein, von einer gespeicherten MySQL-Datenbank Daten einer Tabelle *auszulesen*, als *JSON* oder *CSV* zu *exportieren* und auf einen definierten Ort auf dem Gerät zu *speichern*.

Es soll *mindestens* die Fähigkeit besitzen, *JSON* und *CSV* Dateien aus einem definierten Ort zu *importieren* und die dort gespeicherten Daten in eine ausgewählte Tabelle korrekt zu übertragen.

### Best Case
Im Idealfall ist das **DMS** in der Lage, die [Mindestanforderungen](#mindestanforderungen) zu erfüllen.
Zudem soll es in der Lage sein, diese nicht nur für *MySQL*, sondern auch für weitere Datenbankanbieter,
also *MariaDB*, *PostgreSQL*, *SQL-Server* sowie *SQLite* zu erfüllen.

Es sollte in der Lage sein, verschiedene Nutzer und deren Berechtigung zu verwalten, sodass jeder Nutzer auf seine gespeicherten Datenbankverbindungen oder für ihn freigegebene Verbindungen Zugriff besitzt und diese bearbeiten oder entfernen kann.

Das **DMS** soll zudem nicht nur den Import und Export von Daten ermöglichen, sondern eine allgemeine Datenbankverwaltung darbieten, in welcher der Nutzer direkt im Programm Tabellen erstellen, bearbeiten und löschen kann.

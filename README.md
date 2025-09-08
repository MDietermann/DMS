# 🗄️ DMS - Database Management System

<div align="center">

![DMS Logo](https://img.shields.io/badge/DMS-Database_Management_System-blue?style=for-the-badge&logo=database)

**A powerful, cross-platform database management tool built with modern technologies**

[![Version](https://img.shields.io/badge/version-0.1.0-green.svg)](https://github.com/MDietermann/DMS)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Vue.js](https://img.shields.io/badge/vue.js-%2335495e.svg?style=flat&logo=vuedotjs&logoColor=%234FC08D)](https://vuejs.org/)
[![Tauri](https://img.shields.io/badge/tauri-%2324C8DB.svg?style=flat&logo=tauri&logoColor=%23FFFFFF)](https://tauri.app/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

[Features](#-features) •
[Installation](#-installation) •
[Usage](#-usage) •
[Tech Stack](#-tech-stack) •
[Contributing](#-contributing)

</div>

---

## 📋 Overview

**DMS (Datenbank-Management-System)** is a versatile, cross-platform desktop application designed to simplify database operations. Built with performance and user experience in mind, DMS provides a centralized solution for managing database connections, importing/exporting data, and performing various database operations across multiple database systems.

### 🎯 Key Highlights

- **🔄 Multi-Database Support**: Works with MySQL, PostgreSQL, SQLite, MariaDB, and SQL Server
- **📤📥 Smart Import/Export**: Seamlessly handle JSON and CSV data formats
- **🏎️ High Performance**: Built with Rust backend for speed and memory safety
- **🖥️ Cross-Platform**: Native desktop app for Windows, macOS, and Linux
- **🎨 Modern UI**: Beautiful, responsive interface built with Vue.js and Bootstrap
- **🔒 Secure**: Local storage of connection data with SQLite

---

## ✨ Features

### Core Functionality
- **Database Connection Management**
  - Store and manage database connections locally using SQLite
  - Support for multiple database types (MySQL, PostgreSQL, SQLite, MariaDB, SQL Server)
  - Secure credential storage and connection testing

- **Data Import/Export**
  - Export table data to JSON or CSV formats
  - Import data from JSON or CSV files into existing tables
  - Flexible file location selection
  - Data validation and error handling

- **User Interface**
  - Clean, intuitive dashboard
  - Real-time connection status
  - Advanced search and filtering capabilities
  - Responsive design for various screen sizes

### Advanced Features (Planned)
- **User Management**: Role-based access control and permissions
- **Direct Database Operations**: Create, edit, and delete tables through the UI
- **Query Builder**: Visual query construction interface
- **Data Visualization**: Charts and graphs for data analysis
- **Backup/Restore**: Automated database backup solutions

---

## 🚀 Installation

### Prerequisites

Before installing DMS, ensure you have the following installed:

- **Node.js** (LTS version) - [Download here](https://nodejs.org/)
- **Rust** - Install via [rustup](https://rustup.rs/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Tauri CLI**:
  ```bash
  cargo install tauri-cli
  ```

### Installation Steps

1. **Clone the repository**:
   ```bash
   git clone https://github.com/MDietermann/DMS.git
   cd DMS
   ```

2. **Install dependencies**:
   ```bash
   npm install
   ```

3. **Run in development mode**:
   ```bash
   npm run tauri dev
   ```

4. **Build for production**:
   ```bash
   npm run tauri build
   ```

The built application will be available in the `src-tauri/target/release` directory.

---

## 🎮 Usage

### Getting Started

1. **Launch DMS** after installation
2. **Add Database Connection**:
   - Navigate to "Datenbankverwaltung" (Database Management)
   - Enter your database credentials
   - Test the connection
   - Save for future use

3. **Export Data**:
   - Go to "Datenexport" (Data Export)
   - Select your database and table
   - Choose output format (JSON/CSV)
   - Select destination folder
   - Export your data

4. **Import Data**:
   - Navigate to "Datenimport" (Data Import)
   - Select source file (JSON/CSV)
   - Choose target database and table
   - Map columns and import

### Navigation

- **Startseite** (Home): Dashboard and overview
- **Datenimport** (Data Import): Import data from files
- **Datenexport** (Data Export): Export data to files
- **Datenbankverwaltung** (Database Management): Manage connections

---

## 🛠️ Tech Stack

### Backend
- **[Rust](https://www.rust-lang.org/)** - Systems programming language for performance and safety
- **[Tauri](https://tauri.app/)** - Framework for building cross-platform desktop apps
- **[SQLite](https://www.sqlite.org/)** - Local database for storing connection information
- **[Tokio](https://tokio.rs/)** - Asynchronous runtime for Rust

### Frontend
- **[Vue.js 3](https://vuejs.org/)** - Progressive JavaScript framework
- **[Vue Router](https://router.vuejs.org/)** - Official router for Vue.js
- **[Pinia](https://pinia.vuejs.org/)** - State management for Vue.js
- **[TypeScript](https://www.typescriptlang.org/)** - Typed superset of JavaScript

### Styling & UI
- **[Bootstrap 5](https://getbootstrap.com/)** - CSS framework for responsive design
- **[TailwindCSS](https://tailwindcss.com/)** - Utility-first CSS framework
- **[Bootstrap Vue 3](https://bootstrap-vue-next.github.io/bootstrap-vue-next/)** - Vue 3 components

### Build & Development
- **[Vite](https://vitejs.dev/)** - Fast build tool and development server
- **[PostCSS](https://postcss.org/)** - CSS processor
- **[Autoprefixer](https://github.com/postcss/autoprefixer)** - CSS vendor prefixes

### Database Connectivity
- **MySQL/MariaDB** - Via mysql crate
- **PostgreSQL** - Native support
- **SQLite** - Via rusqlite crate
- **SQL Server** - Native support

---

## 📁 Project Structure

```
DMS/
├── src/                    # Vue.js frontend source
│   ├── components/         # Vue components
│   │   ├── search/         # Search components
│   │   ├── offcanvas/      # Sidebar components
│   │   └── ui/             # UI components
│   ├── views/              # Vue views/pages
│   ├── stores/             # Pinia stores
│   ├── composables/        # Vue composables
│   ├── utils/              # Utility functions
│   ├── types/              # TypeScript type definitions
│   └── scripts/            # Helper scripts
├── src-tauri/              # Rust backend source
│   ├── src/                # Rust source code
│   ├── Cargo.toml          # Rust dependencies
│   ├── tauri.conf.json     # Tauri configuration
│   └── build.rs            # Build script
├── public/                 # Static assets
├── .devcontainer/          # VS Code dev container
├── .vscode/                # VS Code settings
└── package.json            # Node.js dependencies
```

---

## 🔧 Development

### Development Server
```bash
npm run tauri dev
```

### Building
```bash
npm run tauri build
```

### Linting & Formatting
```bash
npm run lint
npm run format
```

### Testing
```bash
npm run test
```

---

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

1. Fork the project
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

### Development Guidelines

- Follow Rust best practices and idioms
- Use TypeScript for all JavaScript code
- Follow Vue.js composition API patterns
- Write tests for new functionality
- Update documentation as needed

---

## 📝 Roadmap

### Version 0.2.0
- [ ] Enhanced error handling and logging
- [ ] Connection pooling and optimization
- [ ] Advanced data filtering options
- [ ] Batch operations support

### Version 0.3.0
- [ ] User authentication and authorization
- [ ] Role-based access control
- [ ] Database schema visualization
- [ ] SQL query builder interface

### Version 1.0.0
- [ ] Plugin system
- [ ] Data visualization charts
- [ ] Automated backups
- [ ] Multi-language support

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- [Tauri Team](https://tauri.app/) for the amazing cross-platform framework
- [Vue.js Team](https://vuejs.org/) for the excellent frontend framework
- [Rust Community](https://www.rust-lang.org/community) for the powerful systems language
- All contributors and supporters of this project

---

<div align="center">

**Made with ❤️ by [MDietermann](https://github.com/MDietermann)**

[⭐ Star this project](https://github.com/MDietermann/DMS) if you find it helpful!

</div>

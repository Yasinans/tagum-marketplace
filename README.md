# Tagum Marketplace - POS + IMS

**Tagum Marketplace POS + IMS** is a web-based Point of Sale (POS) and Inventory Management System (IMS) designed specifically for Tagum Marketplace. It provides a simple and efficient way to manage sales and inventory for the company.

This project is built using **Tauri** (for desktop application packaging), **Vue 3** (frontend framework), and **TypeScript** (for type safety). It also includes a `config.json` file for API configuration.

---

## Table of Contents

- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
- [Development](#development)
- [License](#license)

---

## Installation

To set up the project locally, follow these steps:

1. Clone the repository:
   ```bash
   git clone https://github.com/Yasinans/tagum-marketplace.git
   cd tagum-marketplace
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Build the project:
   ```bash
   npm run build
   ```

4. Run the Tauri application:
   ```bash
   npm run tauri dev
   ```

---

## Configuration

After building the application, a `config.json` file is generated in the `AppData/Roaming/tagum-marketplace` directory. This file contains the following default configuration:

```json
{
  "api_url": "http://localhost:3000"
}
```

### Modifying the Configuration
- If your API is hosted on a different URL, update the `api_url` value in the `config.json` file to point to the correct API endpoint.

---

## Usage

1. **Running the Application**:
   - Use the following command to start the application in development mode:
     ```bash
     npm run tauri dev
     ```
   - For production, build the application using:
     ```bash
     npm run tauri build
     ```

2. **Configuring the API**:
   - Ensure the `config.json` file in `AppData/Roaming/tagum-marketplace` has the correct `api_url` for your API.

---

## Development

This project is for school purposes and is not open for contributions. However, if you are working on it locally, here are some useful commands:

- **Run the development server**:
  ```bash
  npm run tauri dev
  ```
- **Build the project**:
  ```bash
  npm run tauri build
  ```

---

## License

This project does not have a specific license. It is intended for educational and school purposes only.

---

## Acknowledgments

This project was developed as part of a school requirement. Special thanks to the tools and libraries used, including **Tauri**, **Vue 3**, **TypeScript**, and **Tailwind CSS**.

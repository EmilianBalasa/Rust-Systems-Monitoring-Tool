# Rust Systems Monitoring Tool

This is a backend system monitoring service developed in Rust. The application functions as a web server that collects and exposes real-time data about the computer's status, such as CPU usage, RAM allocation, running processes, and disk storage.

---

## 🎓 Workshop Context

This project was developed during a **Rust Workshop**.

* **Author:** Emilian-Valentin Balasa
* **Objective:** To learn core and advanced Rust concepts, including asynchronous programming with `tokio`, building web services with `axum`, and real-time data streaming.

---

## 🚀 API Endpoints & Demo

This is a backend service. You can test its functionality using `curl` or any API client. The server runs on `http://localhost:8080`.

### 1. Health Check

Checks if the server is up and running.

* **Endpoint:** `GET /healthcheck`
* **`curl` Command:**
    ```bash
    curl http://localhost:8080/healthcheck
    ```
* **Response:** `OK`

### 2. Real-Time Metrics (Server-Sent Events)

Opens a stream that sends a JSON packet with all system metrics **every second**.

* **Endpoint:** `GET /realtime`
* **`curl` Command:**
    ```bash
    curl http://localhost:8080/realtime
    ```
* **Response (Stream Example):**
    ```json
    data: {"system":{...},"process":[...],"memory":{...},"cpu":{...},"disk":[...]}
    
    data: {"system":{...},"process":[...],"memory":{...},"cpu":{...},"disk":[...]}
    ...
    ```

### 3. Snapshot Metrics (JSON API)

Gets an on-demand snapshot of the metrics.

* **All Metrics:** `GET /metrics`
    ```bash
    curl http://localhost:8080/metrics
    ```
* **Specific Metrics:** `GET /metrics/{kind}`
    * `curl http://localhost:8080/metrics/cpu`
    * `curl http://localhost:8080/metrics/memory`
    * `curl http://localhost:8080/metrics/process`
    * `curl http://localhost:8080/metrics/disk`
    * `curl http://localhost:8080/metrics/system`

---

## 🛠️ Technologies Used

* **[Rust](https://www.rust-lang.org/)**: The main programming language.
* **[Axum](https://github.com/tokio-rs/axum)**: A modern web framework for building the API.
* **[Tokio](https://tokio.rs/)**: An asynchronous runtime for high-performance I/O operations.
* **[Sysinfo](https://crates.io/crates/sysinfo)**: A crate for reading system information (CPU, RAM, processes, etc.).
* **[Serde](https://serde.rs/)**: A framework for serializing and deserializing data into JSON.
* **[Tokio-Stream](https://crates.io/crates/tokio-stream)**: Used to create the Server-Sent Events (SSE) data stream.

---

## 💡 Important Note: Backend Service

This project is a **backend API service**. It does not include a graphical frontend to visualize the data.

> **What this means:**
>
> * **The application has NO visual interface.** It runs in a terminal and waits for HTTP requests.
> * **Its purpose** is to *provide* data (in JSON format) to other applications.
> * A **frontend** (a webpage made with React/Vue/Svelte, or even another Rust app) could connect to the `/realtime` endpoint to receive and display this data as graphs.

## 🏁 Conclusion and Future Work

This project successfully demonstrates how Rust can be used to create highly performant and efficient monitoring services.

Planned future steps could include:
* Creating a **web dashboard** (frontend) to consume this API.
* Containerizing the application using **Docker**.
* Adding new endpoints for actions (e.g., killing a process).

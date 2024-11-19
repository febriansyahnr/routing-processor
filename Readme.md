# Routing Processor
Implementasi dari Routing Processor menggunakan rust. Aplikasi ini dapat menentukan priority dari suatu processor.

## Project Structure
Project ini menggunakan rust workspace. Terdapat beberapa main folder seperti:
- `core`: Core merupakan shared library folder yang digunakan oleh beberapa binary folder.
- `seeder`: Seeder murpakan binary yang digunakan untuk mengisi database dengan data dummy.
- `web_api`: Web API merupakan binary yang digunakan untuk mengakses endpoint dari aplikasi ini.

### Core
Core merupakan shared library yang digunakan oleh beberapa binary project seperti `seeder`, `web_api`. Core ini berisi beberapa modul seperti:
- `repositories`
- `services`
- `models`
- `utils`
- `configs`

### Seeder
Seeder merupakan binary yang digunakan untuk mengisi database dengan data dummy.
Cara menjalankan seeder:
1. Pastikan `.env` sudah disesuaikan.
2. Jalankan migrasi up menggunakan perintah `sqlx migrate run --database-url mysql://user:pass@localhost/db_name`
3. Ubah nilai `let should_run` di `seeder/src/main.rs` menjadi `true`
4. Jalankan perintah `cargo run -p seeder`

### Web API
Web API merupakan binary yang digunakan untuk mengakses endpoint dari aplikasi ini.
Cara menjalankan Web API:
1. Pastikan `.env` sudah disesuaikan.
2. Jalankan perintah `cargo run -p web_api`

## Tech Stack
Ada beberapa tech stack yang digunakan pada project ini, seperti:
- Sqlx: Database driver.
- Chrono: library untuk memanipulasi waktu.
- Serde: Library untuk Serelize dan Deserialize JSON.
- Dotenvy: Library untuk membaca file .env.

## Tool
Ada beberapa tool yang digunakan pada project ini, seperti:
- Rust: Programming language.
- Cargo: Rust package manager.
- Sqlx-cli: Aplikasi untuk membuat migration dan menjalankan migration. `cargo install sqlx-cli`
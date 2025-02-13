# 🚀 Actix Web CRUD API with Diesel

A simple **RESTful API** built with **Rust**, **Actix-Web**, **Diesel**, and **SQLite** to perform CRUD operations on users.

---

## 📂 Prerequisites
- **Rust** installed ✅
- **Diesel CLI** installed:
  ```sh
  cargo install diesel_cli --no-default-features --features sqlite
  ```
- **SQLite database**

---

## 🛠 Database Setup
1. Create a `.env` file in the project root and add:
   ```
   DATABASE_URL=database.sqlite
   ```
2. Run Diesel migrations:
   ```sh
   diesel setup
   diesel migration run
   ```

---

## 🚀 Run the Server
```sh
cargo run
```
The server will run at `http://127.0.0.1:8080`

---

## 🌍 API Endpoints

| Method  | Endpoint       | Description          |
|---------|--------------|----------------------|
| **GET**  | `/users`      | Get all users       |
| **POST** | `/users`      | Create a new user   |
| **GET**  | `/users/{id}` | Get a specific user |
| **PUT**  | `/users/{id}` | Update a user       |
| **DELETE** | `/users/{id}` | Delete a user       |

---

## 📌 Testing the API
### ➤ **Create a New User**
```sh
curl -X POST -H "Content-Type: application/json" -d '{"name":"John","email":"john@example.com"}' http://127.0.0.1:8080/users
```

### ➤ **Get All Users**
```sh
curl -X GET http://127.0.0.1:8080/users
```

### ➤ **Get a Specific User**
```sh
curl -X GET http://127.0.0.1:8080/users/1
```

### ➤ **Update a User**
```sh
curl -X PUT -H "Content-Type: application/json" -d '{"name":"John Updated","email":"john_updated@example.com"}' http://127.0.0.1:8080/users/1
```

### ➤ **Delete a User**
```sh
curl -X DELETE http://127.0.0.1:8080/users/1
```

---

## 🔥 Technologies Used
- **Rust** 🦀
- **Actix-Web** 🚀
- **Diesel ORM** 🗄
- **SQLite** 🛢
- **SerDe** (for JSON serialization)

---

## 📜 Project Structure
```
📂 actix_web_api
├── 📂 src
│   ├── 📄 main.rs          # Main entry point
│   ├── 📄 db.rs            # Database connection
│   ├── 📄 models.rs        # Data models
│   ├── 📄 schema.rs        # Database schema
│   ├── 📄 handlers.rs      # CRUD operations
├── 📂 migrations           # Database migrations
├── 📄 Cargo.toml           # Dependencies
├── 📄 .env                 # Environment variables
├── 📄 README.md            # This file 📖
```

---

## 🛠 Contributing
To contribute:
1. **Fork** the project.
2. **Create a new branch** for your changes.
3. **Submit a pull request**.

---

## 📞 Contact
- 📧 **Email:** `your.email@example.com`
- 🔗 **GitHub:** [YourGitHub](https://github.com/yourusername)
```


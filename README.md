# Hackademy

**Hackademy** is a Rust-based cybersecurity and hacking quiz app built on:
- [Poem](https://github.com/poem-web/poem) (web framework)
- [Askama](https://github.com/djc/askama) (templating)
- [SurrealDB](https://surrealdb.com) (database)

## Getting Started

### Prerequisites
- Rust (if building locally)
- Docker & docker-compose (if containerizing)
- SurrealDB (running locally or in Docker)

### Running Locally
1. **Start SurrealDB**:
   ```bash
   surreal start --user root --pass root memory

	2.	Set up .env in the project root (see .env.example).
	3.	Run:

cargo run


	4.	Open: http://localhost:3000

Docker
	1.	Clone repository.
	2.	Modify .env with your desired config.
	3.	Build the image:

docker build -t hackademy .


	4.	Run container:

docker run -p 3000:3000 --name hackademy -e SURREALDB_URI=host.docker.internal:8000 hackademy

(You should have SurrealDB running on your host at port 8000.)

Docker Compose
	1.	Copy the example docker-compose.yml.
	2.	Run:

docker-compose up --build


	3.	Access Hackademy at http://localhost:3000.

Usage
	•	Register a new account under http://localhost:3000/auth/register.
	•	Login via http://localhost:3000/auth/login.
	•	Browse categories, subcategories, take quizzes, or search.

Contributing

Pull requests are welcome. For significant changes, open an issue first to discuss what you’d like to change.

License

MIT
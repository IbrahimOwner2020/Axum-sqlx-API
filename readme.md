# Getting Started with Axum and SQLx in Rust

This project serves as a starting point for learning how to use Axum, a web framework for Rust, along with SQLx, a SQL query builder and executor, to build web applications with a PostgreSQL database backend.

## Prerequisites

Before you begin, ensure you have met the following requirements:

-   Rust installed on your machine. If you haven't installed Rust yet, you can do so by following the instructions at [rustup.rs](https://rustup.rs/).
-   PostgreSQL installed and running locally or accessible via a remote server. You'll need a PostgreSQL database to follow along with the examples in this project.

## Installation

1. Clone this repository to your local machine:

    ```bash
    git clone https://github.com/IbrahimOwner2020/Axum-sqlx-API
    ```

2. Navigate to the project directory:

    ```bash
    cd your_project
    ```

3. Install project dependencies:

    ```bash
    cargo build
    ```

## Usage

1. Configure the PostgreSQL connection URL in the `.env` file. Rename `.env.example` to `.env` and update the `DATABASE_URL` value with your PostgreSQL connection URL.

2. Run the migration to set up the database schema:

    ```bash
    cargo run --bin migrate
    ```

3. Start the application server:

    ```bash
    cargo run
    ```

4. Visit `http://localhost:8000` in your web browser to see the application running.

## Learning Resources

-   [Axum Documentation](https://docs.rs/axum/latest/axum/)
-   [SQLx Documentation](https://docs.rs/sqlx/latest/sqlx/)

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request.

## License

This project is licensed under the [MIT License](LICENSE).

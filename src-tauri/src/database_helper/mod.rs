use tauri_plugin_sql::{ Migration, MigrationKind };

pub fn get_migrations() -> Vec<Migration>
{
    vec![
        Migration {
            version: 1,
            description: "create_users_table",
            sql: "CREATE TABLE users (
                id INTEGER PRIMARY KEY,
                role TEXT,
                username TEXT,
                password TEXT,
                notify INTEGER
            );",
            kind: MigrationKind::Up
        },
        Migration {
            version: 2,
            description: "create_categories_table",
            sql: "CREATE TABLE categories (
                id INTEGER PRIMARY KEY,
                type TEXT,
                name TEXT
            );",
            kind: MigrationKind::Up
        },
        Migration {
            version: 3,
            description: "create_transactions_table",
            sql: "CREATE TABLE transactions (
                id INTEGER PRIMARY KEY,
                user_id INTEGER,
                category_id INTEGER,
                description TEXT,
                type TEXT,
                amount INTEGER,
                method TEXT,
                date NUMERIC,
                FOREIGN KEY (user_id) REFERENCES users(id),
                FOREIGN KEY (category_id) REFERENCES categories(id)
            );",
            kind: MigrationKind::Up
        }
    ]
}

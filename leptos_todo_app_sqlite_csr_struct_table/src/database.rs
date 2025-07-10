use sqlx::{Row, Sqlite, migrate::MigrateDatabase};

static DB: std::sync::OnceLock<sqlx::SqlitePool> = std::sync::OnceLock::new();

pub async fn init_db() {
    // Create db if not exists.
    let db_url = "sqlite://db.sqlite3";
    if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
        match Sqlite::create_database(db_url).await {
            Ok(()) => {
                println!(">>> Successfully created the db.");
            }
            Err(e) => println!(">>> Failed to create the db: {e}"),
        };
    }

    // Setup the connection pool.
    let pool = sqlx::SqlitePool::connect("sqlite://db.sqlite3")
        .await
        .expect("Could not make pool.");
    println!(">>> Database pool initialized.");
    _ = DB.set(pool);

    // Create table if not exists.
    let create_table_q = r#"
        create table if not exists customers (
            id text,
            first_name text,
            last_name text,
            company text
        );"#;

    match sqlx::query(create_table_q).execute(get_db()).await {
        Ok(rs) => println!(">>> Successfully created table: {rs:?}"),
        Err(e) => {
            println!(">>> Failed to create table: {e}");
            return;
        }
    };

    // Populate the table, if empty.
    if let Err(e) = populate_customers().await {
        println!(">>> Failed to populate customers table: {e}");
    }
}

pub fn get_db<'a>() -> &'a sqlx::SqlitePool {
    DB.get().expect("database unitialized")
}

async fn populate_customers() -> Result<(), sqlx::Error> {
    //
    let count: i64 = sqlx::query("SELECT COUNT(*) FROM customers")
        .fetch_one(get_db())
        .await?
        .get(0);

    if count == 0 {
        for i in 1..=1000 {
            let query = format!(
                "INSERT INTO customers (id, first_name, last_name, company) \
                 VALUES ('{}', '{}', '{}', '{}');",
                i,
                format!("first_name_{}", i),
                format!("last_name_{}", i),
                format!("company_{}", i)
            );
            sqlx::query(query.as_str()).execute(get_db()).await?;
        }
        println!(">>> Successfully populated customers table.");
    } else {
        println!(">>> There are {count} customers in the database.");
    }

    Ok(())
}

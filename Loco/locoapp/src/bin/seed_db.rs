mod seed;
use seed::seed_articles;
use sea_orm::Database; 


#[tokio::main]
async fn main() {
    // Connexion à la DB SQLite
    let db = Database::connect("sqlite://db.sqlite").await
        .expect("Failed to connect to DB");

    match seed_articles(&db).await {
        Ok(_) => println!("Database seeded successfully!"),
        Err(e) => eprintln!("Failed to seed database: {}", e),
    }
}
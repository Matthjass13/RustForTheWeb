/*use sea_orm::entity::*;
use sea_orm::query::*;
use sea_orm::DatabaseConnection;
use sea_orm::DbErr;
use crate::entity::article;*/

use locoapp::models::_entities::articles; // ton modèle généré par cargo loco
use sea_orm::{DatabaseConnection, ActiveModelTrait, Set, DbErr};


pub async fn seed_articles(db: &DatabaseConnection) -> Result<(), DbErr> {
     let articles = vec![
        Article {
            id: None, // si tu as un champ auto-increment
            title: "Premier article".to_owned(),
            content: "Contenu du premier article".to_owned(),
        },
        Article {
            id: None,
            title: "Second article".to_owned(),
            content: "Contenu du second article".to_owned(),
        },
        // ajoute autant d’articles que tu veux
    ];

    for a in articles {
        let mut active: Article::ActiveModel = a.into(); // conversion en ActiveModel
        active.insert(db).await?;
    }

    Ok(())
}
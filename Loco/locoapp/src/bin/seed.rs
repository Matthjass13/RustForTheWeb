/* 
Not sure if this file is actually doing something in this project.
The app.rs file is seemingly used to seed the db instead.
In doubt, I let this file live for now.
*/

use locoapp::models::_entities::articles;
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
    ];

    for a in articles {
        let mut active: Article::ActiveModel = a.into();
        active.insert(db).await?;
    }

    Ok(())
}
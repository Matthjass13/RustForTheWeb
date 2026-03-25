use std::thread;

#[derive(Debug, serde::Serialize)]
pub struct ArticleAnalysis {
    pub id: i32,
    pub word_count: usize,
}

pub fn analyze_articles_parallel(
    articles: Vec<(i32, String)>,
) -> Vec<ArticleAnalysis> {
    let mut handles = vec![];

    for (id, content) in articles {
        let handle = thread::spawn(move || {
            let word_count = content.split_whitespace().count();

            ArticleAnalysis {
                id,
                word_count,
            }
        });

        handles.push(handle);
    }

    let mut results = vec![];

    for handle in handles {
        results.push(handle.join().unwrap());
    }

    results
}
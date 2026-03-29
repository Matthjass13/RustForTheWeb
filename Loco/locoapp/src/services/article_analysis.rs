use std::thread;
use std::time::Instant;

#[derive(Debug, serde::Serialize, Clone)]
pub struct ArticleAnalysis {
    pub id: i32,
    pub word_count: usize,
}

pub struct ParallelTiming {
    pub results: Vec<ArticleAnalysis>,
    pub spawn_time_ms: u128,
    pub execution_time_ms: u128,
}

pub fn analyze_articles_parallel_with_timing(
    articles: Vec<(i32, String)>,
) -> ParallelTiming {
    let mut handles = vec![];
s
    let start_spawn = Instant::now();

    for (id, content) in articles {
        let handle = thread::spawn(move || {
            let mut word_count = 0;


            // Artificial calculus to simulate a big operation
            for _ in 0..5_000_000 {
                let _ = 2 * 2;
            }


            ArticleAnalysis { id, word_count }
        });

        handles.push(handle);
    }

    let spawn_time = start_spawn.elapsed().as_millis();

    let start_exec = Instant::now();

    let mut results = vec![];

    for handle in handles {
        results.push(handle.join().unwrap());
    }

    let execution_time = start_exec.elapsed().as_millis();

    ParallelTiming {
        results,
        spawn_time_ms: spawn_time,
        execution_time_ms: execution_time,
    }
}

pub fn analyze_articles_sequential(
    articles: Vec<(i32, String)>,
) -> Vec<ArticleAnalysis> {
    articles
        .into_iter()
        .map(|(id, content)| {
            let word_count = content.split_whitespace().count();

            // Artificial calculus to simulate a big operation
            for _ in 0..5_000_000 {
                let _ = 2 * 2;
            }

            ArticleAnalysis {
                id,
                word_count,
            }
        })
        .collect()
}


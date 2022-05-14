use reqwest::{Client, Error};
use serde::Deserialize;
use std::fmt;

#[derive(Deserialize, Debug)]
struct HNItem {
    title: String,
    url: String,
    time: i32,
    score: i32,
    descendants: i32,
}

impl fmt::Display for HNItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {} {} {}",
            self.title, self.url, self.time, self.score, self.descendants
        )
    }
}

async fn get_top_stories() -> Result<Vec<i32>, Error> {
    let client = Client::new();
    let resp = client
        .get("https://hacker-news.firebaseio.com/v0/topstories.json")
        .send()
        .await?;
    let hn_items: Vec<i32> = resp.json().await?;
    Ok(hn_items)
}

async fn fetch_story_details(story_id: i32) -> Result<HNItem, Error> {
    let client = Client::new();
    let resp = client
        .get(
            format!(
                "https://hacker-news.firebaseio.com/v0/item/{}.json",
                story_id
            )
            .as_str(),
        )
        .send()
        .await?;
    let story_details: HNItem = resp.json().await?;
    Ok(story_details)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();
    let num_stories = args
        .get(1)
        .unwrap_or(&"10".to_string())
        .parse::<usize>()
        .unwrap();

    println!("Getting top {} stories from HackerNews...", num_stories);
    let top_stories = get_top_stories().await?;

    let top_n_stories = top_stories
        .iter()
        .take(num_stories)
        .cloned()
        .collect::<Vec<i32>>();

    for story_id in top_n_stories {
        let story_details = fetch_story_details(story_id).await?;
        println!("{}", story_details);
    }

    Ok(())
}

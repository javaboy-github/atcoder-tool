use regex::Regex;
use reqwest;

pub struct Task {
    pub id: String,
    pub label: String,
    pub title: String,
    pub url: String
}

pub struct Contest {
    pub id: String,
    pub title: String,
    pub url: String,
    pub tasks: Vec<Task>
}

impl Contest {
    pub fn get_contest_information(id: &str) -> Result<Contest, String> {
        let url = &format!("https://atcoder.jp/contests/{}", id);
        let body = reqwest::blocking::get(url).map_err(|e| "network error").unwrap().text().unwrap();
        let reg = Regex::new("<title>(.*) - AtCoder</title>").unwrap();
        let title_pos = reg.find(&body).unwrap();
        let title = body[title_pos.start()+7..title_pos.end()-18].to_string();
        Ok(Contest {
            id: String::from(id),
            title,
            url: url.to_string(),
            tasks: Vec::new()
        })
    }
}

#[test]
fn can_get_contest_information() {
    let contest = Contest::get_contest_information("abc300").unwrap();
    assert_eq!(contest.title, "UNIQUE VISION Programming Contest 2023 Spring(AtCoder Beginner Contest 300)");
}

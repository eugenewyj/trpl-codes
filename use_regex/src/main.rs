#[macro_use]
extern crate lazy_static;
use regex::Regex;

const TO_SEARCH: &'static str = "
On 2017-12-01, happy. On 2018-01-01, New Year
";

fn main() {
    test1();
    test2();
    test3();
}

fn test1() {
    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    for caps in re.captures_iter(TO_SEARCH) {
        println!(
            "year:{}, month:{}, day:{}",
            caps.get(1).unwrap().as_str(),
            caps.get(2).unwrap().as_str(),
            caps.get(3).unwrap().as_str()
        );
    }
}

fn test2() {
    let re = Regex::new(
        r"(?x)
    (?P<year>\d{4}) # the year
    -
    (?P<month>\d{2}) # the month
    -
    (?P<day>\d{2}) # the day
    ",
    )
    .unwrap();
    let caps = re.captures("2018-01-01").unwrap();
    assert_eq!("2018", &caps["year"]);
    assert_eq!("01", &caps["month"]);
    assert_eq!("01", &caps["day"]);
    println!(
        "year2:{}, month2:{}, day2:{}",
        &caps["year"], &caps["month"], &caps["day"]
    );
}

lazy_static! {
    static ref RE: Regex = Regex::new(
        r"(?x)
        (?P<year>\d{4})- # the year
        (?P<month>\d{2})- # the month
        (?P<day>\d{2}) # the day
    "
    )
    .unwrap();
    static ref EMAIL_RE: Regex = Regex::new(
        r"(?x)
        ^\w+@(?:gmail|163|qq)\.(?:com|cn|com\.cn|net)$
    "
    )
    .unwrap();
}

fn regex_date(text: &str) -> regex::Captures {
    RE.captures(text).unwrap()
}

fn regex_mail(text: &str) -> bool {
    EMAIL_RE.is_match(text)
}

fn test3() {
    let caps = regex_date("2018-01-01");
    assert_eq!("2018", &caps["year"]);
    assert_eq!("01", &caps["month"]);
    assert_eq!("01", &caps["day"]);
    println!(
        "year3:{}, month3:{}, day3:{}",
        &caps["year"], &caps["month"], &caps["day"]
    );
    let after = RE.replace_all("2018-01-01", "$month/$day/$year");
    assert_eq!(after, "01/01/2018");
    assert!(regex_mail("alex@gmail.com"), true);
    assert_eq!(regex_mail("alex@gmail.cn.com"), false);
}

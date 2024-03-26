
const URL: &str = "https://himawari8.nict.go.jp/img/D531106/1d/550";

fn main() {
    
    let date = chrono::Utc::now().format("%Y/%m/%d/%H");

    let url = format!("{}/{}{}", URL, date, "0000_0_0.png");
    print!("Downloading image from: {}", url);

    // get the picture and store it in memory
    let image_bytes = reqwest::blocking::get(&url)
        .unwrap()
        .error_for_status()
        .unwrap()
        .bytes()
        .unwrap();

    let img = image::load_from_memory(&image_bytes).unwrap();



}

use chrono::TimeDelta;


const URL: &str = "https://himawari8.nict.go.jp/img/D531106/1d/550";

fn main() {
    
    let date = chrono::Utc::now().checked_sub_signed(TimeDelta::try_hours(1).unwrap()).unwrap();
    let date = date.format("%Y/%m/%d/%H").to_string();


    

    let url = format!("{}/{}{}", URL, date, "0000_0_0.png");
    print!("Downloading image from: {}", url);


    
    wallpaper::set_mode(wallpaper::Mode::Fit).unwrap();
    wallpaper::set_from_url(&url).unwrap();



}

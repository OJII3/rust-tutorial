fn main() {
    let moon = 834400.0;
    let car_speed = 80.0;
    let btrain_speed = 300.0;
    println!("車で月まで{}", moon / car_speed / 24.0);
    println!("月まで新幹線で{}", moon/ btrain_speed / 24.0);
}

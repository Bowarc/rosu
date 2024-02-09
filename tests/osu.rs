
use std::io::BufRead;
#[test]
fn osu() {
    let f = std::fs::File::open("resources/Panda_Eyes_&_Teminite_-_Highscore_(Fort)_[Game Over].osu").unwrap();
    let reader = std::io::BufReader::new(f);
    // println!("{:?}", reader.lines().next());

    // panic!("");
    let mut p = osu_format::Parser::new(reader.lines());
    let data = p.parse().unwrap();

    for obj in data.timing_points.iter(){
        println!("{obj:?}\n")
    }
}
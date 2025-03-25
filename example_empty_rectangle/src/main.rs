use meet_in_the_middle::find_path;
use state::{PositionInRectangle, RectangleSize};

mod state;

fn main() {
    let rectangle_size = RectangleSize::new(
        2000.try_into().unwrap(),
        3000.try_into().unwrap());

    let source = PositionInRectangle::new(&rectangle_size, 57, 234);

    let target = PositionInRectangle::new(&rectangle_size, 2763, 1467);

    let path: Vec<_> = find_path(&source, &target).into_iter().map(|n| n.to_string()).collect();
    println!("Path found with length: {:?}", path.len());
    println!("Start: {:?}, End: {:?}", path.iter().take(4).collect::<Vec<_>>(), path.iter().rev().take(4).rev().collect::<Vec<_>>());
}
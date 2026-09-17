use signaalinsuodatin::filter::Filter;

fn main() {
    let filter: Filter = Filter::new().unwrap();
    filter.run();
}

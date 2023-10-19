mod merge_sorted;

fn main() {
  merge_sorted::MergeSortedArray::run(
    &mut [1, 2, 3, 0, 0, 0],
    3,
    &mut [2, 5, 6],
    3,
  );
  println!("Hello, world!");
}

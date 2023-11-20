mod merge_sorted;
mod remove_element;
mod duplicate_array;

fn main() {
  merge_sorted::MergeSortedArray::run(
    &mut [1, 2, 3, 0, 0, 0],
    3,
    &mut [2, 5, 6],
    3,
  );
  remove_element::RemoveElement::remove_element(&mut [3, 2, 2, 3], 3);
  println!("Hello, world!");
}

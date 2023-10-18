/**
 * Merge Sorted Array

 * You are given two integer arrays nums1 and nums2, sorted in non-decreasing order, and two integers m and n, representing the number of elements in nums1 and nums2 respectively.

Merge nums1 and nums2 into a single array sorted in non-decreasing order.

The final sorted array should not be returned by the function, but instead be stored inside the array nums1. To accommodate this, nums1 has a length of m + n, where the first m elements denote the elements that should be merged, and the last n elements are set to 0 and should be ignored. nums2 has a length of n.
*/

pub struct MergeSortedArray;

impl MergeSortedArray {
  pub fn run(nums1: &mut [i32], m: i32, nums2: &mut [i32], n: i32) {
    let (mut i, mut j, mut k) = (m - 1, n - 1, m + n - 1);

    while i >= 0 && j >= 0 {
      if nums1[i as usize] >= nums2[j as usize] {
        nums1[k as usize] = nums1[i as usize];
        i -= 1;
      } else {
        nums1[k as usize] = nums2[j as usize];
        j -= 1;
      }
      k -= 1;
    }

    while j >= 0 {
      nums1[k as usize] = nums2[j as usize];
      j -= 1;
      k -= 1;
    }
  }
}

#[test]
fn test_merge_array_sorted() {
  let mut test_one = vec![1, 2, 3, 0, 0, 0];
  let expected = vec![1, 2, 2, 3, 5, 6];

  let mut test_two = vec![1];
  let expected_two = vec![1];

  MergeSortedArray::run(&mut test_one, 3, &mut [2, 5, 6], 3);
  MergeSortedArray::run(&mut test_two, 1, &mut [], 0);

  assert_eq!(test_one, expected);
  assert_eq!(test_two, expected_two);
}

# Example algoritms

## 1. merge sorted

### Intuition

### Approach

### Complexity

- Time complexity:

<!-- Add your time complexity here, e.g. $$O(n)$$ -->

- Space complexity:

<!-- Add your space complexity here, e.g. $$O(n)$$ -->

### code

```
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
```

## 2. remove element

### Intuition

#### es

Mi primeras ideas fueron iterar el arreglo, para asi poder acceder a cada valor
de este y compararlos con el argumento que tuviera `val`

#### en

My first ideas were to iterate the array, so I could access each value of it and
compare them with the argument that had `val`

### Approach

### es

Mi enfoque se basó primero en eliminar los valores que fuesen diferentes a `val`

### en

My approach was to first remove values that were different from `val`

### Complexity

- Time complexity:

<!-- Add your time complexity here, e.g. $$O(n)$$ -->

- Space complexity:

<!-- Add your space complexity here, e.g. $$O(n)$$ -->

### Code

```
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = 0;
    let mut j = 0;

    while j < nums.len() {
      if nums[j] != val {
        nums[i] = nums[j];
        i += 1;
      }
      j += 1;
    }

    i as i32
    }
}
```

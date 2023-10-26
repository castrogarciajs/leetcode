/**
 * Dada una matriz de números enteros y un valor de número entero, elimine todas las apariciones de val en nums en el lugar. El orden de los elementos puede cambiarse. Luego devuelve el número de elementos en nums que no son iguales a val.

Considere la cantidad de elementos en nums que no son iguales a val be k. Para ser aceptado, debe hacer lo siguiente:

Cambie los números de la matriz de modo que los primeros k elementos de nums contengan los elementos que no sean iguales a val. Los elementos restantes de nums no son tan importantes como el tamaño de nums.
Regresar k.
Juez personalizado:
 */

pub struct RemoveElement;

impl RemoveElement {
  pub fn remove_element(nums: &mut [i32], val: i32) -> i32 {
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


#[test]
fn test_remove_element() {
  let mut nums = vec![3, 2, 2, 3];
  let val = 3;
  let result = RemoveElement::remove_element(&mut nums, val);
  assert_eq!(result, 2);
  assert_eq!(nums, vec![2, 2, 2, 3]);

  let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
  let val = 2;
  let result = RemoveElement::remove_element(&mut nums, val);
  assert_eq!(result, 5);
  assert_eq!(nums, vec![0, 1, 3, 0, 4, 0, 4, 2]);
}
/**
 * Dada una matriz de números enteros y un valor de número entero, elimine todas las apariciones de val en nums en el lugar. El orden de los elementos puede cambiarse. Luego devuelve el número de elementos en nums que no son iguales a val.

Considere la cantidad de elementos en nums que no son iguales a val be k. Para ser aceptado, debe hacer lo siguiente:

Cambie los números de la matriz de modo que los primeros k elementos de nums contengan los elementos que no sean iguales a val. Los elementos restantes de nums no son tan importantes como el tamaño de nums.
Regresar k.
Juez personalizado:
 */

export function removeElement(nums: number[], val: number) {
	let i = 0
	let j = 0

	while (j < nums.length) {
		if (nums[j] !== val) {
			nums[i] = nums[j]
			i++
		}
		j++
	}
	console.log(i)

	return i
}

removeElement([3, 2, 2, 3], 3)

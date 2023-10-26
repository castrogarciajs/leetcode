import { assertEquals } from './deps.ts'
import { mergeSortedArray } from './merge_sorted.ts'

Deno.test('mergeSortedArray', () => {
	const testOne = mergeSortedArray([1, 2, 3, 0, 0, 0], 3, [2, 5, 6], 3)
	const expected = [1, 2, 2, 3, 5, 6]
	const testTwo = mergeSortedArray([1], 1, [], 0)
	const expectedTwo = [1]

	assertEquals(testOne, expected)
	assertEquals(testTwo, expectedTwo)
})

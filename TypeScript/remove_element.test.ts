import { assertEquals } from './deps.ts'
import { removeElement } from './remove_element.ts'

Deno.test('removeElement', () => {
	const testOne = removeElement([3, 2, 2, 3], 3)
	const expected = 2
	const testTwo = removeElement([0, 1, 2, 2, 3, 0, 4, 2], 2)
	const expectedTwo = 5

	assertEquals(testOne, expected)
	assertEquals(testTwo, expectedTwo)
})

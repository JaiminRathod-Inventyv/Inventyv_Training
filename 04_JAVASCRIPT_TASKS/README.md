# Assignment: JavaScript Tasks - Promises and Symbols

This folder contains JavaScript assignments focused on Promises,Pattern and the Symbol data type.

## Files

- `task1_promise.js` – Demonstrates Promise creation with array manipulation and conditional resolution.
- `task3_promise_symbol.js` – Advanced version using Symbol data types for array definitions and manipulation.

---

## Task 1: Promise with Array Manipulation

**File:** `task1_promise.js`

**Problem Statement:**

1. Create a function that has an array defined and pass the first element and rest of the array to a second function.
2. The second function will have another array defined, then it will concatenate the arrays like: `[first_element, array(defined in 2nd function), rest_array(from array defined in 1st function)]`.
3. Return that concatenated array to the first function that stores it in a global array.
4. Create a promise that will resolve only if the sum of all elements in that concatenated array is greater than 35, otherwise reject it.

**Key Concepts:**

- Array manipulation (shift, spread operator)
- Promise creation and handling
- Promise resolution/rejection based on conditions
- `.then()` and `.catch()` for handling promise outcomes

---

## Task 3: Promise with Symbol Data Type

**File:** `task3_promise_symbol.js`

**Problem Statement:**
Same as Task 1, but with an additional requirement:

- The arrays and array2 should be defined using Symbol data type.

**Key Concepts:**

- Symbol data type in JavaScript
- Using Symbols as object keys
- Symbol property access
- Promise handling with Symbol-based data structures

---

## How to Run

### Prerequisites

- Node.js installed on your system

### Execution Steps

1. **For Task 1:**

   ```bash
   node task1_promise.js
   ```

2. **For Task 3:**
   ```bash
   node task3_promise_symbol.js
   ```

---

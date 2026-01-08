/* PROBLEM STATEMENT :
1. Create a function that has array defined and pass the first element and rest of the array to second function 
2. Second function will have another array defined than it will concat the arraya like [first_element , array(defined in 2nd function) , rest_array(from array defined in 1st function)]
3. return that concatenated array to first function that stores it in global array.
4. create a promise that will resolve only if sum of all elements in that concatenated array should be greater than 35 o.w. reject it. 
----->the array and array2 should be defined using Symbol data type.
*/

let combined_arr = [];

function first_func() {
  const arr_symbol = Symbol("arr");
  let arr = {
    [arr_symbol]: [Symbol(1), Symbol(2), Symbol(3), Symbol(4), Symbol(5)],
  };
  const first_el = arr[arr_symbol][0];
  arr[arr_symbol].shift();
  console.log("arr after shift:", arr);
  combined_arr = second_func(first_el, arr[arr_symbol]);
}

function second_func(first_el, rest_arr) {
  const arr2_symbol = Symbol("arr2");
  let arr2 = {
    [arr2_symbol]: [Symbol(6), Symbol(7),Symbol(8)],
  };
  let combined_arr = [first_el, ...arr2[arr2_symbol], ...rest_arr];
  console.log("combined array:", combined_arr);
  return combined_arr;
}

first_func();

new Promise((resolve, reject) => {
  let sum = combined_arr.reduce((acc, val) => acc + Number(val.description), 0);
  if (sum > 35) {
    resolve(sum);
    return;
  }
  reject("Sum is less than or equal to 35");
})
  .then((sum) => console.log("sum:", sum))
  .catch((error) => console.log("error:", error));

/* PROBLEM STATEMENT :
1. Create a function that has array defined and pass the first element and rest of the array to second function 
2. Second function will have another array defined than it will concat the arraya like [first_element , array(defined in 2nd function) , rest_array(from array defined in 1st function)]
3. return that concatenated array to first function that stores it in global array.
4. create a promise that will resolve only if sum of all elements in that concatenated array should be greater than 35 o.w. reject it. 
*/

let res = [];
function first_func() {
  let arr = [1, 2, 3, 4, 5];
  let first_el = arr[0];
  arr.shift();
  console.log(arr);
  res = second_func(first_el, arr);
}

function second_func(first_el, rest_arr) {
  let arr2 = [6, 7, 8];
  arr2 = [first_el, ...arr2, ...rest_arr];
  console.log(arr2);
  return arr2;
}

first_func();

new Promise((resolve, reject) => {
  let sum = res.reduce((acc, val) => acc + val, 0);
  if (sum > 35) {
    resolve(sum);
  }
  reject("Sum is less than or equal to 35");
})
  .then((sum) => {
    console.log("sum:", sum);
  })
  .catch((error) => {
    console.log("error:", error);
  });

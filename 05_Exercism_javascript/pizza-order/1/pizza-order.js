/// <reference path="./global.d.ts" />
//
// @ts-check

/**
 * Determine the price of the pizza given the pizza and optional extras
 *
 * @param {Pizza} pizza name of the pizza to be made
 * @param {Extra[]} extras list of extras
 *
 * @returns {number} the price of the pizza
 */
export function pizzaPrice(pizza, ...extras) {
  const prices = {
    'Margherita': 7,
    'Caprese': 9,
    'Formaggio': 10
  };
  if(extras.length === 0){
    return prices[pizza] || 0;
  }
  let extra_cost = 0;
  if(extras[0]==='ExtraSauce') extra_cost= 1;
  if(extras[0]==='ExtraToppings') extra_cost=2;
  return extra_cost + pizzaPrice(pizza,...extras.slice(1));
  // throw new Error('Remove this line and implement the function');
}

/**
 * Calculate the price of the total order, given individual orders
 *
 * (HINT: For this exercise, you can take a look at the supplied "global.d.ts" file
 * for a more info about the type definitions used)
 *
 * @param {PizzaOrder[]} pizzaOrders a list of pizza orders
 * @returns {number} the price of the total order
 */
export function orderPrice(pizzaOrders) {
   let total_price = 0;
  for(let pizza of pizzaOrders){
    total_price += pizzaPrice(pizza.pizza,...pizza.extras);
  }
  return total_price;
  throw new Error('Remove this line and implement the function');
}

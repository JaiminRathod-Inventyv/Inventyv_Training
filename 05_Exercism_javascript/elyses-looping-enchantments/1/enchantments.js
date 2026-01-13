// @ts-check

/**
 * Determine how many cards of a certain type there are in the deck
 *
 * @param {number[]} stack
 * @param {number} card
 *
 * @returns {number} number of cards of a single type there are in the deck
 */
export function cardTypeCheck(stack, card) {
  // 🚨 Use .forEach
  let cnt=0;
  stack.forEach((s)=>{
    if(s===card){
      cnt++;
    }
  })
  return cnt;
  throw new Error('Implement the cardTypeCheck function');
}

/**
 * Determine how many cards are odd or even
 *
 * @param {number[]} stack
 * @param {boolean} type the type of value to check for - odd or even
 * @returns {number} number of cards that are either odd or even (depending on `type`)
 */
export function determineOddEvenCards(stack, type) {
  
  // 🚨 Use a `for...of` loop
  let cnt =0 ;
  for(const s of stack){
    if(type===true && s%2===0){
      cnt++;
    }
    if(type===false && s%2!==0){
      cnt++;
    }
  }
  return cnt;
  throw new Error('Implement the determineOddEvenCards function');
}

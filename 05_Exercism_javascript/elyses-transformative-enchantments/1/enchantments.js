// @ts-check

/**
 * Double every card in the deck.
 *
 * @param {number[]} deck
 *
 * @returns {number[]} deck with every card doubled
 */
export function seeingDouble(deck) {
  return deck.map((d)=>2*d);
  throw new Error('Implement the seeingDouble function');
}

/**
 *  Creates triplicates of every 3 found in the deck.
 *
 * @param {number[]} deck
 *
 * @returns {number[]} deck with triplicate 3s
 */
export function threeOfEachThree(deck) {
  for(let i=0;i<deck.length;i++){
    if(deck[i]===3){
      deck.splice(i+1,0,3,3);
      i += 3;
    }
  }
  return deck;
  throw new Error('Implement the threeOfEachThree function');
}

/**
 * Extracts the middle two cards from a deck.
 * Assumes a deck is always 10 cards.
 *
 * @param {number[]} deck of 10 cards
 *
 * @returns {number[]} deck with only two middle cards
 */
export function middleTwo(deck) {
   const middleIndex1 = deck.length / 2 - 1;
  const middleIndex2 = deck.length / 2;
  return [deck[middleIndex1], deck[middleIndex2]];
  throw new Error('Implement the middleTwo function');
}

/**
 * Moves the outside two cards to the middle.
 *
 * @param {number[]} deck with even number of cards
 *
 * @returns {number[]} transformed deck
 */

export function sandwichTrick(deck) {
  const first = deck.shift();
  const last = deck.pop();
  const midIndex = deck.length / 2;
  deck.splice(midIndex, 0, last, first);
  return deck;
  throw new Error('Implement the sandwichTrick function');
}

/**
 * Removes every card from the deck except 2s.
 *
 * @param {number[]} deck
 *
 * @returns {number[]} deck with only 2s
 */
export function twoIsSpecial(deck) {
  return deck.filter((d)=>d===2);
  throw new Error('Implement the twoIsSpecial function');
}

/**
 * Returns a perfectly order deck from lowest to highest.
 *
 * @param {number[]} deck shuffled deck
 *
 * @returns {number[]} ordered deck
 */
export function perfectlyOrdered(deck) {
  return deck.sort((a,b)=>a-b);
  throw new Error('Implement the perfectlyOrdered function');
}

/**
 * Reorders the deck so that the top card ends up at the bottom.
 *
 * @param {number[]} deck
 *
 * @returns {number[]} reordered deck
 */
export function reorder(deck) {
  return deck.reverse();
  throw new Error('Implement the reorder function');
}

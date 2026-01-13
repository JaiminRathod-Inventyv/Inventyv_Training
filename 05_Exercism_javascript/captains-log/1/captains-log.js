// @ts-check

/**
 * Generates a random starship registry number.
 *
 * @returns {string} the generated registry number.
 */
export function randomShipRegistryNumber() {
  let max = 9999;
  let min = 1000;
  let number = Math.random() * (max - min) + min;
  return  `NCC-${number}`;
  throw new Error(
    'Please remove this line and implement the randomShipRegistryNumber() function',
  );
}

/**
 * Generates a random stardate.
 *
 * @returns {number} a stardate between 41000 (inclusive) and 42000 (exclusive).
 */
export function randomStardate() {
  let max = 42000.0;
  let min = 41000.0;
 return Math.random() * (max - min) + min;
   
  throw new Error(
    'Please remove this line and implement the randomStardate() function',
  );
}

/**
 * Generates a random planet class.
 *
 * @returns {string} a one-letter planet class.
 */
export function randomPlanetClass() {
  const classes = ['D', 'H', 'J', 'K', 'L', 'M', 'N', 'R', 'T', 'Y'];
  const index = Math.floor(Math.random() * classes.length);
  return classes[index];
  throw new Error(
    'Please remove this line and implement the randomStardate() function',
  );
}

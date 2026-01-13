/// <reference path="./global.d.ts" />
// @ts-check

/**
 * Implement the functions needed to solve the exercise here.
 * Do not forget to export them so they are available for the
 * tests. Here an example of the syntax as reminder:
 *
 * export function yourFunction(...) {
 *   ...
 * }
 */
export function cookingStatus(remainingTime=-1){
 if (remainingTime === 0) return 'Lasagna is done.';
  if (remainingTime === -1) return 'You forgot to set the timer.';
  return 'Not done, please wait.';
}


export function preparationTime(layers, minutes=2){
 return layers.length *minutes;
}

export function quantities (layers){
  let noodles_cnt = 0;
  let sauce_cnt = 0;
  for (let i = 0; i < layers.length; i++) {
    if(layers[i] === 'noodles') noodles_cnt += 50;
    else if(layers[i] === 'sauce') sauce_cnt += 0.2;
  }
  return {
    noodles : noodles_cnt,
    sauce : sauce_cnt
  }
}

export function addSecretIngredient (friendsList, myList) {
  myList.push(friendsList[friendsList.length -1]); 
}

export function scaleRecipe(recipe, portion=2){
  let newrecipe = {};
  for(let i in recipe){
    newrecipe[i] = recipe[i] * portion/2;
  }
  return newrecipe;
}




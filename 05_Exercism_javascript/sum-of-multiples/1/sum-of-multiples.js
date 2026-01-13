//
// This is only a SKELETON file for the 'Sum Of Multiples' exercise. It's been provided as a
// convenience to get you started writing code faster.
//

export const sum = (arr , level) => {
  const mySet = new Set();
  arr.forEach((a)=>{
    if (a === 0) return;
    for (let i = a; i < level; i += a) {
      mySet.add(i);
    }
  })
  let summ = 0;
  for(const i of mySet){
    summ += i;
  }
  return summ;
  
  throw new Error('Remove this line and implement the function');
};

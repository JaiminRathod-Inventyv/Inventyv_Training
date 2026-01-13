// @ts-check
//
// The line above enables type checking for this file. Various IDEs interpret
// the @ts-check directive. It will give you helpful autocompletion when
// implementing this exercise.

/**
 * Removes duplicate tracks from a playlist.
 *
 * @param {string[]} playlist
 * @returns {string[]} new playlist with unique entries
 */
export function removeDuplicates(playlist) {
  const set = new Set();
  playlist.forEach((p)=>{
    set.add(p);
  })
  return [...set];
  // throw new Error('Remove this line and implement the function');
}

/**
 * Checks whether a playlist includes a track.
 *
 * @param {string[]} playlist
 * @param {string} track
 * @returns {boolean} whether the track is in the playlist
 */
export function hasTrack(playlist, track) {
  const set = new Set();
  playlist.forEach((p)=>{
    set.add(p);
  })
  return set.has(track);
  // throw new Error('Remove this line and implement the function');
}

/**
 * Adds a track to a playlist.
 *
 * @param {string[]} playlist
 * @param {string} track
 * @returns {string[]} new playlist
 */
export function addTrack(playlist, track) {
   playlist.push(track);
   return removeDuplicates(playlist);
  // throw new Error('Remove this line and implement the function');
}

/**
 * Deletes a track from a playlist.
 *
 * @param {string[]} playlist
 * @param {string} track
 * @returns {string[]} new playlist
 */
export function deleteTrack(playlist, track) {
 playlist =  playlist.filter((p)=>{
   return p !== track;
 })
  return playlist;
  // throw new Error('Remove this line and implement the function');
}

/**
 * Lists the unique artists in a playlist.
 *
 * @param {string[]} playlist
 * @returns {string[]} list of artists
 */
export function listArtists(playlist) {
  const set = new Set();
  playlist.forEach((p)=>{
    set.add(p.slice(p.indexOf('-')+1).trim());
  })
  return [...set];
  throw new Error('Remove this line and implement the function');
}

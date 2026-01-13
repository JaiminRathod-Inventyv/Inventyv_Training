// @ts-check

/**
 * Given a certain command, help the chatbot recognize whether the command is valid or not.
 *
 * @param {string} command
 * @returns {boolean} whether or not is the command valid
 */

export function isValidCommand(command) {
  return /^chatbot/i.test(command);
  throw new Error('Remove this line and implement the function');
}

/**
 * Given a certain message, help the chatbot get rid of all the emoji's encryption through the message.
 *
 * @param {string} message
 * @returns {string} The message without the emojis encryption
 */
export function removeEmoji(message) {
  const reg = new RegExp('emoji\\d+','g');
  return message.replace(reg,'');
  throw new Error('Remove this line and implement the function');
}

/**
 * Given a certain phone number, help the chatbot recognize whether it is in the correct format.
 *
 * @param {string} number
 * @returns {string} the Chatbot response to the phone Validation
 */
export function checkPhoneNumber(number) {
  const reg = /\(\+[0-9]{2}\)\s[0-9]{3}-[0-9]{3}-[0-9]{3}/;
  if(reg.test(number)){
    return 'Thanks! You can now download me to your phone.';
  }
  return `Oops, it seems like I can't reach out to ${number}`;
  throw new Error('Remove this line and implement the function');
}

/**
 * Given a certain response from the user, help the chatbot get only the URL.
 *
 * @param {string} userInput
 * @returns {string[] | null} all the possible URL's that the user may have answered
 */
export function getURL(userInput) {
  return userInput.match(/\S*\.\S*/g)
  throw new Error('Remove this line and implement the function');
}

/**
 * Greet the user using the full name data from the profile.
 *
 * @param {string} fullName
 * @returns {string} Greeting from the chatbot
 */
export function niceToMeetYou(fullName) {
  let str = []
  str = fullName.split(", ")
  console.log(str)
  return "Nice to meet you, " + str[1] + " " + str[0];
  throw new Error('Remove this line and implement the function');
}

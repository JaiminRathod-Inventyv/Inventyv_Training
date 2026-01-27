import readline from "readline";

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
});

function pattern(n) {
  for (let i = 1; i <= n; i++) {
    let line = "";
    for (let j = 1; j <= n; j++) {
      line +=
        (i < n - i + 1 ? i : n - i + 1) < (j < n - j + 1 ? j : n - j + 1)
          ? i < n - i + 1
            ? i
            : n - i + 1
          : j < n - j + 1
          ? j
          : n - j + 1;
    }
    console.log(line);
  }
}
rl.question("Enter a number: ", (input) => {
  const n = Number(input);
  pattern(n);
  rl.close();
});

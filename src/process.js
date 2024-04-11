import { exec } from 'child_process'

export function processDiff(file, folder, url) {
  const mockGenerator = `npx har-to-mocks ./${file} ./${folder} --url=${url}`

  exec(mockGenerator, (error, stdout, stderr) => {
    if (error) {
      console.error(`Error executing npm script: ${error.message}`);
      process.exit(1);
    }
    console.log(stderr);
    console.log(stdout);
  });
}

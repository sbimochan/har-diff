import { exec } from 'child_process';

export function processMock(file, folder, url) {
  const mockGenerator = `npx har-to-mocks ./${file} ./${folder} --url=${url}`;

  return new Promise((resolve, reject) => {
    exec(mockGenerator, (error, stdout) => {
      if (error) {
        console.error(`Error executing npm script: ${error.message}`);
        return reject(error);
      }
      if (stderr) return reject(stderr);
      console.log(stdout);
    });
    console.log('Mocks generated');
    console.log('Sorting all json files');
    exec(`npx json-sort-cli ${folder}*`, (error) => {
      if (error) return reject(error);

      if (stderr) return reject(stderr);
    });
    console.log('Sorting files completed');
  });
}

export function processDiff() {
  exec(`git add source target`);
  console.log('Stage source and target files');
  exec(`git commit -m "Temp commit of source and target folder ${Date.now()})}"`);
  exec(`mv source/* target/`);
}

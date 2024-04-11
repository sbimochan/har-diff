import { exec } from 'child_process';

export function processDiff(file, folder, url) {
  const mockGenerator = `npx har-to-mocks ./${file} ./${folder} --url=${url}`;

  exec(mockGenerator, (error, stdout) => {
    if (error) {
      console.error(`Error executing npm script: ${error.message}`);
      process.exit(1);
    }
    // console.log(stderr);
    console.log(stdout);
  })
  console.log('Mocks generated')
  console.log('Sorting all json files')
  exec(`npx json-sort-cli source*`, (error) => {
    console.error(error)
  })
  exec(`npx json-sort-cli target*`, (error) => {
    console.error(error)
  })
  console.log('Sorting files completed')

}

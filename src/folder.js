import { execSync } from 'child_process';
import fs from 'fs';

export async function makeFolder(folderPath) {
  console.log(folderPath)
  await fs.accessSync(folderPath, fs.constants.F_OK, async (err) => {
    if (!err) {
      await execSync(`rm -rf ${folderPath}`, async(error, stdout, stderr) => {
        if (error) {
          console.error(`Error deleting folder: ${error.message}`);
          return;
        }
        if (stderr) {
          console.error(`Error deleting folder: ${stderr}`);
          return;
        }
        console.log(` ${folderPath} Folder deleted`);
        await createFolder(folderPath);
      });
    } else {
      await createFolder(folderPath);
    }
  });
}

async function createFolder(folderPath) {
  await execSync(`mkdir ${folderPath}`, (error, stdout, stderr) => {
    if (error) {
      console.error(`Error creating folder: ${error.message}`);
      return;
    }
    if (stderr) {
      console.error(`Error creating folder: ${stderr}`);
      return;
    }
    console.log('Folder created');
  });
}

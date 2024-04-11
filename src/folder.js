import { exec } from 'child_process';
import fs from 'fs';

export function makeFolder(folderPath) {
  fs.access(folderPath, fs.constants.F_OK, (err) => {
    if (!err) {
      exec(`rm -rf ${folderPath}`, (error, stdout, stderr) => {
        if (error) {
          console.error(`Error deleting folder: ${error.message}`);
          return;
        }
        if (stderr) {
          console.error(`Error deleting folder: ${stderr}`);
          return;
        }
        console.log(` ${folderPath} Folder deleted`);
        createFolder(folderPath);
      });
    } else {
      createFolder(folderPath);
    }
  });
}

function createFolder(folderPath) {
  exec(`mkdir ${folderPath}`, (error, stdout, stderr) => {
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

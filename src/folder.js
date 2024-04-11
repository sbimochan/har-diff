import { exec } from 'child_process';
import fs from 'fs';

// Path to the folder
export function makeFolder(folderPath) {
  // Check if the folder exists
  fs.access(folderPath, fs.constants.F_OK, (err) => {
    if (!err) {
      // Folder exists, delete it
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
      // Folder doesn't exist, create it directly
      createFolder(folderPath);
    }
  });
}

// Function to create the folder
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

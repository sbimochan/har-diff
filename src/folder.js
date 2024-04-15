import { execSync } from 'child_process';
import fs from 'fs';

export async function makeFolder(folderPath) {
  try {
    await execSync(`rm -rf ${folderPath}`);
    console.log(`${folderPath} folder deleted`);
  } catch (error) {
    if (error.code !== 'ENOENT') {
      console.error(`Error deleting folder: ${error.message}`);
      throw error;
    }
  }
  await createFolder(folderPath);
}

async function createFolder(folderPath) {
  try {
    await execSync(`mkdir ${folderPath}`);
    console.log('Folder created');
  } catch (error) {
    console.error(`Error creating folder: ${error.message}`);
    throw error;
  }
}

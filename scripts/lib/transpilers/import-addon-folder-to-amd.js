// TODO: this module should be tested and optimized
import fs from "fs-extra";
import convertESModuletoAMD from "./convert-es-module-to-amd.js";
import convertHBSToAMD from "./convert-hbs-to-amd.js";
import findProjectRoot from "../utils/find-project-root.js";
import lookup from "../utils/recursive-file-lookup.js";
import { dirname } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

export default function(moduleName, addonPath, projectRoot, options) {
  return new Promise(async (resolve, reject) => {
    const PROJECT_ROOT = projectRoot || (await findProjectRoot());
    const packagePath = await getAddonPath(addonPath, PROJECT_ROOT);

    if (!packagePath) {
      return resolve("");
    }

    let targetFiles = [];

    lookup(packagePath, ["hbs", "js", "ts"], options)
      .then(files => {
        targetFiles = files;

        return Promise.all(files.map(fileName => fs.readFile(fileName)));
      })
      .then(async contents => {
        const convertions = contents.map((content, index) =>
          convertFile(content, moduleName, targetFiles[index], addonPath)
        );
        const transformedFiles = await Promise.all(convertions);

        return resolve(transformedFiles.join("\n"));
      })
      .catch(error => {
        console.log(error);
        console.log(`importAddonFolderToAMD error: ${error}`);
        reject(error);
      });
  });
}

async function getAddonPath(addonPath, projectRoot) {
  const mberPackage = `${__dirname}/../../node_modules/${addonPath}`;

  if (await fs.pathExists(mberPackage)) {
    return mberPackage;
  }

  const targetPath = addonPath.startsWith(projectRoot)
    ? addonPath
    : `${projectRoot}/node_modules/${addonPath}`;

  if (await fs.pathExists(targetPath)) {
    return targetPath;
  } else if (await fs.pathExists(addonPath)) {
    return addonPath;
  }

  return null;
}

function convertFile(
  code,
  libraryName,
  fileAbsolutePath,
  moduleEntryPoint = "ember-data/addon"
) {
  const startIndex = fileAbsolutePath.indexOf(moduleEntryPoint);
  const moduleName = fileAbsolutePath.slice(
    startIndex + moduleEntryPoint.length
  );

  if (fileAbsolutePath.endsWith(".js") || fileAbsolutePath.endsWith(".ts")) {
    const finalModuleName = `${libraryName}${moduleName.slice(0, -3)}`;

    return convertESModuletoAMD(code, { moduleName: finalModuleName });
  }

  const finalModuleName = `${libraryName}${moduleName.slice(0, -4)}`;

  return convertHBSToAMD(code, { moduleName: finalModuleName });
}

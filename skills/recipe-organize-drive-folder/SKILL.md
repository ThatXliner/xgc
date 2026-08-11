---
name: recipe-organize-drive-folder
description: "Create a Google Drive folder structure and move files into the right locations."
metadata:
  version: 0.22.5
  openclaw:
    category: "recipe"
    domain: "productivity"
    requires:
      bins:
        - xgc
      skills:
        - xgc-drive
---

# Organize Files into Google Drive Folders

> **PREREQUISITE:** Load the following skills to execute this recipe: `xgc-drive`

Create a Google Drive folder structure and move files into the right locations.

## Steps

1. Create a project folder: `xgc drive files create --json '{"name": "Q2 Project", "mimeType": "application/vnd.google-apps.folder"}'`
2. Create sub-folders: `xgc drive files create --json '{"name": "Documents", "mimeType": "application/vnd.google-apps.folder", "parents": ["PARENT_FOLDER_ID"]}'`
3. Move existing files into folder: `xgc drive files update --params '{"fileId": "FILE_ID", "addParents": "FOLDER_ID", "removeParents": "OLD_PARENT_ID"}'`
4. Verify structure: `xgc drive files list --params '{"q": "FOLDER_ID in parents"}' --format table`

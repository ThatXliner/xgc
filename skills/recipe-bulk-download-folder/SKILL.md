---
name: recipe-bulk-download-folder
description: "List and download all files from a Google Drive folder."
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

# Bulk Download Drive Folder

> **PREREQUISITE:** Load the following skills to execute this recipe: `xgc-drive`

List and download all files from a Google Drive folder.

## Steps

1. List files in folder: `xgc drive files list --params '{"q": "'\''FOLDER_ID'\'' in parents"}' --format json`
2. Download each file: `xgc drive files get --params '{"fileId": "FILE_ID", "alt": "media"}' -o filename.ext`
3. Export Google Docs as PDF: `xgc drive files export --params '{"fileId": "FILE_ID", "mimeType": "application/pdf"}' -o document.pdf`

---
name: recipe-email-drive-link
description: "Share a Google Drive file and email the link with a message to recipients."
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
        - xgc-gmail
---

# Email a Google Drive File Link

> **PREREQUISITE:** Load the following skills to execute this recipe: `xgc-drive`, `xgc-gmail`

Share a Google Drive file and email the link with a message to recipients.

## Steps

1. Find the file: `xgc drive files list --params '{"q": "name = '\''Quarterly Report'\''"}'`
2. Share the file: `xgc drive permissions create --params '{"fileId": "FILE_ID"}' --json '{"role": "reader", "type": "user", "emailAddress": "client@example.com"}'`
3. Email the link: `xgc gmail +send --to client@example.com --subject 'Quarterly Report' --body 'Hi, please find the report here: https://docs.google.com/document/d/FILE_ID'`

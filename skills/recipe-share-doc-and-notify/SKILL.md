---
name: recipe-share-doc-and-notify
description: "Share a Google Docs document with edit access and email collaborators the link."
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
        - xgc-docs
        - xgc-gmail
---

# Share a Google Doc and Notify Collaborators

> **PREREQUISITE:** Load the following skills to execute this recipe: `xgc-drive`, `xgc-docs`, `xgc-gmail`

Share a Google Docs document with edit access and email collaborators the link.

## Steps

1. Find the doc: `xgc drive files list --params '{"q": "name contains '\''Project Brief'\'' and mimeType = '\''application/vnd.google-apps.document'\''"}'`
2. Share with editor access: `xgc drive permissions create --params '{"fileId": "DOC_ID"}' --json '{"role": "writer", "type": "user", "emailAddress": "reviewer@company.com"}'`
3. Email the link: `xgc gmail +send --to reviewer@company.com --subject 'Please review: Project Brief' --body 'I have shared the project brief with you: https://docs.google.com/document/d/DOC_ID'`

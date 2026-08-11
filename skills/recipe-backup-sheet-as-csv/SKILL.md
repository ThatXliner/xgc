---
name: recipe-backup-sheet-as-csv
description: "Export a Google Sheets spreadsheet as a CSV file for local backup or processing."
metadata:
  version: 0.22.5
  openclaw:
    category: "recipe"
    domain: "productivity"
    requires:
      bins:
        - xgc
      skills:
        - xgc-sheets
        - xgc-drive
---

# Export a Google Sheet as CSV

> **PREREQUISITE:** Load the following skills to execute this recipe: `xgc-sheets`, `xgc-drive`

Export a Google Sheets spreadsheet as a CSV file for local backup or processing.

## Steps

1. Get spreadsheet details: `xgc sheets spreadsheets get --params '{"spreadsheetId": "SHEET_ID"}'`
2. Export as CSV: `xgc drive files export --params '{"fileId": "SHEET_ID", "mimeType": "text/csv"}'`
3. Or read values directly: `xgc sheets +read --spreadsheet SHEET_ID --range 'Sheet1' --format csv`

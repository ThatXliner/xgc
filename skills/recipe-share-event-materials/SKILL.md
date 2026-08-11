---
name: recipe-share-event-materials
description: "Share Google Drive files with all attendees of a Google Calendar event."
metadata:
  version: 0.22.5
  openclaw:
    category: "recipe"
    domain: "productivity"
    requires:
      bins:
        - xgc
      skills:
        - xgc-calendar
        - xgc-drive
---

# Share Files with Meeting Attendees

> **PREREQUISITE:** Load the following skills to execute this recipe: `xgc-calendar`, `xgc-drive`

Share Google Drive files with all attendees of a Google Calendar event.

## Steps

1. Get event attendees: `xgc calendar events get --params '{"calendarId": "primary", "eventId": "EVENT_ID"}'`
2. Share file with each attendee: `xgc drive permissions create --params '{"fileId": "FILE_ID"}' --json '{"role": "reader", "type": "user", "emailAddress": "attendee@company.com"}'`
3. Verify sharing: `xgc drive permissions list --params '{"fileId": "FILE_ID"}' --format table`

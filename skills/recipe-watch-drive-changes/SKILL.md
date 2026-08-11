---
name: recipe-watch-drive-changes
description: "Subscribe to change notifications on a Google Drive file or folder."
metadata:
  version: 0.22.5
  openclaw:
    category: "recipe"
    domain: "engineering"
    requires:
      bins:
        - xgc
      skills:
        - xgc-events
---

# Watch for Drive Changes

> **PREREQUISITE:** Load the following skills to execute this recipe: `xgc-events`

Subscribe to change notifications on a Google Drive file or folder.

## Steps

1. Create subscription: `xgc events subscriptions create --json '{"targetResource": "//drive.googleapis.com/drives/DRIVE_ID", "eventTypes": ["google.workspace.drive.file.v1.updated"], "notificationEndpoint": {"pubsubTopic": "projects/PROJECT/topics/TOPIC"}, "payloadOptions": {"includeResource": true}}'`
2. List active subscriptions: `xgc events subscriptions list`
3. Renew before expiry: `xgc events +renew --subscription SUBSCRIPTION_ID`

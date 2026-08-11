---
name: recipe-create-presentation
description: "Create a new Google Slides presentation and add initial slides."
metadata:
  version: 0.22.5
  openclaw:
    category: "recipe"
    domain: "productivity"
    requires:
      bins:
        - xgc
      skills:
        - xgc-slides
---

# Create a Google Slides Presentation

> **PREREQUISITE:** Load the following skills to execute this recipe: `xgc-slides`

Create a new Google Slides presentation and add initial slides.

## Steps

1. Create presentation: `xgc slides presentations create --json '{"title": "Quarterly Review Q2"}'`
2. Get the presentation ID from the response
3. Share with team: `xgc drive permissions create --params '{"fileId": "PRESENTATION_ID"}' --json '{"role": "writer", "type": "user", "emailAddress": "team@company.com"}'`

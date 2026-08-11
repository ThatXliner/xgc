---
name: recipe-collect-form-responses
description: "Retrieve and review responses from a Google Form."
metadata:
  version: 0.22.5
  openclaw:
    category: "recipe"
    domain: "productivity"
    requires:
      bins:
        - xgc
      skills:
        - xgc-forms
---

# Check Form Responses

> **PREREQUISITE:** Load the following skills to execute this recipe: `xgc-forms`

Retrieve and review responses from a Google Form.

## Steps

1. List forms: `xgc forms forms list` (if you don't have the form ID)
2. Get form details: `xgc forms forms get --params '{"formId": "FORM_ID"}'`
3. Get responses: `xgc forms forms responses list --params '{"formId": "FORM_ID"}' --format table`

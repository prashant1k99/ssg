### Layout:

For templates

- for content/{contnet-type}/{title}.md

---

- Template: theme/{theme-name}/{contnet-type}.html -> Is used for rendering the content -> it should generate the slug = https://example.com/{contnet-type}/{title}
  NOTE: This should not create the listing page for content-type

--- OR ---

- Template: theme/{theme-name}/{contnet-type}/index.html -> Used for listing of the page -> It should generate the slug = https://example.com/{contnet-type}
- Template: theme/{theme-name}/{contnet-type}/template.html -> Used for specific content rendering, it should generate the slug = https://example.com/{contnet-type}/{title}

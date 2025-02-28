# Theme Building Guide

This guide will help you create custom themes for the Static Site Generator (SSG) project. By following this guide, you can design and implement your own themes to give your static site a unique look and feel.

## Table of Contents

1. [Introduction](#introduction)
2. [Theme Structure](#theme-structure)
3. [Required Files](#required-files)
4. [Creating a Custom Theme](#creating-a-custom-theme)
5. [Using the Custom Theme](#using-the-custom-theme)
6. [Examples](#examples)
7. [Best Practices](#best-practices)
8. [Troubleshooting](#troubleshooting)
9. [Conclusion](#conclusion)
10. [Additional Resources](#additional-resources)
11. [Contact Information](#contact-information)

## Introduction

This documentation provides a comprehensive guide for users to create and use custom themes in the SSG project. It includes the necessary steps, required files, and examples to help users get started with theme building.

## Theme Structure

A theme in the SSG project is a directory that contains HTML templates and other assets (such as CSS, JavaScript, and images). The theme directory should be placed inside the `theme` directory of the project.

Here is an example structure of a theme named `my-theme`:

```
ssg/
├── theme/
│   └── my-theme/
│       ├── index.html
│       ├── template.html
│       ├── styles.css
│       ├── script.js
│       └── images/
│           └── logo.png
```

## Required Files

To create a functional theme, you need to include the following required files in your theme directory:

1. **index.html**: The main template file for the home page of the site.
2. **template.html**: The template file used for rendering Markdown content.

### index.html

This file is used as the main template for the home page of your site. It should include placeholders for dynamic content that will be populated during the build process.

Example `index.html`:

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>{{ settings.site_title }}</title>
    <link rel="stylesheet" href="styles.css" />
  </head>
  <body>
    <header>
      <h1>{{ settings.site_title }}</h1>
      <nav>
        <ul>
          <li><a href="/">Home</a></li>
          <li><a href="/about.html">About</a></li>
          <li><a href="/blog/index.html">Blog</a></li>
        </ul>
      </nav>
    </header>
    <main>
      <h2>Welcome to {{ settings.site_title }}</h2>
      <p>{{ settings.site_description }}</p>
    </main>
    <footer>
      <p>&copy; {{ settings.site_title }} {{ settings.current_year }}</p>
    </footer>
    <script src="script.js"></script>
  </body>
</html>
```

### template.html

This file is used to render Markdown content. It should include placeholders for the content and other dynamic data.

Example `template.html`:

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>{{ settings.site_title }} - {{ title }}</title>
    <link rel="stylesheet" href="../styles.css" />
  </head>
  <body>
    <header>
      <h1><a href="/">{{ settings.site_title }}</a></h1>
    </header>
    <main>
      <article>
        <h2>{{ title }}</h2>
        <div>{{ content | safe }}</div>
      </article>
    </main>
    <footer>
      <p>&copy; {{ settings.site_title }} {{ settings.current_year }}</p>
    </footer>
    <script src="../script.js"></script>
  </body>
</html>
```

## Creating a Custom Theme

Follow these steps to create your custom theme:

1. **Create the Theme Directory**:

   - Inside the `theme` directory of the project, create a new directory with your theme name (e.g., `my-theme`).

2. **Add Required Files**:

   - Create the `index.html` and `template.html` files inside your theme directory. Use the examples provided above as a starting point.

3. **Add Assets**:
   - Add any additional assets your theme requires, such as CSS files, JavaScript files, and images. Place them inside your theme directory.

## Using the Custom Theme

To use your custom theme, update the `config.toml` file in the root directory of the project with the name of your theme.

Example `config.toml`:

```toml
[settings]
site_title = "My Static Site"
site_description = "A static site built with SSG"
theme = "my-theme"
out_dir = "dist"
asset_dir = "static"
current_year = 2025

[custom]
# Add any custom settings here
```

## Examples

Here are a few examples of how you can customize your theme:

### Changing the Layout

You can modify the layout of your `index.html` and `template.html` files to change the structure of your site. For example, you can add a sidebar, change the header, or update the footer.

### Adding Custom Styles

Create a `styles.css` file inside your theme directory and link it in your `index.html` and `template.html` files. You can add custom styles to give your site a unique look.

Example `styles.css`:

```css
body {
  font-family: Arial, sans-serif;
}

header {
  background-color: #333;
  color: #fff;
  padding: 1rem;
}

nav ul {
  list-style-type: none;
}

nav ul li {
  display: inline;
  margin-right: 1rem;
}

footer {
  background-color: #333;
  color: #fff;
  text-align: center;
  padding: 1rem;
  position: fixed;
  bottom: 0;
  width: 100%;
}
```

### Adding Custom Scripts

Create a `script.js` file inside your theme directory and link it in your `index.html` and `template.html` files. You can add custom JavaScript to add interactivity to your site.

Example `script.js`:

```js
document.addEventListener("DOMContentLoaded", function () {
  console.log("Custom theme script loaded");
});
```

## Best Practices

- **Consistency**: Ensure that your theme's design and layout are consistent across all pages.
- **Accessibility**: Follow web accessibility guidelines to make your site usable for all users.
- **Performance**: Optimize your assets (e.g., compress images, minify CSS and JavaScript) to improve site performance.
- **Maintainability**: Keep your code clean and well-documented to make it easy to maintain and update your theme.

## Troubleshooting

### Common Issues

1. **Theme Not Found**:

   - Ensure that the theme directory is correctly named and placed inside the `theme` directory.
   - Verify that the `config.toml` file has the correct theme name.

2. **Missing Assets**:

   - Check that all required assets (e.g., CSS, JavaScript, images) are included in your theme directory.
   - Ensure that the paths to the assets are correct in your HTML files.

3. **Rendering Issues**:
   - Verify that your HTML templates have the correct placeholders for dynamic content.
   - Check for any syntax errors in your HTML, CSS, or JavaScript files.

## Conclusion

By following this guide, you can create and customize your own themes for the SSG project. Experiment with different layouts, styles, and scripts to create a unique static site that meets your needs.

If you have any questions or need further assistance, feel free to open an issue in the [SSG GitHub repository](https://github.com/prashant1k99/ssg).

## Additional Resources

- [SSG GitHub Repository](https://github.com/prashant1k99/ssg)
- [Markdown Guide](https://www.markdownguide.org/)
- [Web Accessibility Guidelines](https://www.w3.org/WAI/WCAG21/quickref/)

## Contact Information

For further assistance, you can reach out to the project maintainer:

- **Name**: Prashant Singh
- **GitHub**: [prashant1k99](https://github.com/prashant1k99)

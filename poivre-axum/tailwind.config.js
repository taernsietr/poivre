/** @type {import('tailwindcss').Config} */
    module.exports = {
      content: {
        relative: true,
        files: [
          "*.html",
          "./src/**/*.rs",
          "./poivre-axum/src/**/*.rs"
        ],
      },
      theme: {
        extend: {},
      },
      plugins: [],
    }
    

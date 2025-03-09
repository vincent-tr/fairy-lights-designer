const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  plugins: [
    new CopyWebpackPlugin({ patterns: ['index.html', 'style.css', 'favicon.ico'] })
  ],
  experiments: {
    asyncWebAssembly: true,
    syncWebAssembly: true
  },
  devServer: {
    host: '0.0.0.0',
    port: 12345,
    allowedHosts: 'all',
    proxy: [
      {
        context: ['/api'],
        target: 'http://localhost:3000',
      },
    ],
  },
};

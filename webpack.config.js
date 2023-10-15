const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const { ProvidePlugin } = require('webpack');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = ({
  entry: `./web`,
  output: {
    path: path.resolve(__dirname, `dist`),
    filename: 'index.js',
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: `./web/index.html`,
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, `.`),
      extraArgs: '--target web'
    }),
    new ProvidePlugin({
      TextDecoder: ['text-encoding', 'TextDecoder'],
      TextEncoder: ['text-encoding', 'TextEncoder'],
    }),
  ],
  mode: 'development',
  resolve: {
    alias: {
      pkg: path.resolve(__dirname, 'pkg'),
    },
  },
  experiments: {
    asyncWebAssembly: true,
  },
});

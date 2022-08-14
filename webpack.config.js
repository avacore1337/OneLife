const path = require('path')
const CopyPlugin = require('copy-webpack-plugin')
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')
const VueLoaderPlugin = require('vue-loader/lib/plugin')

const dist = path.resolve(__dirname, 'dist')

module.exports = {
  mode: 'production',
  resolve: {
    alias: {
      vue$: 'vue/dist/vue.js',
    },
  },
  module: {
    rules: [
      { test: /\.js$/, use: 'babel-loader' },
      { test: /\.vue$/, use: 'vue-loader' },
      { test: /\.css$/, use: ['vue-style-loader', 'css-loader'] },
    ],
  },
  entry: {
    HackTimer: './js/HackTimer.js',
    index: './js/index.js',
  },
  output: {
    path: dist,
    filename: '[name].js',
  },
  devServer: {
    contentBase: dist,
  },
  plugins: [
    new CopyPlugin([path.resolve(__dirname, 'static')]),

    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),

    new VueLoaderPlugin(),
  ],
}

const webpack = require('webpack');
const path = require('path');

module.exports = {
  devtool: 'eval-source-map',
  entry: ['whatwg-fetch', './index.web.js'],
  output: {
    path: path.resolve(__dirname, './web'),
    filename: 'bundle.js',
  },
  module: {
    loaders: [
      {
        test: /\.js?$/,
        //exclude: /node_modules/,
        loader: 'babel-loader',
        query: {
          presets: ['react-native', 'es2015', 'react', 'stage-0'],
        },
      },
    ],
  },
  resolve: {
    alias: {
      'react-native': 'react-native-web',
      'react-native-svg': 'svgs',
    },
  },
};

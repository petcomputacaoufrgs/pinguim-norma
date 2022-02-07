const CopyPlugin = require('copy-webpack-plugin');
const path = require('path');

module.exports = {
  entry: {
    './index': './index.js',
    './execution': './execution.js',
  },
  experiments: {
    asyncWebAssembly: true,
  },
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: '[name].js',
  },
  mode: 'development',
  plugins: [
    new CopyPlugin({
      patterns: [
        { from: '*.html', to: './' }
      ]
    }),
  ],
  module: {
    rules: [
      {
        test: /\.css$/i,
        use: ['style-loader', 'css-loader'],
      }
    ]
  }
};

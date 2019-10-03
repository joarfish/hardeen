const path = require('path');
const production = process.env.NODE_ENV === 'production';
const TerserPlugin = require('terser-webpack-plugin');
const BundleAnalyzerPlugin = require('webpack-bundle-analyzer').BundleAnalyzerPlugin;

module.exports = {
	mode: production ? 'production' : 'development',
	devtool: 'inline-source-map',
	entry: './src/main.tsx',
	output: {
		path: path.join(__dirname, 'dist'),
		filename: '[name].bundle.js',
		chunkFilename: "[name].chunk.js",
	},
	resolve: {
		extensions: ['.ts', '.tsx', '.js', '.jsx']
	},
	/*optimization: {
		minimizer: [
			new TerserPlugin({
				parallel: true,
				terserOptions: {
					ecma: 6
				}
			})
		]
	},*/
	module: {
		rules: [
			{
				test: /\.css$/,
				use: ['style-loader', 'css-loader']
			},
			{
				test: /\.tsx?$/,
				loader: 'ts-loader',
				options: {
					transpileOnly: true
				}
			}
		]
	},
	devServer: {
		host: 'localhost',
		compress: true,
		port: 9000,
		disableHostCheck: true,
		overlay: true
	},
	plugins: [
	//	new BundleAnalyzerPlugin()
	]
};
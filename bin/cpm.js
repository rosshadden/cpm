const fs = require('fs');
const path = require('path');
const yargs = require('yargs');
const toml = require('toml');

let cwd = path.resolve('.');
let cosmicPath = path.resolve(cwd, 'cosmic.toml');

yargs
	.usage('$0 <cmd> [args]')
	.command('run <action>', 'Run an action defined in the package file.', {}, () => {
		fs.readFile(cosmicPath, 'utf-8', (err, data) => {
			console.log(data);
		});
	})
	.help('help')
	.argv;

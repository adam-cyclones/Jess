"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const lib_jess_1 = require("lib-jess");
const commander_1 = require("commander");
const log_1 = __importDefault(require("./log"));
const util_1 = require("./util");
const path_1 = require("path");
;
(async () => {
    const program = new commander_1.Command();
    program.version('0.0.1');
    program.command('compile <source> [destination]')
        .description('compile a .jss or .tss source file.')
        .action((source, destination) => {
        lib_jess_1.compiler.then(async ({ compileFile }) => {
            try {
                const ext = path_1.extname(source);
                // cause for alarm
                if (!(ext === lib_jess_1.JESS_EXTNAME || ext === lib_jess_1.TESS_EXTNAME)) {
                    throw `Unsupported file format '${ext}', Jess expected a ${lib_jess_1.JESS_EXTNAME} or ${lib_jess_1.TESS_EXTNAME} extension!`;
                }
                console.log(await compileFile(path_1.resolve(process.cwd(), source)));
            }
            catch (err) {
                throw err;
            }
        })
            .catch(err => {
            console.log(err);
            if (err.code) {
                switch (err.code) {
                    case "ENOENT":
                        log_1.default.error(`${util_1.isRelativePath(source) ? `'${path_1.resolve(process.cwd(), source)}'` : `'${source}'`} is not found! ${err.code}`);
                        break;
                }
            }
            else {
                log_1.default.error(err.stack ? `Panic! ${err.stack}` : err);
            }
        });
    });
    program.parse(process.argv);
})();

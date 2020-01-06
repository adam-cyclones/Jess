import {
  compiler,
  JESS_EXTNAME,
  TESS_EXTNAME
} from 'lib-jess';
import { Command } from 'commander';
import log from './log';
import { isRelativePath } from './util';
import { extname, resolve } from 'path';

;(async () => {
  const program = new Command();

  program.version('0.0.1');
  program.command('compile <source> [destination]')
    .description('compile a .jss or .tss source file.')
    .action((source, destination) => {
      compiler.then(async ({ compileFile }) => {
        try {
          const ext = extname(source);
          // cause for alarm
          if (!(ext === JESS_EXTNAME || ext === TESS_EXTNAME)) {
            throw `Unsupported file format '${ext}', Jess expected a ${JESS_EXTNAME} or ${TESS_EXTNAME} extension!`;
          }
          console.log(await compileFile(resolve(process.cwd(), source)));
        } catch(err) {
          throw err;
        }
      })
      .catch(err => {
        console.log(err)
        if (err.code) {
          switch (err.code) {
            case "ENOENT":
              log.error(`${isRelativePath(source) ? `'${resolve(process.cwd(), source)}'` : `'${source}'`} is not found! ${err.code}`);
            break;
          }
        } else {
          log.error(err.stack ? `Panic! ${err.stack}` : err);
        }
      });

    });

  program.parse(process.argv)
})();

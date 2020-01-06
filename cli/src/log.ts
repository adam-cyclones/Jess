import winston from 'winston';
import { capitalize, getLangCode } from "./util";
import { Format } from 'logform';

winston.format.combine(...[])

const customFormat = (color: boolean, time: boolean) => winston.format.combine(
  ...[
    (color ? winston.format.colorize() : null) as Format,
    winston.format.align(),
    winston.format.printf((info) => {
      const {
        level,
        message,
        ...args
      } = info;

      const ts = new Intl.DateTimeFormat(getLangCode(), {
        day: '2-digit',
        month: '2-digit',
        year: 'numeric',
        second: 'numeric',
        minute: 'numeric',
        hour: 'numeric'
      })
      .format(new Date(Date.now()));

      return `${time ? `${ts} ` : ' '}${capitalize(level)}: ${message.trim()} ${Object.keys(args).length ? JSON.stringify(args, null, 2) : ''}`;
    }),
  ].filter(Boolean),
);

export default winston.createLogger({
  transports: [
    new winston.transports.Console({
      format: customFormat(true, false)
    }),
    new winston.transports.File({
      format: winston.format.combine(
          customFormat(false, true),
          winston.format.uncolorize()
        ),
      dirname: process.cwd(),
      filename: 'jess-debug.log'
    })
  ]
})
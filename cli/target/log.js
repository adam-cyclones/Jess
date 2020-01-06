"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const winston_1 = __importDefault(require("winston"));
const util_1 = require("./util");
winston_1.default.format.combine(...[]);
const customFormat = (color, time) => winston_1.default.format.combine(...[
    (color ? winston_1.default.format.colorize() : null),
    winston_1.default.format.align(),
    winston_1.default.format.printf((info) => {
        const { level, message, ...args } = info;
        const ts = new Intl.DateTimeFormat(util_1.getLangCode(), {
            day: '2-digit',
            month: '2-digit',
            year: 'numeric',
            second: 'numeric',
            minute: 'numeric',
            hour: 'numeric'
        })
            .format(new Date(Date.now()));
        return `${time ? `${ts} ` : ' '}${util_1.capitalize(level)}: ${message.trim()} ${Object.keys(args).length ? JSON.stringify(args, null, 2) : ''}`;
    }),
].filter(Boolean));
exports.default = winston_1.default.createLogger({
    transports: [
        new winston_1.default.transports.Console({
            format: customFormat(true, false)
        }),
        new winston_1.default.transports.File({
            format: winston_1.default.format.combine(customFormat(false, true), winston_1.default.format.uncolorize()),
            dirname: process.cwd(),
            filename: 'jess-debug.log'
        })
    ]
});

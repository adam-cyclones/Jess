import { Stack, iStack } from './stack';

const L_C_BRACE = '{';
const R_C_BRACE = '}';



const CURLY_BRACES: RegExp = new RegExp(`${L_C_BRACE}|${R_C_BRACE}`, 'g');
type tCBraceToken = '{' | '}';

interface iBrace {
  token: tCBraceToken;
  index: number;
  raw: string;
  depth: number;
  incr: () => void;
  decr: () => void;
  onToken(token: tCBraceToken , callback: (callback: iBraceOnTokenDetail) => void): void;
  captureUntil(token: tCBraceToken): void;
  stopCapture(callback: (detail: iBraceStopCaptureCallbackDetail) => void): void;
  resetCapturedContent(): void;
}

interface iBraceCaptureLine {
  raw: string,
  depth: number
}

interface iBraceStopCaptureCallbackDetail {
  capturedStack: iBraceCaptureLine[];
  done: boolean
}

interface iBraceOnTokenDetail {
  raw: string,
  depth: number,
}


const curlyBraceStack: iStack = Stack.create(Infinity);

const curlyBraceParserState = {
  captureUntil: '' as string,
  capturedStack: [] as iBraceCaptureLine[],
  captureRunning: false
};

function parseCurlyBraces(line: string): iBrace[] {
  const lineMatcher: any = line;
  const lineMatches = [...lineMatcher.matchAll(CURLY_BRACES)];

  const brace: iBrace[] = lineMatches.map( function (m) {
    const result = <iBrace>{
      token: m[0],
      index: m.index,
      raw: m.input,
      depth: 0,
      onToken(token: tCBraceToken, callback: () => void) {
        const state = curlyBraceParserState;
        if (state.captureUntil) {
          state.capturedStack.push({
            raw: m.input,
            depth: 0,
          })
        }
        if (m[0] === token) {
          callback();
        }
      },
      incr() {
        this.depth ++;
      },
      decr() {
        this.depth --;
      },
      captureUntil(token: tCBraceToken): void {
        const state = curlyBraceParserState;
        if (!state.captureUntil) {
          state.captureRunning = true;
          state.captureUntil = token;
        }
      },
      stopCapture(callback): void {
        const state = curlyBraceParserState;
        if (m[0] === state.captureUntil) {
          result.resetCapturedContent();
        }
        callback({
          capturedStack: state.capturedStack,
          done: !state.captureRunning
        });
      },
      resetCapturedContent() {
        const state = curlyBraceParserState;
        state.captureUntil = '';
        state.capturedStack.length = 0;
        state.captureRunning = false;
        console.log(state)
      }
    }
    return result;
  });

  return brace;
}

export {
  parseCurlyBraces,
  curlyBraceStack
}
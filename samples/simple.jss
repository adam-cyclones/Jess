@import { aBlock, anotherBlock } from "./myImport";

@script {
  const bar = #333 solid red;
}

.foo {
  border: {bar};
}
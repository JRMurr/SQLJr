import test from "ava";

import { test as testFn } from "../index.js";

test("basic test", (t) => {
  t.deepEqual(testFn(), [
    ["1", "aString"],
    ["4", "aDiffString with spaces"],
  ]);
});

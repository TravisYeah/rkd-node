import test from "ava";
import path from "path";
import fs from "fs";
import { createDeltaFile, createTargetFile } from "../index.js";

const createPath = (p) => {
  console.log(process.cwd());
  return path.join(process.cwd(), "__test__", p);
};

test("sum from native", (t) => {
  const source = createPath("rk-wiki.txt");
  const target = createPath("rk-wiki-insert-p.txt");
  const delta = createPath("delta");
  const recreatedTarget = createPath("rk-wiki-insert-p-recreated.txt");
  createDeltaFile(source, target, delta, 4, Math.pow(10, 9) + 9);
  createTargetFile(source, recreatedTarget, delta);
  const targetFile = fs.readFileSync(target);
  const targetFileRecreated = fs.readFileSync(recreatedTarget);
  t.deepEqual(targetFile, targetFileRecreated);
});

import test from 'ava';

import { test as testFn, Execution } from '../index.js';

test('basic test', (t) => {
    t.deepEqual(testFn(), [
        ['1', 'aString'],
        ['4', 'aDiffString with spaces'],
    ]);
});

test('exec struct', (t) => {
    const exec = new Execution();

    t.deepEqual(
        exec.query(`
        CREATE TABLE foo (
          col1 int,
          col2 string
        );
      `),
        []
    );

    t.deepEqual(
        exec.query(`
          INSERT INTO foo
          VALUES
              1, 'aString';
        `),
        []
    );

    t.deepEqual(
        exec.query(`
          INSERT INTO foo
          VALUES
              4, 'aDiffString with spaces';
        `),
        []
    );

    t.deepEqual(
        exec.query(`
          SELECT
            col1,
            col2
          FROM
              foo;
        `),
        [
            {
                col1: '1',
                col2: 'aString',
            },
            {
                col1: '4',
                col2: 'aDiffString with spaces',
            },
        ]
    );
});

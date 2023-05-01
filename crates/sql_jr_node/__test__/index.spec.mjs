import test from 'ava';

import { basicQuery, Execution } from '../index.js';

test('basic test', (t) => {
    t.deepEqual(basicQuery(), [
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

test('parse error', (t) => {
    const exec = new Execution();

    t.throws(() => exec.query(`sad`), { code: 'GenericFailure', message: 'Parse Error' });
});

test('async function', async (t) => {
    const exec = new Execution();

    const get_query = async () => {
        return `
            CREATE TABLE foo (
                col1 int,
                col2 string
            );
        `;
    };

    t.deepEqual(await exec.queryAsync(get_query()), []);
});

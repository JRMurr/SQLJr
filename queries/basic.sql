CREATE TABLE foo (
    col1 int,
    col2 string
);

INSERT INTO foo
    VALUES
        1, 'aString';

INSERT INTO foo
    VALUES
        4, 'aDiffString with spaces';

SELECT
    col1,
    col2
FROM
    foo;


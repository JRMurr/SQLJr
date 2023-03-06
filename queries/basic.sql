CREATE TABLE foo (
    col1 int,
    col2 string
);

INSERT INTO foo
    VALUES
        1, 2;

INSERT INTO foo
    VALUES
        4, 5;

SELECT
    col1,
    col2
FROM
    foo;


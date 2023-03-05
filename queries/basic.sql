CREATE TABLE foo (
    column1 int,
    column2 string
);

INSERT INTO foo
    VALUES
        1, 2;

INSERT INTO foo
    VALUES
        4, 5;

SELECT
    column1,
    column2
FROM
    foo;


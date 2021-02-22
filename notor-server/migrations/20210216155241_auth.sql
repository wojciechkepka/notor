CREATE TYPE user_role AS ENUM ( 'user', 'admin' );

CREATE TABLE IF NOT EXISTS users
(
    id          INT GENERATED ALWAYS AS IDENTITY,
    created     TIMESTAMP NOT NULL,
    username    VARCHAR(64) NOT NULL,
    email       TEXT NOT NULL,
    pass        VARCHAR(64) NOT NULL,
    role        user_role NOT NULL,

    PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS claims
(
    sub         VARCHAR(64) NOT NULL,
    role        VARCHAR(64) NOT NULL,
    exp         BIGINT      NOT NULL
);
